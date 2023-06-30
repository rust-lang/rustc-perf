#![recursion_limit = "1024"]

use anyhow::Context;
use clap::builder::{PossibleValue, TypedValueParser};
use clap::{Arg, Parser, ValueEnum};
use collector::api::next_artifact::NextArtifact;
use collector::compile::benchmark::category::Category;
use collector::compile::benchmark::profile::Profile;
use collector::compile::benchmark::scenario::Scenario;
use collector::compile::benchmark::{
    compile_benchmark_dir, get_compile_benchmarks, Benchmark, BenchmarkName,
};
use collector::{runtime, utils, CollectorCtx, CollectorStepBuilder};
use database::{ArtifactId, Commit, CommitType, Connection, Pool};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};
use std::{str, time::Instant};
use tokio::runtime::Runtime;

use collector::compile::execute::bencher::BenchProcessor;
use collector::compile::execute::profiler::{ProfileProcessor, Profiler};
use collector::runtime::{
    bench_runtime, runtime_benchmark_dir, BenchmarkFilter, CargoIsolationMode,
};
use collector::toolchain::{get_local_toolchain, Sysroot, Toolchain};

fn n_normal_benchmarks_remaining(n: usize) -> String {
    let suffix = if n == 1 { "" } else { "s" };
    format!("{} normal benchmark{} remaining", n, suffix)
}

struct BenchmarkErrors(usize);

impl BenchmarkErrors {
    fn new() -> BenchmarkErrors {
        BenchmarkErrors(0)
    }

    fn incr(&mut self) {
        self.0 += 1;
    }

    fn add(&mut self, count: usize) {
        self.0 += count;
    }

