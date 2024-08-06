#![recursion_limit = "1024"]

use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::future::Future;
use std::io::Write;
use std::io::{BufWriter, IsTerminal};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use std::time::Duration;
use std::{str, time::Instant};

use anyhow::Context;
use clap::builder::TypedValueParser;
use clap::{Arg, Parser};
use collector::compare::compare_artifacts;
use humansize::{format_size, BINARY};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tabled::builder::Builder;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Alignment, Border, Color, Modify};
use tokio::runtime::Runtime;

use collector::api::next_artifact::NextArtifact;
use collector::artifact_stats::{
    compile_and_get_stats, ArtifactStats, ArtifactWithStats, CargoProfile,
};
use collector::codegen::{codegen_diff, CodegenType};
use collector::compile::benchmark::category::Category;
use collector::compile::benchmark::codegen_backend::CodegenBackend;
use collector::compile::benchmark::profile::Profile;
use collector::compile::benchmark::scenario::Scenario;
use collector::compile::benchmark::{
    compile_benchmark_dir, get_compile_benchmarks, ArtifactType, Benchmark, BenchmarkName,
};
use collector::compile::execute::bencher::BenchProcessor;
use collector::compile::execute::profiler::{ProfileProcessor, Profiler};
use collector::runtime::{
    bench_runtime, get_runtime_benchmark_groups, prepare_runtime_benchmark_suite,
    runtime_benchmark_dir, BenchmarkFilter, BenchmarkSuite, BenchmarkSuiteCompilation,
    CargoIsolationMode, RuntimeProfiler, DEFAULT_RUNTIME_ITERATIONS,
};
use collector::runtime::{profile_runtime, RuntimeCompilationOpts};
use collector::toolchain::{
    create_toolchain_from_published_version, get_local_toolchain, Sysroot, Toolchain,
    ToolchainConfig,
};
use collector::utils::cachegrind::cachegrind_diff;
use collector::utils::{is_installed, wait_for_future};
use collector::{utils, CollectorCtx, CollectorStepBuilder};
use database::{ArtifactId, ArtifactIdNumber, Commit, CommitType, Connection, Pool};

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

#[derive(Debug)]
struct BenchmarkDirs<'a> {
    compile: &'a Path,
    runtime: &'a Path,
}

struct CompileBenchmarkConfig {
    benchmarks: Vec<Benchmark>,
    profiles: Vec<Profile>,
    scenarios: Vec<Scenario>,
    backends: Vec<CodegenBackend>,
    iterations: Option<usize>,
    is_self_profile: bool,
    bench_rustc: bool,
}

struct RuntimeBenchmarkConfig {
    runtime_suite: BenchmarkSuite,
    filter: BenchmarkFilter,
    iterations: u32,
}

impl RuntimeBenchmarkConfig {
    fn new(suite: BenchmarkSuite, filter: BenchmarkFilter, iterations: u32) -> Self {
        Self {
            runtime_suite: suite.filter(&filter),
            filter,
            iterations,
        }
    }
}

struct SharedBenchmarkConfig {
    artifact_id: ArtifactId,
    toolchain: Toolchain,
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
                let cgann_diff = out_dir.join(filename("cgann-diff", &id_diff));

                if let Err(e) = cachegrind_diff(&cgout1, &cgout2, &cgann_diff) {
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

#[allow(clippy::too_many_arguments)]
fn profile_compile(
    toolchain: &Toolchain,
    profiler: Profiler,
    out_dir: &Path,
    benchmarks: &[Benchmark],
    profiles: &[Profile],
    scenarios: &[Scenario],
    backends: &[CodegenBackend],
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
            let result = wait_for_future(benchmark.measure(
                &mut processor,
                profiles,
                scenarios,
                backends,
                toolchain,
                Some(1),
            ));
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

/// We need to have a separate wrapper over a Vec<T>, otherwise Clap would incorrectly
/// assume that `EnumArgParser` parses a single item, rather than a list of items.
#[derive(Clone, Debug)]
struct MultiEnumValue<T>(Vec<T>);

/// Parser for enums (like profile or scenario) which can be passed either as a comma-delimited
/// string or as the "All" string, which selects all variants.
#[derive(Clone)]
struct EnumArgParser<T>(PhantomData<T>);

impl<T> Default for EnumArgParser<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: clap::ValueEnum + Sync + Send + 'static> TypedValueParser for EnumArgParser<T> {
    type Value = MultiEnumValue<T>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        if value == "All" {
            Ok(MultiEnumValue(T::value_variants().to_vec()))
        } else {
            let values: Result<Vec<T>, _> = value
                .to_str()
                .unwrap()
                .split(',')
                .map(|item| clap::value_parser!(T).parse_ref(cmd, arg, OsStr::new(item)))
                .collect();

            Ok(MultiEnumValue(values?))
        }
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

    /// Arguments passed to `cargo --config <value>`.
    #[arg(long)]
    cargo_config: Vec<String>,

    /// Exclude all benchmarks matching a prefix in this comma-separated list
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<String>,

    /// Exclude all benchmarks matching a suffix in this comma-separated list
    #[arg(long, value_delimiter = ',')]
    exclude_suffix: Vec<String>,

    /// Include only benchmarks matching a prefix in this comma-separated list
    #[arg(long, value_delimiter = ',')]
    include: Vec<String>,

    /// Include only benchmarks belonging to the given categories.
    #[arg(long, value_parser = EnumArgParser::<Category>::default(), default_value = "Primary,Secondary")]
    category: MultiEnumValue<Category>,
}

#[derive(Debug, clap::Args)]
struct CompileTimeOptions {
    /// Measure the build profiles in this comma-separated list
    #[arg(
        long = "profiles",
        alias = "builds", // the old name, for backward compatibility
        value_parser = EnumArgParser::<Profile>::default(),
        // Don't run rustdoc by default
        default_value = "Check,Debug,Opt",
    )]
    profiles: MultiEnumValue<Profile>,