    fn fail_if_nonzero(self) -> anyhow::Result<()> {
        if self.0 > 0 {
            anyhow::bail!("{} benchmarks failed", self.0)
        }
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn bench(
    rt: &mut Runtime,
    mut conn: Box<dyn Connection>,
    profiles: &[Profile],
    scenarios: &[Scenario],
    toolchain: &Toolchain,
    benchmarks: &[Benchmark],
    iterations: Option<usize>,
    is_self_profile: bool,
    mut collector: CollectorCtx,
) -> BenchmarkErrors {
    let mut errors = BenchmarkErrors::new();
    eprintln!(
        "Benchmarking {} for triple {}",
        collector.artifact_id, toolchain.triple
    );

    if is_self_profile {
        if let Err(e) = check_measureme_installed() {
            panic!("{}Or omit --self-profile` to opt out\n", e);
        }
    }

    let bench_rustc = collector.bench_rustc;

    let start = Instant::now();
    let mut skipped = false;

    let mut measure_and_record =
        |benchmark_name: &BenchmarkName,
         category: Category,
         print_intro: &dyn Fn(),
         measure: &dyn Fn(&mut BenchProcessor) -> anyhow::Result<()>| {
            let is_fresh = rt.block_on(collector.start_compile_step(conn.as_mut(), benchmark_name));
            if !is_fresh {
                skipped = true;
                eprintln!("skipping {} -- already benchmarked", benchmark_name);
                return;
            }
            let mut tx = rt.block_on(conn.transaction());
            let (supports_stable, category) = category.db_representation();
            rt.block_on(tx.conn().record_compile_benchmark(
                &benchmark_name.0,
                Some(supports_stable),
                category,
            ));
            print_intro();

            let mut processor = BenchProcessor::new(
                rt,
                tx.conn(),
                benchmark_name,
                &collector.artifact_id,
                collector.artifact_row_id,
                is_self_profile,
            );
            let result = measure(&mut processor);
            if let Err(s) = result {
                eprintln!(
                    "collector error: Failed to benchmark '{}', recorded: {:#}",
                    benchmark_name, s
                );
                errors.incr();
                rt.block_on(tx.conn().record_error(
                    collector.artifact_row_id,
                    &benchmark_name.0,
                    &format!("{:?}", s),
                ));
            };
            rt.block_on(collector.end_compile_step(tx.conn(), benchmark_name));
            rt.block_on(tx.commit()).expect("committed");
        };

    // Normal benchmarks.
    for (nth_benchmark, benchmark) in benchmarks.iter().enumerate() {
        measure_and_record(
            &benchmark.name,
            benchmark.category(),
            &|| {
                eprintln!(
                    "{}",
                    n_normal_benchmarks_remaining(benchmarks.len() - nth_benchmark)
                )
            },
            &|processor| benchmark.measure(processor, profiles, scenarios, toolchain, iterations),
        )
    }

    // The special rustc benchmark, if requested.
    if bench_rustc {
        measure_and_record(
            &BenchmarkName("rustc".to_string()),
            Category::Primary,
            &|| eprintln!("Special benchmark commencing (due to `--bench-rustc`)"),
            &|processor| processor.measure_rustc(toolchain).context("measure rustc"),
        );
    }

    let end = start.elapsed();

    eprintln!(
        "collection took {:?} with {} failed benchmarks",
        end, errors.0
    );

    if skipped {
        log::info!("skipping duration record -- skipped parts of run");
    } else {
        rt.block_on(conn.record_duration(collector.artifact_row_id, end));
    }

    rt.block_on(async move {
        // This ensures that we're good to go with the just updated data.
        conn.maybe_create_indices().await;
    });
    errors
}

fn check_measureme_installed() -> Result<(), String> {
    let not_installed = IntoIterator::into_iter(["summarize", "crox", "flamegraph"])
        .filter(|n| !is_installed(n))
        .collect::<Vec<_>>();
    if not_installed.is_empty() {
        Ok(())
    } else {
        Err(format!("To run this command you need {0} on your PATH. To install run `cargo install --git https://github.com/rust-lang/measureme --branch stable {0}`\n", not_installed.join(" ")))
    }
}

fn check_installed(name: &str) -> anyhow::Result<()> {
    if !is_installed(name) {
        anyhow::bail!("`{}` is not installed but must be", name);
    }
    Ok(())
}

fn is_installed(name: &str) -> bool {
    Command::new(name).output().is_ok()
}

fn generate_cachegrind_diffs(
    id1: &str,
    id2: &str,
    out_dir: &Path,
    benchmarks: &[Benchmark],
    profiles: &[Profile],
    scenarios: &[Scenario],
    errors: &mut BenchmarkErrors,
) -> Vec<PathBuf> {
    let mut annotated_diffs = Vec::new();
    for benchmark in benchmarks {
        for &profile in profiles {
            for scenario in scenarios.iter().flat_map(|scenario| {
                if profile == Profile::Doc && scenario.is_incr() {
                    return vec![];
                }
                match scenario {
                    Scenario::Full | Scenario::IncrFull | Scenario::IncrUnchanged => {
                        vec![format!("{:?}", scenario)]
                    }
                    Scenario::IncrPatched => (0..benchmark.patches.len())
                        .map(|i| format!("{:?}{}", scenario, i))
                        .collect::<Vec<_>>(),
                }
            }) {
                let filename = |prefix, id| {
                    format!(
                        "{}-{}-{}-{:?}-{}",
                        prefix, id, benchmark.name, profile, scenario
                    )
                };
                let id_diff = format!("{}-{}", id1, id2);
                let cgout1 = out_dir.join(filename("cgout", id1));
                let cgout2 = out_dir.join(filename("cgout", id2));
                let cgfilt1 = out_dir.join(filename("cgfilt", id1));
                let cgfilt2 = out_dir.join(filename("cgfilt", id2));
                let cgfilt_diff = out_dir.join(filename("cgfilt-diff", &id_diff));
                let cgann_diff = out_dir.join(filename("cgann-diff", &id_diff));

                if let Err(e) = rustfilt(&cgout1, &cgfilt1) {
                    errors.incr();
                    eprintln!("collector error: {:?}", e);
                    continue;
                }
                if let Err(e) = rustfilt(&cgout2, &cgfilt2) {
                    errors.incr();
                    eprintln!("collector error: {:?}", e);
                    continue;
                }
                if let Err(e) = cg_diff(&cgfilt1, &cgfilt2, &cgfilt_diff) {
                    errors.incr();
                    eprintln!("collector error: {:?}", e);
                    continue;
                }
                if let Err(e) = cg_annotate(&cgfilt_diff, &cgann_diff) {
                    errors.incr();
                    eprintln!("collector error: {:?}", e);
                    continue;
                }

                annotated_diffs.push(cgann_diff);
            }
        }
    }
    annotated_diffs
}

/// Demangles symbols in a file using rustfilt and writes result to path.
fn rustfilt(cgout: &Path, path: &Path) -> anyhow::Result<()> {
    if !is_installed("rustfilt") {
        anyhow::bail!("`rustfilt` not installed.");
    }
    let output = Command::new("rustfilt")
        .arg("-i")
        .arg(cgout)
        .stderr(Stdio::inherit())
        .output()
        .context("failed to run `rustfilt`")?;

    if !output.status.success() {
        anyhow::bail!("failed to process output with rustfilt");
    }

    fs::write(path, output.stdout).context("failed to write `rustfilt` output")?;

    Ok(())
}

/// Compares two Cachegrind output files using cg_diff and writes result to path.
fn cg_diff(cgout1: &Path, cgout2: &Path, path: &Path) -> anyhow::Result<()> {
    if !is_installed("cg_diff") {
        anyhow::bail!("`cg_diff` not installed.");
    }
    let output = Command::new("cg_diff")
        .arg(r"--mod-filename=s/\/rustc\/[^\/]*\///")
        .arg("--mod-funcname=s/[.]llvm[.].*//")
        .arg(cgout1)
        .arg(cgout2)
        .stderr(Stdio::inherit())
        .output()
        .context("failed to run `cg_diff`")?;

    if !output.status.success() {
        anyhow::bail!("failed to generate cachegrind diff");
    }

    fs::write(path, output.stdout).context("failed to write `cg_diff` output")?;

    Ok(())
}

/// Post process Cachegrind output file and writes resutl to path.
fn cg_annotate(cgout: &Path, path: &Path) -> anyhow::Result<()> {
    let output = Command::new("cg_annotate")
        .arg("--show-percs=no")
        .arg(cgout)
        .stderr(Stdio::inherit())
        .output()
        .context("failed to run `cg_annotate`")?;

    if !output.status.success() {
        anyhow::bail!("failed to annotate cachegrind output");
    }

    fs::write(path, output.stdout).context("failed to write `cg_annotate` output")?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn profile(
    toolchain: &Toolchain,
    profiler: Profiler,
    out_dir: &Path,
    benchmarks: &[Benchmark],
    profiles: &[Profile],
    scenarios: &[Scenario],
    errors: &mut BenchmarkErrors,
) {
    eprintln!("Profiling {} with {:?}", toolchain.id, profiler);
    if let Profiler::SelfProfile = profiler {
        check_measureme_installed().unwrap();
    }

    let error_count: usize = benchmarks
        .par_iter()
        .enumerate()
        .map(|(i, benchmark)| {
            let benchmark_id = format!("{} ({}/{})", benchmark.name, i + 1, benchmarks.len());
            eprintln!("Executing benchmark {benchmark_id}");
            let mut processor = ProfileProcessor::new(profiler, out_dir, &toolchain.id);
            let result = benchmark.measure(&mut processor, profiles, scenarios, toolchain, Some(1));
            eprintln!("Finished benchmark {benchmark_id}");

            if let Err(ref s) = result {
                eprintln!(
                    "collector error: Failed to profile '{}' with {:?}, recorded: {:?}",
                    benchmark.name, profiler, s
                );
                1
            } else {
                0
            }
        })
        .sum();
    errors.add(error_count);
}

fn main() {
    match main_result() {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("collector error: {:?}", err);
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone)]
struct ProfileArg(Vec<Profile>);

#[derive(Clone)]
struct ProfileArgParser;

/// We need to use a TypedValueParser to provide possible values help.
/// If we just use `FromStr` + `#[arg(possible_values = [...])]`, `clap` will not allow passing
/// multiple values.
impl TypedValueParser for ProfileArgParser {
    type Value = ProfileArg;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if value == "All" {
            Ok(ProfileArg(Profile::all()))
        } else {
            let profiles: Result<Vec<Profile>, _> = value
                .to_str()
                .unwrap()
                .split(',')
                .map(|item| clap::value_parser!(Profile).parse_ref(cmd, arg, OsStr::new(item)))
                .collect();

            Ok(ProfileArg(profiles?))
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        let values = Profile::value_variants()
            .iter()
            .filter_map(|item| item.to_possible_value())
            .chain([PossibleValue::new("All")]);
        Some(Box::new(values))
    }
}

#[derive(Debug, Clone)]
struct ScenarioArg(Vec<Scenario>);

#[derive(Clone)]
struct ScenarioArgParser;

impl TypedValueParser for ScenarioArgParser {
    type Value = ScenarioArg;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if value == "All" {
            Ok(ScenarioArg(Scenario::all()))
        } else {
            let scenarios: Result<Vec<Scenario>, _> = value
                .to_str()
                .unwrap()
                .split(',')
                .map(|item| clap::value_parser!(Scenario).parse_ref(cmd, arg, OsStr::new(item)))
                .collect();

            Ok(ScenarioArg(scenarios?))
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        let values = Scenario::value_variants()
            .iter()
            .filter_map(|item| item.to_possible_value())
            .chain([PossibleValue::new("All")]);
        Some(Box::new(values))
    }
}

#[derive(Debug, clap::Parser)]
#[command(about, version, author)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[test]
fn verify_cli() {
    // By default, clap lazily checks subcommands. This provides eager testing
    // without having to run the binary for each subcommand.
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

#[derive(Debug, clap::Args)]
struct LocalOptions {
    /// The path to the local rustc to measure
    // Not a `PathBuf` because it can be a file path *or* a `+`-prefixed
    // toolchain name, and `PathBuf` doesn't work well for the latter.
    rustc: String,

    /// Identifier to associate benchmark results with
    #[arg(long)]
    id: Option<String>,

    /// The path to the local Cargo to use
    #[arg(long)]
    cargo: Option<PathBuf>,

    /// Exclude all benchmarks matching a prefix in this comma-separated list
    #[arg(long)]
    exclude: Option<String>,

    /// Exclude all benchmarks matching a suffix in this comma-separated list
    #[arg(long)]
    exclude_suffix: Option<String>,

    /// Include only benchmarks matching a prefix in this comma-separated list
    #[arg(long)]
    include: Option<String>,
}

#[derive(Debug, clap::Args)]
struct CompileTimeOptions {
    /// Measure the build profiles in this comma-separated list
    #[arg(
        long = "profiles",
        alias = "builds", // the old name, for backward compatibility
        value_parser = ProfileArgParser,
        // Don't run rustdoc by default
        default_value = "Check,Debug,Opt",
    )]
    profiles: ProfileArg,

    /// Measure the scenarios in this comma-separated list
    #[arg(
        long = "scenarios",
        alias = "runs", // the old name, for backward compatibility
        value_parser = ScenarioArgParser,
        default_value = "All"
    )]
    scenarios: ScenarioArg,