    /// Measure the scenarios in this comma-separated list
    #[arg(
        long = "scenarios",
        alias = "runs", // the old name, for backward compatibility
        value_parser = EnumArgParser::<Scenario>::default(),
        default_value = "All"
    )]
    scenarios: MultiEnumValue<Scenario>,

    /// Measure the codegen backends in this comma-separated list
    #[arg(long = "backends", value_parser = EnumArgParser::<CodegenBackend>::default(), default_value = "Llvm")]
    codegen_backends: MultiEnumValue<CodegenBackend>,

    /// The path to the local rustdoc to measure
    #[arg(long)]
    rustdoc: Option<PathBuf>,

    /// The path to the local clippy to measure
    #[arg(long)]
    clippy: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
struct RuntimeOptions {
    /// Select a runtime benchmark group that should be compiled and used. If not specified, all
    /// found groups will be compiled.
    #[arg(long)]
    group: Option<String>,
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

#[derive(Clone, Debug, clap::ValueEnum)]
enum PurgeMode {
    /// Purge all old data associated with the artifact
    Old,
    /// Purge old data of failed benchmarks associated with the artifact
    Failed,
}

#[derive(Debug, clap::Args)]
struct PurgeOption {
    /// Removes old data for the specified artifact prior to running the benchmarks.
    #[arg(long = "purge")]
    purge: Option<PurgeMode>,
}

#[derive(Debug, clap::Args)]
#[command(rename_all = "snake_case")]
struct BinaryStatsCompile {
    #[command(flatten)]
    local: LocalOptions,

    /// Cargo profile to use.
    #[arg(long, default_value = "Debug")]
    profile: Profile,

    /// Codegen backend to use.
    #[arg(long = "backend", default_value = "Llvm")]
    codegen_backend: CodegenBackend,

    /// An optional second toolchain to compare to.
    #[arg(long)]
    rustc2: Option<String>,

    /// Codegen backend to use for the second toolchain.
    #[arg(long = "backend2")]
    codegen_backend2: Option<CodegenBackend>,
}

#[derive(Debug, clap::Args)]
#[command(rename_all = "snake_case")]
struct BinaryStatsLocal {
    /// Binary artifact to examine.
    artifact: PathBuf,

    /// Optional second artifact to compare with the first one.
    artifact2: Option<PathBuf>,
}

#[derive(Debug, clap::Subcommand)]
#[command(rename_all = "snake_case")]
enum BinaryStatsMode {
    /// Show size statistics for the selected compile benchmark(s).
    /// Optionally compares sizes between two compiler toolchains, if `--rustc2` is provided.
    Compile(BinaryStatsCompile),
    /// Show size statistics for the selected binary artifact on disk.
    /// Optionally compares sizes with a second provided artifact.
    Local(BinaryStatsLocal),
}

// For each subcommand we list the mandatory arguments in the required
// order, followed by the options in alphabetical order.
#[derive(Debug, clap::Subcommand)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// Show binary (executable or library) section (and optionally symbol) size statistics.
    BinaryStats {
        /// Also print symbol comparison in addition to section comparison.
        ///
        /// Warning: may generate *A LOT* of data.
        #[arg(long, default_value_t = false, global = true)]
        symbols: bool,

        #[clap(subcommand)]
        mode: BinaryStatsMode,
    },

    /// Benchmarks the performance of programs generated by a local rustc
    BenchRuntimeLocal {
        #[command(flatten)]
        local: LocalOptions,

        #[command(flatten)]
        runtime: RuntimeOptions,

        /// How many iterations of each benchmark should be executed.
        #[arg(long, default_value_t = DEFAULT_RUNTIME_ITERATIONS)]
        iterations: u32,

        #[command(flatten)]
        db: DbOption,

        /// Compile runtime benchmarks directly in their crate directory, to make local experiments
        /// faster.
        #[arg(long = "no-isolate")]
        no_isolate: bool,

        #[command(flatten)]
        purge: PurgeOption,
    },

    /// Profiles a runtime benchmark.
    ProfileRuntime {
        #[command(flatten)]
        runtime: RuntimeOptions,

        /// Profiler to use
        profiler: RuntimeProfiler,

        /// The path to the local rustc used to compile the runtime benchmark
        rustc: String,

        /// The path to a second local rustc used to compare with the baseline rustc
        #[arg(long)]
        rustc2: Option<String>,

        /// Name of the benchmark that should be profiled
        benchmark: String,
    },