    /// The path to the local rustdoc to measure
    #[arg(long)]
    rustdoc: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
struct SelfProfileOption {
    /// Collect self-profile data
    #[arg(long = "self-profile")]
    self_profile: bool,
}

#[derive(Debug, clap::Args)]
struct DbOption {
    /// Database output file
    // This would be better as a `PathBuf`, but it's used in various ways that
    // make that tricky without adjusting several points in the code.
    #[arg(long, default_value = "results.db")]
    db: String,
}

#[derive(Debug, clap::Args)]
struct BenchRustcOption {
    /// Run the special `rustc` benchmark
    #[arg(long = "bench-rustc")]
    bench_rustc: bool,
}

// For each subcommand we list the mandatory arguments in the required
// order, followed by the options in alphabetical order.
#[derive(Debug, clap::Subcommand)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// Benchmarks the performance of programs generated by a local rustc
    BenchRuntimeLocal {
        #[command(flatten)]
        local: LocalOptions,

        /// How many iterations of each benchmark should be executed.
        #[arg(long, default_value = "5")]
        iterations: u32,

        #[command(flatten)]
        db: DbOption,

        /// Compile runtime benchmarks directly in their crate directory, to make local experiments
        /// faster.
        #[arg(long = "no-isolate")]
        no_isolate: bool,
    },
    /// Benchmarks a local rustc
    BenchLocal {
        #[command(flatten)]
        local: LocalOptions,

        #[command(flatten)]
        opts: CompileTimeOptions,

        #[command(flatten)]
        db: DbOption,

        #[command(flatten)]
        bench_rustc: BenchRustcOption,

        /// The number of iterations to do for each benchmark
        #[arg(long, default_value = "1")]
        iterations: usize,

        #[command(flatten)]
        self_profile: SelfProfileOption,
    },

    /// Benchmarks the next commit or release for perf.rust-lang.org
    BenchNext {
        /// Site URL
        site_url: String,

        #[command(flatten)]
        db: DbOption,

        #[command(flatten)]
        bench_rustc: BenchRustcOption,

        #[command(flatten)]
        self_profile: SelfProfileOption,
    },

    /// Benchmarks a published toolchain for perf.rust-lang.org's dashboard
    BenchPublished {
        /// Toolchain (e.g. stable, beta, 1.26.0)
        toolchain: String,

        #[command(flatten)]
        db: DbOption,
    },

    /// Profiles a local rustc with one of several profilers
    ProfileLocal {
        /// Profiler to use
        #[arg(value_enum)]
        profiler: Profiler,

        #[command(flatten)]
        local: LocalOptions,

        #[command(flatten)]
        opts: CompileTimeOptions,

        /// Output directory
        #[arg(long = "out-dir", default_value = "results/")]
        out_dir: PathBuf,

        /// The path to the second local rustc (to diff against)
        // Not a `PathBuf` because it can be a file path *or* a `+`-prefixed
        // toolchain name, and `PathBuf` doesn't work well for the latter.
        #[arg(long)]
        rustc2: Option<String>,

        /// How many benchmarks should be profiled in parallel.
        /// This flag is only supported for certain profilers
        #[arg(long, short = 'j', default_value = "1")]
        jobs: u64,
    },