    /// Displays the diff between assembly, LLVM or MIR for a runtime benchmark group.
    CodegenDiff {
        /// Profiler to use
        codegen_type: CodegenType,

        /// Runtime benchmark group to diff (name of a directory in `collector/runtime-benchmarks`)
        group: String,

        /// The path to the local rustc used to compile the runtime benchmark
        rustc1: String,

        /// The path to a second rustc used to compile with the baseline
        rustc2: String,
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

        #[command(flatten)]
        purge: PurgeOption,
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
    InstallNext {
        /// Install additional components to enable benchmarking of the given backends.
        #[arg(long = "backends", value_parser = EnumArgParser::<CodegenBackend>::default(), default_value = "Llvm")]
        codegen_backends: MultiEnumValue<CodegenBackend>,
    },

    /// Download a crate into collector/benchmarks.
    Download(DownloadCommand),

    /// Removes all data associated with artifact(s) with the given name.
    PurgeArtifact {
        /// Name of the artifact.
        name: String,

        #[command(flatten)]
        db: DbOption,
    },

    /// Displays diff between two local bench results.
    BenchCmp {
        #[command(flatten)]
        db: DbOption,

        /// The name of the base artifact to be compared.
        base: String,

        /// The name of the modified artifact to be compared.
        modified: String,
    },
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
    #[arg(long, short('c'), value_enum, global = true, default_value = "Primary")]
    category: Category,

    /// What artifact type (library or binary) does the benchmark build.
    #[arg(long, short('a'), value_enum, global = true, default_value = "library")]
    artifact: ArtifactType,

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

    let benchmark_dirs = BenchmarkDirs {
        compile: &compile_benchmark_dir,
        runtime: &runtime_benchmark_dir,
    };

    // XXX: This doesn't necessarily work for all archs
    let target_triple = format!("{}-unknown-linux-gnu", std::env::consts::ARCH);

    match args.command {
        Commands::BinaryStats { mode, symbols } => {
            match mode {
                BinaryStatsMode::Compile(args) => {
                    binary_stats_compile(args, symbols, &target_triple)?;
                }
                BinaryStatsMode::Local(args) => {
                    binary_stats_local(args, symbols)?;
                }
            }

            Ok(0)
        }

        Commands::BenchRuntimeLocal {
            local,
            runtime,
            iterations,
            db,
            no_isolate,
            purge,
        } => {
            log_db(&db);
            let toolchain = get_local_toolchain_for_runtime_benchmarks(&local, &target_triple)?;
            let pool = Pool::open(&db.db);

            let isolation_mode = if no_isolate {
                CargoIsolationMode::Cached
            } else {
                CargoIsolationMode::Isolated
            };

            let mut rt = build_async_runtime();
            let mut conn = rt.block_on(pool.connection());
            let artifact_id = ArtifactId::Tag(toolchain.id.clone());
            rt.block_on(purge_old_data(conn.as_mut(), &artifact_id, purge.purge));

            let runtime_suite = rt.block_on(load_runtime_benchmarks(
                conn.as_mut(),
                &runtime_benchmark_dir,
                isolation_mode,
                runtime.group,
                &toolchain,
                &artifact_id,
            ))?;

            let shared = SharedBenchmarkConfig {
                artifact_id,
                toolchain,
            };
            let config = RuntimeBenchmarkConfig::new(
                runtime_suite,
                BenchmarkFilter::new(local.exclude, local.include),
                iterations,
            );
            run_benchmarks(&mut rt, conn, shared, None, Some(config))?;
            Ok(0)
        }
        Commands::ProfileRuntime {
            runtime,
            profiler,
            rustc,
            rustc2,
            benchmark,
        } => {
            let get_suite = |rustc: &str, id: &str| {
                let toolchain = get_local_toolchain(
                    &[Profile::Opt],
                    &[CodegenBackend::Llvm],
                    rustc,
                    ToolchainConfig::default(),
                    id,
                    target_triple.clone(),
                )?;
                let suite = prepare_runtime_benchmark_suite(
                    &toolchain,
                    &runtime_benchmark_dir,
                    CargoIsolationMode::Cached,
                    runtime.group.clone(),
                    // Compile with debuginfo to have filenames and line numbers available in the
                    // generated profiles.
                    RuntimeCompilationOpts::default().debug_info("1"),
                )?
                .extract_suite();
                Ok::<_, anyhow::Error>((toolchain, suite))
            };

            println!("Profiling {rustc}");
            let (toolchain1, suite1) = get_suite(&rustc, "1")?;
            let profile1 = profile_runtime(profiler.clone(), &toolchain1, suite1, &benchmark)?;

            if let Some(rustc2) = rustc2 {
                match profiler {
                    RuntimeProfiler::Cachegrind => {
                        println!("Profiling {rustc2}");
                        let (toolchain2, suite2) = get_suite(&rustc2, "2")?;
                        let profile2 = profile_runtime(profiler, &toolchain2, suite2, &benchmark)?;

                        let output = profile1.parent().unwrap().join(format!(
                            "cgann-diff-{}-{}-{benchmark}",
                            toolchain1.id, toolchain2.id
                        ));
                        cachegrind_diff(&profile1, &profile2, &output)
                            .context("Cannot generate Cachegrind diff")?;
                        println!("Cachegrind diff stored in `{}`", output.display());
                    }
                }
            } else {
                println!(
                    "Profiling complete, result can be found in `{}`",
                    profile1.display()
                );
            }

            Ok(0)
        }
        Commands::CodegenDiff {
            codegen_type,
            group,
            rustc1: rustc,
            rustc2,
        } => {
            let get_toolchain = |rustc: &str, id: &str| {
                let toolchain = get_local_toolchain(
                    &[Profile::Opt],
                    &[CodegenBackend::Llvm],
                    rustc,
                    ToolchainConfig::default(),
                    id,
                    target_triple.clone(),
                )?;
                Ok::<_, anyhow::Error>(toolchain)
            };

            let toolchain1 = get_toolchain(&rustc, "1")?;
            let toolchain2 = get_toolchain(&rustc2, "2")?;

            let mut benchmark_groups =
                get_runtime_benchmark_groups(&runtime_benchmark_dir, Some(group))?;
            let group = benchmark_groups.pop().expect("Benchmark group not found");
            assert!(benchmark_groups.is_empty());

            codegen_diff(codegen_type, toolchain1, toolchain2, group)?;
            Ok(0)
        }

        Commands::BenchLocal {
            local,
            opts,
            db,
            bench_rustc,
            iterations,
            self_profile,
            purge,
        } => {
            log_db(&db);
            let profiles = opts.profiles.0;
            let scenarios = opts.scenarios.0;
            let backends = opts.codegen_backends.0;

            let pool = database::Pool::open(&db.db);

            let toolchain = get_local_toolchain(
                &profiles,
                &backends,
                &local.rustc,
                *ToolchainConfig::default()
                    .rustdoc(opts.rustdoc.as_deref())
                    .clippy(opts.clippy.as_deref())
                    .cargo(local.cargo.as_deref(), local.cargo_config.as_slice())
                    .id(local.id.as_deref()),
                "",
                target_triple,
            )?;

            let mut benchmarks = get_compile_benchmarks(
                &compile_benchmark_dir,
                &local.include,
                &local.exclude,
                &local.exclude_suffix,
            )?;
            benchmarks.retain(|b| local.category.0.contains(&b.category()));

            let artifact_id = ArtifactId::Tag(toolchain.id.clone());
            let mut rt = build_async_runtime();
            let mut conn = rt.block_on(pool.connection());
            rt.block_on(purge_old_data(conn.as_mut(), &artifact_id, purge.purge));

            let shared = SharedBenchmarkConfig {
                toolchain,
                artifact_id,
            };
            let config = CompileBenchmarkConfig {
                benchmarks,
                profiles,
                scenarios,
                backends,
                iterations: Some(iterations),
                is_self_profile: self_profile.self_profile,
                bench_rustc: bench_rustc.bench_rustc,
            };

            run_benchmarks(&mut rt, conn, shared, Some(config), None)?;
            Ok(0)
        }

        Commands::BenchNext {
            site_url,
            db,
            bench_rustc,
            self_profile,
        } => {
            log_db(&db);
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

                // Sleep for a bit to avoid spamming the perf server too much
                // This sleep serves to remove a needless sleep in `collector/collect.sh` when
                // a benchmark was actually executed.
                std::thread::sleep(Duration::from_secs(60 * 2));

                // no missing artifacts
                return Ok(0);
            };

            let res = std::panic::catch_unwind(|| {
                let pool = database::Pool::open(&db.db);
                let mut rt = build_async_runtime();

                match next {
                    NextArtifact::Release(tag) => {
                        let toolchain =
                            create_toolchain_from_published_version(&tag, &target_triple)?;
                        bench_published_artifact(
                            rt.block_on(pool.connection()),
                            &mut rt,
                            toolchain,
                            &benchmark_dirs,
                        )
                    }
                    NextArtifact::Commit {
                        commit,
                        include,
                        exclude,
                        runs,
                    } => {
                        // FIXME: remove this when/if NextArtifact::Commit's include/exclude
                        // changed from Option<String> to Vec<String>
                        // to not to manually parse args
                        let split_args = |l: Option<String>| -> Vec<String> {
                            if let Some(l) = l {
                                l.split(',').map(|arg| arg.trim().to_owned()).collect()
                            } else {
                                vec![]
                            }
                        };
                        let sha = commit.sha.to_string();
                        let sysroot = Sysroot::install(
                            sha.clone(),
                            &target_triple,
                            vec![CodegenBackend::Llvm],
                        )
                        .with_context(|| format!("failed to install sysroot for {:?}", commit))?;

                        let mut benchmarks = get_compile_benchmarks(
                            &compile_benchmark_dir,
                            &split_args(include),
                            &split_args(exclude),
                            &[],
                        )?;
                        benchmarks.retain(|b| b.category().is_primary_or_secondary());

                        let artifact_id = ArtifactId::Commit(commit);
                        let mut conn = rt.block_on(pool.connection());
                        let toolchain = Toolchain::from_sysroot(&sysroot, sha);

                        let compile_config = CompileBenchmarkConfig {
                            benchmarks,
                            profiles: vec![
                                Profile::Check,
                                Profile::Debug,
                                Profile::Doc,
                                Profile::Opt,
                            ],
                            scenarios: Scenario::all(),
                            backends: vec![CodegenBackend::Llvm],
                            iterations: runs.map(|v| v as usize),
                            is_self_profile: self_profile.self_profile,
                            bench_rustc: bench_rustc.bench_rustc,
                        };
                        let runtime_suite = rt.block_on(load_runtime_benchmarks(
                            conn.as_mut(),
                            &runtime_benchmark_dir,
                            CargoIsolationMode::Isolated,
                            None,
                            &toolchain,
                            &artifact_id,
                        ))?;

                        let runtime_config = RuntimeBenchmarkConfig {
                            runtime_suite,
                            filter: BenchmarkFilter::keep_all(),
                            iterations: DEFAULT_RUNTIME_ITERATIONS,
                        };
                        let shared = SharedBenchmarkConfig {
                            artifact_id,
                            toolchain,
                        };

                        run_benchmarks(
                            &mut rt,
                            conn,
                            shared,
                            Some(compile_config),
                            Some(runtime_config),
                        )
                    }
                }
            });
            // We need to send a message to this endpoint even if the collector panics
            client.post(format!("{}/perf/onpush", site_url)).send()?;

            match res {
                Ok(res) => res?,
                Err(error) => {
                    log::error!("The collector has crashed\n{error:?}");
                }
            }
            Ok(0)
        }

        Commands::BenchPublished { toolchain, db } => {
            log_db(&db);
            let pool = database::Pool::open(&db.db);
            let mut rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());
            let toolchain = create_toolchain_from_published_version(&toolchain, &target_triple)?;
            bench_published_artifact(conn, &mut rt, toolchain, &benchmark_dirs)?;
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
            let backends = &opts.codegen_backends.0;

            let mut benchmarks = get_compile_benchmarks(
                &compile_benchmark_dir,
                &local.include,
                &local.exclude,
                &local.exclude_suffix,
            )?;
            benchmarks.retain(|b| local.category.0.contains(&b.category()));

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
                        &[CodegenBackend::Llvm],
                        rustc,
                        *ToolchainConfig::default()
                            .rustdoc(opts.rustdoc.as_deref())
                            .clippy(opts.clippy.as_deref())
                            .cargo(local.cargo.as_deref(), local.cargo_config.as_slice())
                            .id(local.id.as_deref()),
                        suffix,
                        target_triple.clone(),
                    )?;
                    let id = toolchain.id.clone();
                    profile_compile(
                        &toolchain,
                        profiler,
                        &out_dir,
                        &benchmarks,
                        profiles,
                        scenarios,
                        backends,
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

        Commands::InstallNext { codegen_backends } => {
            let last_sha = Command::new("git")
                .arg("ls-remote")
                .arg("https://github.com/rust-lang/rust.git")
                .arg("master")
                .output()
                .unwrap();
            let last_sha = String::from_utf8(last_sha.stdout).expect("utf8");
            let last_sha = last_sha.split_whitespace().next().expect(&last_sha);
            let commit = get_commit_or_fake_it(last_sha).expect("success");
            let mut sysroot = Sysroot::install(commit.sha, &target_triple, codegen_backends.0)?;
            sysroot.preserve(); // don't delete it

            // Print the directory containing the toolchain.
            sysroot.components.rustc.pop();
            let s = format!("{:?}", sysroot.components.rustc);
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

            add_perf_config(&target_dir, cmd.category, cmd.artifact);

            println!(
                r#"Benchmark stored at {dir} using category `{}` and artifact type `{}`.
Make sure to modify `{dir}/perf-config.json` if the category/artifact don't match your expectations."#,
                cmd.category,
                cmd.artifact,
                dir = target_dir.display(),
            );
            Ok(0)
        }
        Commands::PurgeArtifact { name, db } => {
            let pool = Pool::open(&db.db);
            let rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());
            rt.block_on(conn.purge_artifact(&ArtifactId::Tag(name.clone())));

            println!("Data of artifact {name} were removed");
            Ok(0)
        }
        Commands::BenchCmp { db, base, modified } => {
            let pool = Pool::open(&db.db);
            let rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());
            rt.block_on(compare_artifacts(conn, base, modified))?;
            Ok(0)
        }
    }
}

fn binary_stats_local(args: BinaryStatsLocal, symbols: bool) -> anyhow::Result<()> {
    let stats = ArtifactStats::from_path(&args.artifact)
        .with_context(|| format!("Cannot load artifact from {}", args.artifact.display()))?;
    let stats2 = args
        .artifact2
        .as_ref()
        .map(|path| {
            ArtifactStats::from_path(path)
                .with_context(|| format!("Cannot load artifact from {}", path.display()))
        })
        .transpose()?;
    print_binary_stats(
        "Sections",
        stats.sections,
        stats2.as_ref().map(|s| s.sections.clone()),
    );
    if symbols {
        print_binary_stats("Symbols", stats.symbols, stats2.map(|s| s.symbols));
    }

    Ok(())
}

fn binary_stats_compile(
    args: BinaryStatsCompile,
    symbols: bool,
    target_triple: &str,
) -> anyhow::Result<()> {
    let BinaryStatsCompile {
        local,
        profile,
        codegen_backend,
        rustc2,
        codegen_backend2,
    } = args;

    let codegen_backend2 = codegen_backend2.unwrap_or(codegen_backend);
    let toolchain = get_local_toolchain(
        &[Profile::Debug, Profile::Opt],
        &[codegen_backend],
        &local.rustc,
        *ToolchainConfig::default()
            .cargo(local.cargo.as_deref(), local.cargo_config.as_slice())
            .id(local.id.as_deref()),
        "",
        target_triple.to_string(),
    )?;
    let toolchain2 = rustc2
        .map(|rustc| {
            get_local_toolchain(
                &[Profile::Debug, Profile::Opt],
                &[codegen_backend2],
                &rustc,
                *ToolchainConfig::default()
                    .cargo(local.cargo.as_deref(), local.cargo_config.as_slice())
                    .id(local.id.as_deref()),
                "",
                target_triple.to_string(),
            )
        })
        .transpose()?;
    let profile = match profile {
        Profile::Debug => CargoProfile::Debug,
        Profile::Opt => CargoProfile::Release,
        _ => return Err(anyhow::anyhow!("Only Debug and Opt profiles are supported")),
    };
    let benchmarks = get_compile_benchmarks(
        &compile_benchmark_dir(),
        &local.include,
        &local.exclude,
        &local.exclude_suffix,
    )?;
    for benchmark in benchmarks {
        println!("Stats for benchmark `{}`", benchmark.name);
        println!("{}", "-".repeat(20));
        let artifacts =
            compile_and_get_stats(&benchmark.path, &toolchain, profile, codegen_backend)?;
        let archives2: HashMap<String, ArtifactWithStats> = toolchain2
            .as_ref()
            .map(|toolchain| {
                compile_and_get_stats(&benchmark.path, toolchain, profile, codegen_backend2)
            })
            .transpose()?
            .unwrap_or_default()
            .into_iter()
            .map(|artifact| (artifact.target_name.clone(), artifact))
            .collect();

        for artifact in artifacts {
            let archive2 = archives2.get(&artifact.target_name);

            println!(
                "Target `{}` (artifact `{}`)",
                artifact.target_name,
                artifact
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&artifact.target_name)
            );

            let sections = artifact.stats.sections;
            let sections2 = archive2.as_ref().map(|a| a.stats.sections.clone());
            print_binary_stats("Section", sections, sections2);

            if symbols {
                let symbols = artifact.stats.symbols;
                let symbols2 = archive2.as_ref().map(|a| a.stats.symbols.clone());
                print_binary_stats("Symbol", symbols, symbols2);
            }
            println!();
        }
    }
    Ok(())
}