    /// Installs the next commit for perf.rust-lang.org
    InstallNext,

    /// Download a crate into collector/benchmarks.
    Download(DownloadCommand),
}

#[derive(Debug, clap::Parser)]
struct DownloadCommand {
    /// Name of the benchmark created directory
    #[arg(long, global = true)]
    name: Option<String>,

    /// Overwrite the benchmark directory if it already exists
    #[arg(long, short('f'), global = true)]
    force: bool,

    /// What category does the benchmark belong to
    #[arg(long, short('c'), value_enum, global = true, default_value = "primary")]
    category: Category,

    #[command(subcommand)]
    command: DownloadSubcommand,
}

#[derive(Debug, clap::Parser)]
enum DownloadSubcommand {
    /// Download a crate from a git repository.
    Git { url: String },
    /// Download a crate from crates.io.
    Crate {
        #[arg(value_name = "CRATE")]
        krate: String,
        version: String,
    },
}

fn main_result() -> anyhow::Result<i32> {
    env_logger::init();

    let args = Cli::parse();

    let compile_benchmark_dir = compile_benchmark_dir();
    let runtime_benchmark_dir = runtime_benchmark_dir();

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    // We want to minimize noise from the runtime
    builder
        .worker_threads(1)
        .max_blocking_threads(1)
        .enable_io();
    let mut rt = builder.build().expect("built runtime");

    // XXX: This doesn't necessarily work for all archs
    let target_triple = format!("{}-unknown-linux-gnu", std::env::consts::ARCH);

    match args.command {
        Commands::BenchRuntimeLocal {
            local,
            iterations,
            db,
            no_isolate,
        } => {
            let toolchain = get_local_toolchain(
                &[Profile::Opt],
                &local.rustc,
                None,
                local.cargo.as_deref(),
                local.id.as_deref(),
                "",
                target_triple,
            )?;
            let pool = Pool::open(&db.db);

            let isolation_mode = if no_isolate {
                CargoIsolationMode::Cached
            } else {
                CargoIsolationMode::Isolated
            };
            let suite = runtime::create_runtime_benchmark_suite(
                &toolchain,
                &runtime_benchmark_dir,
                isolation_mode,
            )?;
            let artifact_id = ArtifactId::Tag(toolchain.id);
            let (conn, collector) = rt.block_on(async {
                let mut conn = pool.connection().await;
                let collector = CollectorStepBuilder::default()
                    .record_runtime_benchmarks(&suite)
                    .start_collection(conn.as_mut(), artifact_id)
                    .await;
                (conn, collector)
            });
            let fut = bench_runtime(
                conn,
                suite,
                collector,
                BenchmarkFilter::new(local.exclude, local.include),
                iterations,
            );
            rt.block_on(fut)?;
            Ok(0)
        }
        Commands::BenchLocal {
            local,
            opts,
            db,
            bench_rustc,
            iterations,
            self_profile,
        } => {
            let profiles = &opts.profiles.0;
            let scenarios = &opts.scenarios.0;

            let pool = database::Pool::open(&db.db);

            let toolchain = get_local_toolchain(
                profiles,
                &local.rustc,
                opts.rustdoc.as_deref(),
                local.cargo.as_deref(),
                local.id.as_deref(),
                "",
                target_triple,
            )?;

            let mut benchmarks = get_compile_benchmarks(
                &compile_benchmark_dir,
                local.include.as_deref(),
                local.exclude.as_deref(),
                local.exclude_suffix.as_deref(),
            )?;
            benchmarks.retain(|b| b.category().is_primary_or_secondary());

            let artifact_id = ArtifactId::Tag(toolchain.id.clone());
            let (conn, collector) = rt.block_on(init_compile_collector(
                &pool,
                &benchmarks,
                bench_rustc.bench_rustc,
                artifact_id,
            ));

            let res = bench(
                &mut rt,
                conn,
                profiles,
                scenarios,
                &toolchain,
                &benchmarks,
                Some(iterations),
                self_profile.self_profile,
                collector,
            );
            res.fail_if_nonzero()?;
            Ok(0)
        }

        Commands::BenchNext {
            site_url,
            db,
            bench_rustc,
            self_profile,
        } => {
            println!("processing artifacts");
            let client = reqwest::blocking::Client::new();
            let response: collector::api::next_artifact::Response = client
                .get(format!("{}/perf/next_artifact", site_url))
                .send()?
                .json()?;
            let next = if let Some(c) = response.artifact {
                c
            } else {
                println!("no artifact to benchmark");
                // no missing artifacts
                return Ok(0);
            };

            let pool = database::Pool::open(&db.db);

            match next {
                NextArtifact::Release(tag) => {
                    bench_published_artifact(
                        tag,
                        pool,
                        &mut rt,
                        &target_triple,
                        &compile_benchmark_dir,
                    )?;

                    client.post(format!("{}/perf/onpush", site_url)).send()?;
                }
                NextArtifact::Commit {
                    commit,
                    include,
                    exclude,
                    runs,
                } => {
                    let sha = commit.sha.to_string();
                    let sysroot = Sysroot::install(sha.clone(), &target_triple)
                        .with_context(|| format!("failed to install sysroot for {:?}", commit))?;

                    let mut benchmarks = get_compile_benchmarks(
                        &compile_benchmark_dir,
                        include.as_deref(),
                        exclude.as_deref(),
                        None,
                    )?;
                    benchmarks.retain(|b| b.category().is_primary_or_secondary());

                    let artifact_id = ArtifactId::Commit(commit);
                    let (conn, collector) = rt.block_on(init_compile_collector(
                        &pool,
                        &benchmarks,
                        bench_rustc.bench_rustc,
                        artifact_id,
                    ));
                    let res = bench(
                        &mut rt,
                        conn,
                        &Profile::all(),
                        &Scenario::all(),
                        &Toolchain::from_sysroot(&sysroot, sha),
                        &benchmarks,
                        runs.map(|v| v as usize),
                        self_profile.self_profile,
                        collector,
                    );

                    client.post(format!("{}/perf/onpush", site_url)).send()?;

                    res.fail_if_nonzero()?;
                }
            }

            Ok(0)
        }

        Commands::BenchPublished { toolchain, db } => {
            let pool = database::Pool::open(&db.db);
            bench_published_artifact(
                toolchain,
                pool,
                &mut rt,
                &target_triple,
                &compile_benchmark_dir,
            )?;
            Ok(0)
        }

        Commands::ProfileLocal {
            profiler,
            local,
            opts,
            out_dir,
            rustc2,
            jobs,
        } => {
            let jobs = jobs.max(1);
            if jobs > 1 && !profiler.supports_parallel_execution() {
                anyhow::bail!(
                    "Profiler {:?} does not support parallel execution.",
                    profiler
                );
            }

            let profiles = &opts.profiles.0;
            let scenarios = &opts.scenarios.0;

            let mut benchmarks = get_compile_benchmarks(
                &compile_benchmark_dir,
                local.include.as_deref(),
                local.exclude.as_deref(),
                local.exclude_suffix.as_deref(),
            )?;
            benchmarks.retain(|b| b.category().is_primary_or_secondary());

            let mut errors = BenchmarkErrors::new();

            eprintln!("Running with {jobs} job(s)");
            rayon::ThreadPoolBuilder::new()
                .num_threads(jobs as usize)
                .build_global()
                .unwrap();

            let mut get_toolchain_and_profile =
                |rustc: &str, suffix: &str| -> anyhow::Result<String> {
                    let toolchain = get_local_toolchain(
                        profiles,
                        rustc,
                        opts.rustdoc.as_deref(),
                        local.cargo.as_deref(),
                        local.id.as_deref(),
                        suffix,
                        target_triple.clone(),
                    )?;
                    let id = toolchain.id.clone();
                    profile(
                        &toolchain,
                        profiler,
                        &out_dir,
                        &benchmarks,
                        profiles,
                        scenarios,
                        &mut errors,
                    );
                    Ok(id)
                };

            if let Some(rustc2) = rustc2 {
                if local.rustc.starts_with('+') && local.rustc == rustc2 && local.id.is_none() {
                    anyhow::bail!("identical toolchain IDs; use --id to get distinct IDs");
                }
                let id1 = get_toolchain_and_profile(local.rustc.as_str(), "1")?;
                let id2 = get_toolchain_and_profile(rustc2.as_str(), "2")?;

                if profiler == Profiler::Cachegrind {
                    check_installed("valgrind")?;
                    check_installed("cg_annotate")?;
                    check_installed("rustfilt")?;

                    let diffs = generate_cachegrind_diffs(
                        &id1,
                        &id2,
                        &out_dir,
                        &benchmarks,
                        profiles,
                        scenarios,
                        &mut errors,
                    );
                    match diffs.len().cmp(&1) {
                        Ordering::Equal => {
                            let short = out_dir.join("cgann-diff-latest");
                            std::fs::copy(&diffs[0], &short).expect("copy to short path");
                            eprintln!("Original diff at: {}", diffs[0].to_string_lossy());
                            eprintln!("Short path: {}", short.to_string_lossy());
                        }
                        _ => {
                            eprintln!("Diffs:");
                            for diff in diffs {
                                eprintln!("{}", diff.to_string_lossy());
                            }
                        }
                    }
                }
            } else {
                get_toolchain_and_profile(local.rustc.as_str(), "")?;
            }

            errors.fail_if_nonzero()?;
            Ok(0)
        }

        Commands::InstallNext => {
            let last_sha = Command::new("git")
                .arg("ls-remote")
                .arg("https://github.com/rust-lang/rust.git")
                .arg("master")
                .output()
                .unwrap();
            let last_sha = String::from_utf8(last_sha.stdout).expect("utf8");
            let last_sha = last_sha.split_whitespace().next().expect(&last_sha);
            let commit = get_commit_or_fake_it(last_sha).expect("success");
            let mut sysroot = Sysroot::install(commit.sha, &target_triple)?;
            sysroot.preserve(); // don't delete it

            // Print the directory containing the toolchain.
            sysroot.rustc.pop();
            let s = format!("{:?}", sysroot.rustc);
            println!("{}", &s[1..s.len() - 1]);

            Ok(0)
        }
        Commands::Download(cmd) => {
            let target_dir = get_downloaded_crate_target(&compile_benchmark_dir, &cmd);
            check_target_dir(&target_dir, cmd.force)?;

            match cmd.command {
                DownloadSubcommand::Git { url } => download_from_git(&target_dir, &url)?,
                DownloadSubcommand::Crate { krate, version } => {
                    download_from_crates_io(&target_dir, &krate, &version)?
                }
            };

            add_perf_config(&target_dir, cmd.category);

            println!("Benchmark stored at {}", target_dir.display());
            Ok(0)
        }
    }
}