fn build_async_runtime() -> Runtime {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    // We want to minimize noise from the runtime
    builder
        .worker_threads(1)
        .max_blocking_threads(1)
        .enable_time()
        .enable_io();
    builder.build().expect("built runtime")
}

fn print_binary_stats(
    name_header: &str,
    items: HashMap<String, u64>,
    items2: Option<HashMap<String, u64>>,
) {
    let use_diff = items2.is_some();
    let items2 = items2.unwrap_or_default();
    let items_vec = items
        .clone()
        .into_iter()
        .filter(|s| !s.0.is_empty())
        .collect::<Vec<_>>();

    let mut builder = Builder::default();
    if use_diff {
        builder.set_header([
            name_header,
            "Size (before)",
            "Size (after)",
            "Diff",
            "Diff (%)",
        ]);
    } else {
        builder.set_header([name_header, "Size"]);
    }

    struct Row {
        name: String,
        before: u64,
        after: u64,
        diff: i64,
    }

    let mut rows = items_vec
        .into_iter()
        .map(|(section, size)| {
            let after = items2.get(&section).copied().unwrap_or(0);

            let diff = after as i64 - size as i64;
            Row {
                name: section,
                before: size,
                after,
                diff,
            }
        })
        .collect::<Vec<_>>();

    let total_before: u64 = items.values().sum();
    let total_after: u64 = items2.values().sum();

    // Fill in items from the second artifact that are not present in the first one
    if use_diff {
        for (section, size) in items2 {
            if !items.contains_key(&section) {
                rows.push(Row {
                    name: section,
                    before: 0,
                    after: size,
                    diff: size as i64,
                });
            }
        }
    }
    rows.sort_by_cached_key(|row| Reverse((row.diff.abs(), row.before, row.name.clone())));

    // Combine all unchanged rows into one.
    if use_diff {
        let mut unchanged_count = 0;
        let mut total_unchanged = 0;
        rows.retain(|row| {
            if row.diff == 0 {
                unchanged_count += 1;
                total_unchanged += row.before;
                false
            } else {
                true
            }
        });
        if total_unchanged > 0 {
            rows.push(Row {
                name: format!("<{unchanged_count} unchanged rows>"),
                before: total_unchanged,
                after: total_unchanged,
                diff: 0,
            });
        }
    }

    rows.push(Row {
        name: "Total".to_string(),
        before: total_before,
        after: total_after,
        diff: total_after as i64 - total_before as i64,
    });

    let mut smaller_rows = vec![];
    let mut larger_rows = vec![];
    for (index, row) in rows.into_iter().enumerate() {
        let Row {
            name,
            before,
            after,
            diff,
        } = row;
        let mut columns = vec![name, format_size(before, BINARY)];
        if use_diff {
            columns.push(format_size(after, BINARY));
            let diff_str = format!("{}{diff}", if diff > 0 { "+" } else { "" });
            columns.push(diff_str);

            let diff_percent = if before == 0 {
                if after == 0 {
                    "0.0%".to_string()
                } else {
                    "+100.0%".to_string()
                }
            } else {
                let percent = ((after as f64 / before as f64) - 1.0) * 100.0;
                format!("{}{:.1}%", if percent > 0.0 { "+" } else { "" }, percent)
            };

            columns.push(diff_percent);
            match diff.cmp(&0) {
                Ordering::Less => {
                    smaller_rows.push(index);
                }
                Ordering::Greater => {
                    larger_rows.push(index);
                }
                _ => {}
            }
        }
        builder.push_record(columns);
    }

    let mut table = builder.build();
    let use_color = std::io::stdout().is_terminal();
    if use_color {
        for row in smaller_rows {
            table.with(Modify::new((row + 1, 3)).with(Color::FG_GREEN));
            table.with(Modify::new((row + 1, 4)).with(Color::FG_GREEN));
        }
        for row in larger_rows {
            table.with(Modify::new((row + 1, 3)).with(Color::FG_RED));
            table.with(Modify::new((row + 1, 4)).with(Color::FG_RED));
        }
    }

    table.with(Modify::new(Columns::new(1..)).with(Alignment::right()));
    table.with(tabled::settings::Style::sharp());
    table.with(
        Modify::new(Rows::last()).with(
            Border::default()
                .top('─')
                .corner_top_left('│')
                .corner_top_right('│'),
        ),
    );
    println!("{}", table);
}

fn get_local_toolchain_for_runtime_benchmarks(
    local: &LocalOptions,
    target_triple: &str,
) -> anyhow::Result<Toolchain> {
    get_local_toolchain(
        &[Profile::Opt],
        &[CodegenBackend::Llvm],
        &local.rustc,
        *ToolchainConfig::default()
            .cargo(local.cargo.as_deref(), local.cargo_config.as_slice())
            .id(local.id.as_deref()),
        "",
        target_triple.to_string(),
    )
}

async fn load_runtime_benchmarks(
    conn: &mut dyn Connection,
    benchmark_dir: &Path,
    isolation_mode: CargoIsolationMode,
    group: Option<String>,
    toolchain: &Toolchain,
    artifact_id: &ArtifactId,
) -> anyhow::Result<BenchmarkSuite> {
    let BenchmarkSuiteCompilation {
        suite,
        failed_to_compile,
    } = prepare_runtime_benchmark_suite(
        toolchain,
        benchmark_dir,
        isolation_mode,
        group,
        RuntimeCompilationOpts::default(),
    )?;

    record_runtime_compilation_errors(conn, artifact_id, failed_to_compile).await;
    Ok(suite)
}

async fn record_runtime_compilation_errors(
    connection: &mut dyn Connection,
    artifact_id: &ArtifactId,
    errors: HashMap<String, String>,
) {
    let artifact_row_number = connection.artifact_id(artifact_id).await;
    for (krate, error) in errors {
        connection
            .record_error(artifact_row_number, &krate, &error)
            .await;
    }
}