async fn init_compile_collector(
    pool: &database::Pool,
    benchmarks: &[Benchmark],
    bench_rustc: bool,
    artifact_id: ArtifactId,
) -> (Box<dyn Connection>, CollectorCtx) {
    let mut conn = pool.connection().await;
    let collector = CollectorStepBuilder::default()
        .record_compile_benchmarks(benchmarks, bench_rustc)
        .start_collection(conn.as_mut(), artifact_id)
        .await;
    (conn, collector)
}

fn bench_published_artifact(
    toolchain: String,
    pool: Pool,
    rt: &mut Runtime,
    target_triple: &str,
    benchmark_dir: &Path,
) -> anyhow::Result<()> {
    let status = Command::new("rustup")
        .args(["install", "--profile=minimal", &toolchain])
        .status()
        .context("rustup install")?;
    if !status.success() {
        anyhow::bail!("failed to install toolchain for {}", toolchain);
    }

    let profiles = if collector::version_supports_doc(&toolchain) {
        Profile::all()
    } else {
        Profile::all_non_doc()
    };
    let scenarios = if collector::version_supports_incremental(&toolchain) {
        Scenario::all()
    } else {
        Scenario::all_non_incr()
    };

    let which = |tool| {
        String::from_utf8(
            Command::new("rustup")
                .arg("which")
                .arg("--toolchain")
                .arg(&toolchain)
                .arg(tool)
                .output()
                .context(format!("rustup which {}", tool))?
                .stdout,
        )
        .context("utf8")
    };
    let rustc = which("rustc")?;
    let rustdoc = which("rustdoc")?;
    let cargo = which("cargo")?;

    // Exclude benchmarks that don't work with a stable compiler.
    let mut benchmarks = get_compile_benchmarks(benchmark_dir, None, None, None)?;
    benchmarks.retain(|b| b.category().is_stable());

    let artifact_id = ArtifactId::Tag(toolchain.clone());
    let (conn, collector) = rt.block_on(init_compile_collector(
        &pool,
        &benchmarks,
        /* bench_rustc */ false,
        artifact_id,
    ));
    let res = bench(
        rt,
        conn,
        &profiles,
        &scenarios,
        &Toolchain {
            rustc: PathBuf::from(rustc.trim()),
            rustdoc: Some(PathBuf::from(rustdoc.trim())),
            cargo: PathBuf::from(cargo.trim()),
            id: toolchain,
            triple: target_triple.to_string(),
        },
        &benchmarks,
        Some(3),
        /* is_self_profile */ false,
        collector,
    );
    res.fail_if_nonzero()?;
    Ok(())
}