fn log_db(db_option: &DbOption) {
    println!("Using database `{}`", db_option.db);
}

async fn purge_old_data(
    conn: &mut dyn Connection,
    artifact_id: &ArtifactId,
    purge_mode: Option<PurgeMode>,
) {
    match purge_mode {
        Some(PurgeMode::Old) => {
            // Delete everything associated with the artifact
            conn.purge_artifact(artifact_id).await;
        }
        Some(PurgeMode::Failed) => {
            // Delete all benchmarks that have an error for the given artifact
            let artifact_row_id = conn.artifact_id(artifact_id).await;
            let errors = conn.get_error(artifact_row_id).await;
            for krate in errors.keys() {
                conn.collector_remove_step(artifact_row_id, krate).await;
            }
        }
        None => {}
    }
}

/// Record a collection entry into the database, specifying which benchmark steps will be executed.
async fn init_collection(
    connection: &mut dyn Connection,
    shared: &SharedBenchmarkConfig,
    compile: Option<&CompileBenchmarkConfig>,
    runtime: Option<&RuntimeBenchmarkConfig>,
) -> CollectorCtx {
    assert!(runtime.is_some() || compile.is_some());
    let mut builder = CollectorStepBuilder::default();
    if let Some(compile) = compile {
        builder = builder.record_compile_benchmarks(&compile.benchmarks, compile.bench_rustc);
    }
    if let Some(runtime) = runtime {
        builder = builder.record_runtime_benchmarks(&runtime.runtime_suite);
    }
    builder
        .start_collection(connection, &shared.artifact_id)
        .await
}

/// Execute all benchmarks specified by the given configurations.
fn run_benchmarks(
    rt: &mut Runtime,
    mut connection: Box<dyn Connection>,
    shared: SharedBenchmarkConfig,
    compile: Option<CompileBenchmarkConfig>,
    runtime: Option<RuntimeBenchmarkConfig>,
) -> anyhow::Result<()> {
    rt.block_on(record_toolchain_sizes(
        connection.as_mut(),
        &shared.artifact_id,
        &shared.toolchain,
    ));

    let collector = rt.block_on(init_collection(
        connection.as_mut(),
        &shared,
        compile.as_ref(),
        runtime.as_ref(),
    ));

    let start = Instant::now();

    // Compile benchmarks
    let compile_result = if let Some(compile) = compile {
        let errors = bench_compile(rt, connection.as_mut(), &shared, compile, &collector);
        errors
            .fail_if_nonzero()
            .context("Compile benchmarks failed")
    } else {
        Ok(())
    };

    // Runtime benchmarks
    let runtime_result = if let Some(runtime) = runtime {
        rt.block_on(bench_runtime(
            connection.as_mut(),
            runtime.runtime_suite,
            &collector,
            runtime.filter,
            runtime.iterations,
        ))
        .context("Runtime benchmarks failed")
    } else {
        Ok(())
    };

    let end = start.elapsed();
    rt.block_on(connection.record_duration(collector.artifact_row_id, end));

    compile_result.and(runtime_result)
}

/// Perform benchmarks on a published artifact.
fn bench_published_artifact(
    mut connection: Box<dyn Connection>,
    rt: &mut Runtime,
    toolchain: Toolchain,
    dirs: &BenchmarkDirs,
) -> anyhow::Result<()> {
    let artifact_id = ArtifactId::Tag(toolchain.id.clone());

    let profiles = if collector::version_supports_doc(&toolchain.id) {
        vec![Profile::Check, Profile::Debug, Profile::Doc, Profile::Opt]
    } else {
        vec![Profile::Check, Profile::Debug, Profile::Opt]
    };
    let scenarios = if collector::version_supports_incremental(&toolchain.id) {
        Scenario::all()
    } else {
        Scenario::all_non_incr()
    };

    // Exclude benchmarks that don't work with a stable compiler.
    let mut compile_benchmarks = get_compile_benchmarks(dirs.compile, &[], &[], &[])?;
    compile_benchmarks.retain(|b| b.category().is_stable());

    let runtime_suite = rt.block_on(load_runtime_benchmarks(
        connection.as_mut(),
        dirs.runtime,
        CargoIsolationMode::Isolated,
        None,
        &toolchain,
        &artifact_id,
    ))?;

    let shared = SharedBenchmarkConfig {
        artifact_id,
        toolchain,
    };
    run_benchmarks(
        rt,
        connection,
        shared,
        Some(CompileBenchmarkConfig {
            benchmarks: compile_benchmarks,
            profiles,
            scenarios,
            backends: vec![CodegenBackend::Llvm],
            iterations: Some(3),
            is_self_profile: false,
            bench_rustc: false,
        }),
        Some(RuntimeBenchmarkConfig::new(
            runtime_suite,
            BenchmarkFilter::keep_all(),
            DEFAULT_RUNTIME_ITERATIONS,
        )),
    )
}

const COMPILE_BENCHMARK_TIMEOUT: Duration = Duration::from_secs(60 * 30);

async fn with_timeout<F: Future<Output = anyhow::Result<()>>>(fut: F) -> anyhow::Result<()> {
    match tokio::time::timeout(COMPILE_BENCHMARK_TIMEOUT, fut).await {
        Ok(res) => res,
        Err(_) => Err(anyhow::anyhow!(
            "Benchmark timeouted in {} seconds",
            COMPILE_BENCHMARK_TIMEOUT.as_secs()
        )),
    }
}

/// Perform compile benchmarks.
fn bench_compile(
    rt: &mut Runtime,
    conn: &mut dyn Connection,
    shared: &SharedBenchmarkConfig,
    config: CompileBenchmarkConfig,
    collector: &CollectorCtx,
) -> BenchmarkErrors {
    let mut errors = BenchmarkErrors::new();
    eprintln!(
        "Benchmarking {} for triple {}",
        shared.artifact_id, shared.toolchain.triple
    );

    let bench_rustc = config.bench_rustc;

    let start = Instant::now();

    let mut measure_and_record =
        |benchmark_name: &BenchmarkName,
         category: Category,
         print_intro: &dyn Fn(),
         measure: &dyn Fn(&mut BenchProcessor) -> anyhow::Result<()>| {
            let is_fresh = rt.block_on(collector.start_compile_step(conn, benchmark_name));
            if !is_fresh {
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
                tx.conn(),
                benchmark_name,
                &shared.artifact_id,
                collector.artifact_row_id,
                config.is_self_profile,
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
    for (nth_benchmark, benchmark) in config.benchmarks.iter().enumerate() {
        measure_and_record(
            &benchmark.name,
            benchmark.category(),
            &|| {
                eprintln!(
                    "{}",
                    n_normal_benchmarks_remaining(config.benchmarks.len() - nth_benchmark)
                )
            },
            &|processor| {
                rt.block_on(with_timeout(benchmark.measure(
                    processor,
                    &config.profiles,
                    &config.scenarios,
                    &config.backends,
                    &shared.toolchain,
                    config.iterations,
                )))
                .with_context(|| anyhow::anyhow!("Cannot compile {}", benchmark.name))
            },
        )
    }

    // The special rustc benchmark, if requested.
    if bench_rustc {
        measure_and_record(
            &BenchmarkName("rustc".to_string()),
            Category::Primary,
            &|| eprintln!("Special benchmark commencing (due to `--bench-rustc`)"),
            &|processor| {
                rt.block_on(with_timeout(processor.measure_rustc(&shared.toolchain)))
                    .context("measure rustc")
            },
        );
    }

    let end = start.elapsed();

    eprintln!(
        "collection took {:?} with {} failed benchmarks",
        end, errors.0
    );

    rt.block_on(async move {
        // This ensures that we're good to go with the just updated data.
        conn.maybe_create_indices().await;
    });
    errors
}

/// Records the sizes of individual components (rustc, libLLVM, etc.) for the given toolchain
/// and artifact id into the database.
async fn record_toolchain_sizes(
    conn: &mut dyn Connection,
    artifact_id: &ArtifactId,
    toolchain: &Toolchain,
) {
    let aid = conn.artifact_id(artifact_id).await;

    async fn record(
        conn: &mut dyn Connection,
        aid: ArtifactIdNumber,
        component: &str,
        path: Option<&Path>,
    ) {
        if let Some(path) = path {
            if let Ok(size) = fs::metadata(path).map(|m| m.len()) {
                conn.record_artifact_size(aid, component, size).await;
            }
        }
    }

    let paths = &toolchain.components;
    record(conn, aid, "rustc", Some(&paths.rustc)).await;
    record(conn, aid, "rustdoc", paths.rustdoc.as_deref()).await;
    record(conn, aid, "cargo", Some(&paths.cargo)).await;
    record(conn, aid, "librustc_driver", paths.lib_rustc.as_deref()).await;
    record(conn, aid, "libstd", paths.lib_std.as_deref()).await;
    record(conn, aid, "libtest", paths.lib_test.as_deref()).await;
    record(conn, aid, "libLLVM", paths.lib_llvm.as_deref()).await;
}

fn add_perf_config(directory: &Path, category: Category, artifact: ArtifactType) {
    let data = serde_json::json!({
        "category": category,
        "artifact": artifact
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