fn add_perf_config(directory: &Path, category: Category) {
    let data = serde_json::json!({
        "category": category.to_string()
    });

    let mut file = BufWriter::new(
        File::create(directory.join("perf-config.json"))
            .expect("Could not create perf-config.json file"),
    );
    serde_json::to_writer_pretty(&mut file, &data).expect("Could not write perf-config.json file");
    file.write_all(b"\n").unwrap();
}

fn check_target_dir(target_dir: &Path, force: bool) -> anyhow::Result<()> {
    if target_dir.exists() {
        if force {
            std::fs::remove_dir_all(target_dir).unwrap_or_else(|_| {
                panic!(
                    "Cannot remove previous directory at {}",
                    target_dir.display()
                )
            });
        } else {
            return Err(anyhow::anyhow!(
                "Directory {} already exists",
                target_dir.display()
            ));
        }
    }
    Ok(())
}

fn get_downloaded_crate_target(benchmark_dir: &Path, cmd: &DownloadCommand) -> PathBuf {
    let name = cmd.name.clone().unwrap_or_else(|| match &cmd.command {
        // Git repository URLs sometimes end with .git, so we get rid of it.
        // URLs in general can end with /, which we also want to remove to make sure that the
        // last part of the URL is the repository name.
        DownloadSubcommand::Git { url } => url
            .trim_end_matches('/')
            .trim_end_matches(".git")
            .split('/')
            .last()
            .expect("Crate name could not be determined from git URL")
            .to_string(),
        DownloadSubcommand::Crate { krate, version } => format!("{krate}-{version}"),
    });
    PathBuf::from(benchmark_dir).join(name)
}

fn download_from_git(target: &Path, url: &str) -> anyhow::Result<()> {
    let tmpdir = tempfile::TempDir::new().unwrap();
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(tmpdir.path())
        .status()
        .expect("Git clone failed");
    generate_lockfile(tmpdir.path());

    // Save downloaded commit SHA to keep information about downloaded revision
    let git_sha = get_git_sha(tmpdir.path())?;
    std::fs::write(tmpdir.path().join("git-commit.txt"), git_sha)?;

    // Remove .git directory to avoid messing with nested git repositories
    if let Err(error) = std::fs::remove_dir_all(tmpdir.path().join(".git")) {
        log::error!("Could not delete .git directory: {error:?}");
    }

    utils::fs::rename(&tmpdir, target)?;
    Ok(())
}

fn get_git_sha(directory: &Path) -> anyhow::Result<String> {
    let stdout = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(directory)
        .output()?
        .stdout;
    Ok(String::from_utf8(stdout)?)
}

fn download_from_crates_io(target_dir: &Path, krate: &str, version: &str) -> anyhow::Result<()> {
    let url = format!("https://crates.io/api/v1/crates/{krate}/{version}/download");
    let response = reqwest::blocking::get(url)
        .expect("Cannot download crate")
        .error_for_status()?;

    let data = flate2::read::GzDecoder::new(response);
    let mut archive = tar::Archive::new(data);

    let tmpdir = tempfile::TempDir::new().unwrap();
    archive.unpack(&tmpdir)?;

    // The content of the crate is not at the package root, it should be nested
    // under <crate-name>-<version> directory.
    let unpacked_dir = tmpdir.path().join(format!("{krate}-{version}"));
    generate_lockfile(&unpacked_dir);
    utils::fs::rename(&unpacked_dir, target_dir)?;

    Ok(())
}

fn generate_lockfile(directory: &Path) {
    let manifest_path = directory.join("Cargo.toml");

    // Cargo metadata should do nothing if there is already a lockfile present.
    // Otherwise it will generate a lockfile.
    Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .current_dir(manifest_path.parent().unwrap())
        .stdout(std::process::Stdio::null())
        .status()
        .expect("Cannot generate lock file");
}

fn get_commit_or_fake_it(sha: &str) -> anyhow::Result<Commit> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    Ok(rt
        .block_on(collector::master_commits())
        .map_err(|e| anyhow::anyhow!("{:?}", e))
        .context("getting master commit list")?
        .into_iter()
        .find(|c| c.sha == *sha)
        .map(|c| Commit {
            sha: c.sha.as_str().into(),
            date: c.time.into(),
            r#type: CommitType::Master,
        })
        .unwrap_or_else(|| {
            log::warn!("utilizing fake commit!");
            Commit {
                sha: sha.into(),
                date: database::Date::ymd_hms(2000, 1, 1, 0, 0, 0),
                r#type: CommitType::Master,
            }
        }))
}
