#![recursion_limit = "1024"]

use anyhow::Context;
use chrono::Utc;
use clap::builder::TypedValueParser;
use clap::{Arg, Parser};
use collector::compare::compare_artifacts;
use hashbrown::HashSet;
use humansize::{format_size, BINARY};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
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
use std::str::FromStr;
use std::time::Duration;
use std::{str, time::Instant};
use tabled::builder::Builder;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::style::Border;
use tabled::settings::{Alignment, Color, Modify, Width};
use tokio::runtime::Runtime;

use collector::artifact_stats::{
    compile_and_get_stats, ArtifactStats, ArtifactWithStats, CargoProfile,
};
use collector::benchmark_set::{get_benchmark_set, BenchmarkSetId, BenchmarkSetMember};
use collector::codegen::{codegen_diff, CodegenType};
use collector::compile::benchmark::category::Category;
use collector::compile::benchmark::codegen_backend::CodegenBackend;
use collector::compile::benchmark::profile::Profile;
use collector::compile::benchmark::scenario::Scenario;
use collector::compile::benchmark::target::Target;
use collector::compile::benchmark::{
    compile_benchmark_dir, get_compile_benchmarks, ArtifactType, Benchmark, BenchmarkName,
    CompileBenchmarkFilter,
};
use collector::compile::execute::bencher::BenchProcessor;
use collector::compile::execute::profiler::{ProfileProcessor, Profiler};
use collector::runtime::{
    bench_runtime, get_runtime_benchmark_groups, prepare_runtime_benchmark_suite,
    runtime_benchmark_dir, BenchmarkSuite, BenchmarkSuiteCompilation, CargoIsolationMode,
    RuntimeBenchmarkFilter, RuntimeProfiler, DEFAULT_RUNTIME_ITERATIONS,
};
use collector::runtime::{profile_runtime, RuntimeCompilationOpts};
use collector::toolchain::{
    create_toolchain_from_published_version, get_local_toolchain, Sysroot, SysrootDownloadError,
    Toolchain, ToolchainConfig,
};
use collector::utils::cachegrind::cachegrind_diff;
use collector::utils::{is_installed, wait_for_future};
use collector::{
    command_output, utils, CollectorCtx, CollectorStepBuilder, LocalSelfProfileStorage,
    S3SelfProfileStorage, SelfProfileStorage,
};
use database::{
    ArtifactId, ArtifactIdNumber, BenchmarkJob, BenchmarkJobConclusion, CollectorConfig, Commit,
    CommitType, Connection, Pool,
};

/// Directory used to cache downloaded Rust toolchains on disk.
const TOOLCHAIN_CACHE_DIRECTORY: &str = "cache";

/// Maximum allowed number of toolchains in the toolchain cache directory.
/// If the directory will have more toolchains, it will be purged.
const TOOLCHAIN_CACHE_MAX_TOOLCHAINS: usize = 30;

fn n_normal_benchmarks_remaining(n: usize) -> String {
    let suffix = if n == 1 { "" } else { "s" };
    format!("{n} normal benchmark{suffix} remaining")
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
    self_profile_storage: Option<Box<dyn SelfProfileStorage>>,
    bench_rustc: bool,
    targets: Vec<Target>,
}

struct RuntimeBenchmarkConfig {
    runtime_suite: BenchmarkSuite,
    filter: RuntimeBenchmarkFilter,
    iterations: u32,
    target: Target,
}

impl RuntimeBenchmarkConfig {
    fn new(
        suite: BenchmarkSuite,
        filter: RuntimeBenchmarkFilter,
        iterations: u32,
        target: Target,
    ) -> Self {
        Self {
            runtime_suite: suite.filter(&filter),
            filter,
            iterations,
            target,
        }
    }
}

struct SharedBenchmarkConfig {
    artifact_id: ArtifactId,
    toolchain: Toolchain,
    job_id: Option<u32>,
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

#[allow(clippy::too_many_arguments)]
fn generate_diffs(
    id1: &str,
    id2: &str,
    out_dir: &Path,
    benchmarks: &[Benchmark],
    profiles: &[Profile],
    scenarios: &[Scenario],
    errors: &mut BenchmarkErrors,
    profiler: &Profiler,
) -> Vec<PathBuf> {
    let mut annotated_diffs = Vec::new();
    for benchmark in benchmarks {
        for &profile in profiles {
            for scenario in scenarios.iter().flat_map(|scenario| {
                if profile.is_doc() && scenario.is_incr() {
                    return vec![];
                }
                match scenario {
                    Scenario::Full | Scenario::IncrFull | Scenario::IncrUnchanged => {
                        vec![format!("{:?}", scenario)]
                    }
                    Scenario::IncrPatched => (0..benchmark.patches.len())
                        .map(|i| format!("{scenario:?}{i}"))
                        .collect::<Vec<_>>(),
                }
            }) {
                let filename = |prefix, id| {
                    format!(
                        "{}-{}-{}-{:?}-{}{}",
                        prefix,
                        id,
                        benchmark.name,
                        profile,
                        scenario,
                        profiler.postfix()
                    )
                };
                let id_diff = format!("{id1}-{id2}");
                let prefix = profiler.prefix();
                let left = out_dir.join(filename(prefix, id1));
                let right = out_dir.join(filename(prefix, id2));
                let output = out_dir.join(filename(&format!("{prefix}-diff"), &id_diff));

                if let Err(e) = profiler.diff(&left, &right, &output) {
                    errors.incr();
                    eprintln!("collector error: {e:?}");
                    continue;
                }

                annotated_diffs.push(output);
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
    targets: &[Target],
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
                targets,
                // We always want to profile everything
                &hashbrown::HashSet::new(),
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
            eprintln!("collector error: {err:?}");
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

    /// Include only benchmarks in this comma-separated list
    #[arg(
        long,
        value_delimiter = ',',
        conflicts_with("include"),
        conflicts_with("exclude"),
        conflicts_with("exclude_suffix")
    )]
    exact_match: Vec<String>,

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

    /// The path to the local clippy to measure.
    /// It should be a path to the `clippy-driver` binary.
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
    #[arg(long, default_value = "results.db", env = "DATABASE_URL")]
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

        /// Metric used to compare artifacts.
        #[arg(long)]
        metric: Option<database::metric::Metric>,

        /// The name of the base artifact to be compared.
        base: Option<String>,

        /// The name of the modified artifact to be compared.
        modified: Option<String>,
    },

    /// Registers a new collector in the database.
    /// Use `--is_active` to immediately mark the collector as active.
    AddCollector {
        #[command(flatten)]
        db: DbOption,

        /// Name of the collector.
        #[arg(long)]
        collector_name: String,

        /// Target tuple which will the collector be benchmarking.
        #[arg(long)]
        target: String,

        /// Should the collector be marked as active immediately?
        /// Only active collectors will receive jobs.
        #[arg(long)]
        is_active: bool,

        /// The benchmark set index that the collector will be benchmarking.
        #[arg(long)]
        benchmark_set: u32,
    },

    /// Benchmark test cases pulled from the job queue.
    BenchmarkJobQueue {
        /// The unique identifier for the collector.
        /// It has to exist in the database; you can create new collectors using the `add_collector`
        /// command.
        #[arg(long)]
        collector_name: String,

        /// Git SHA of the commit that the collector is currently on.
        /// If not present, the collector will attempt to figure it out from git directly.
        #[arg(long)]
        git_sha: Option<String>,

        /// Periodically check if the collector's commit SHA matches the commit SHA of the
        /// rustc-perf repository.
        #[arg(long)]
        check_git_sha: bool,

        #[command(flatten)]
        db: DbOption,
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

impl<'a> From<&'a LocalOptions> for CompileBenchmarkFilter<'a> {
    fn from(value: &'a LocalOptions) -> Self {
        if !value.exact_match.is_empty() {
            Self::Exact(&value.exact_match)
        } else if !value.include.is_empty()
            || !value.exclude.is_empty()
            || !value.exclude_suffix.is_empty()
        {
            Self::Fuzzy {
                include: &value.include,
                exclude: &value.exclude,
                exclude_suffix: &value.exclude_suffix,
            }
        } else {
            Self::All
        }
    }
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

    // We need to find the host tuple for a couple of things (several collector commands need it).
    // Probably the simplest way of determining it is asking rustc what is its host tuple.
    // However, where to get that rustc? We could just try using "rustc", but that is not always
    // available, e.g. on Rust's CI.
    // So we try to figure out if we have some rustc available from the command that is being
    // executed; such rustc should definitely be executable on this host.
    // If we don't, we'll simply fall back to `rustc`.
    let used_rustc: Option<String> = match &args.command {
        Commands::BinaryStats {
            mode: BinaryStatsMode::Compile(args),
            ..
        } => Some(args.local.rustc.clone()),
        Commands::BenchRuntimeLocal { local, .. } => Some(local.rustc.clone()),
        Commands::ProfileRuntime { rustc, .. } => Some(rustc.clone()),
        Commands::CodegenDiff { rustc1, .. } => Some(rustc1.clone()),
        Commands::BenchLocal { local, .. } => Some(local.rustc.clone()),
        Commands::ProfileLocal { local, .. } => Some(local.rustc.clone()),
        Commands::BinaryStats {
            mode: BinaryStatsMode::Local(_),
            ..
        }
        | Commands::BenchPublished { .. }
        | Commands::InstallNext { .. }
        | Commands::Download(_)
        | Commands::PurgeArtifact { .. }
        | Commands::BenchCmp { .. }
        | Commands::AddCollector { .. }
        | Commands::BenchmarkJobQueue { .. } => None,
    };

    let host_target_tuple = match used_rustc {
        Some(rustc) if !rustc.starts_with("+") => get_host_tuple_from_rustc(&rustc),
        _ => get_host_tuple_from_rustc("rustc"),
    };
    // We only unwrap the host tuple in places where we actually need it, to avoid panicking if it
    // is missing, but we don't really need it.
    let require_host_target_tuple = || {
        host_target_tuple.expect(
            "Cannot determine host target tuple. Please make a `rustc` binary available in PATH.",
        )
    };

    match args.command {
        Commands::BinaryStats { mode, symbols } => {
            match mode {
                BinaryStatsMode::Compile(args) => {
                    binary_stats_compile(args, symbols, &require_host_target_tuple())?;
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

            let host_tuple = require_host_target_tuple();
            let toolchain = get_local_toolchain_for_runtime_benchmarks(&local, &host_tuple)?;
            let pool = Pool::open(&db.db);

            let isolation_mode = if no_isolate {
                CargoIsolationMode::Cached
            } else {
                CargoIsolationMode::Isolated
            };

            let rt = build_async_runtime();
            let mut conn = rt.block_on(pool.connection());
            let artifact_id = ArtifactId::Commit(Commit {
                sha: toolchain.id.clone(),
                date: Utc::now().into(),
                r#type: CommitType::Master,
            });

            rt.block_on(purge_old_data(conn.as_mut(), &artifact_id, purge.purge));

            let runtime_suite = rt.block_on(load_runtime_benchmarks(
                conn.as_mut(),
                &runtime_benchmark_dir,
                isolation_mode,
                runtime.group,
                &toolchain,
                &artifact_id,
                None,
            ))?;

            let shared = SharedBenchmarkConfig {
                artifact_id,
                toolchain,
                job_id: None,
            };
            let config = RuntimeBenchmarkConfig::new(
                runtime_suite,
                RuntimeBenchmarkFilter::new(local.exclude, local.include),
                iterations,
                Target::from_str(&host_tuple).expect("Found unexpected host target"),
            );
            rt.block_on(run_benchmarks(conn.as_mut(), shared, None, Some(config)))?;
            Ok(0)
        }
        Commands::ProfileRuntime {
            runtime,
            profiler,
            rustc,
            rustc2,
            benchmark,
        } => {
            let host_target_tuple = require_host_target_tuple();
            let get_suite = |rustc: &str, id: &str| {
                let toolchain = get_local_toolchain(
                    &[Profile::Opt],
                    &[CodegenBackend::Llvm],
                    rustc,
                    ToolchainConfig::default(),
                    id,
                    host_target_tuple.clone(),
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
            let host_target_tuple = require_host_target_tuple();
            let get_toolchain = |rustc: &str, id: &str| {
                let toolchain = get_local_toolchain(
                    &[Profile::Opt],
                    &[CodegenBackend::Llvm],
                    rustc,
                    ToolchainConfig::default(),
                    id,
                    host_target_tuple.clone(),
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
                require_host_target_tuple(),
            )?;

            let mut benchmarks = get_compile_benchmarks(&compile_benchmark_dir, (&local).into())?;
            benchmarks.retain(|b| local.category.0.contains(&b.category()));

            let artifact_id = ArtifactId::Commit(Commit {
                sha: toolchain.id.clone(),
                date: Utc::now().into(),
                r#type: CommitType::Master,
            });

            let rt = build_async_runtime();
            let mut conn = rt.block_on(pool.connection());
            rt.block_on(purge_old_data(conn.as_mut(), &artifact_id, purge.purge));

            let shared = SharedBenchmarkConfig {
                toolchain,
                artifact_id,
                job_id: None,
            };
            let config = CompileBenchmarkConfig {
                benchmarks,
                profiles,
                scenarios,
                backends,
                iterations: Some(iterations),
                self_profile_storage: if self_profile.self_profile {
                    Some(Box::new(LocalSelfProfileStorage::new(Path::new(
                        "self-profile-storage",
                    ))))
                } else {
                    None
                },
                bench_rustc: bench_rustc.bench_rustc,
                targets: vec![Target::default()],
            };

            rt.block_on(run_benchmarks(conn.as_mut(), shared, Some(config), None))?;
            Ok(0)
        }

        Commands::BenchPublished { toolchain, db } => {
            log_db(&db);
            let pool = database::Pool::open(&db.db);
            let rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());
            let toolchain =
                create_toolchain_from_published_version(&toolchain, &require_host_target_tuple())?;
            rt.block_on(bench_published_artifact(
                conn,
                toolchain,
                &benchmark_dirs,
                None,
            ))?;
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

            let mut benchmarks = get_compile_benchmarks(&compile_benchmark_dir, (&local).into())?;
            benchmarks.retain(|b| local.category.0.contains(&b.category()));

            let mut errors = BenchmarkErrors::new();

            eprintln!("Running with {jobs} job(s)");
            rayon::ThreadPoolBuilder::new()
                .num_threads(jobs as usize)
                .build_global()
                .unwrap();

            let host_target_tuple = require_host_target_tuple();
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
                        host_target_tuple.clone(),
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
                        &[Target::default()],
                    );
                    Ok(id)
                };

            if let Some(rustc2) = rustc2 {
                if local.rustc.starts_with('+') && local.rustc == rustc2 && local.id.is_none() {
                    anyhow::bail!("identical toolchain IDs; use --id to get distinct IDs");
                }
                let id1 = get_toolchain_and_profile(local.rustc.as_str(), "1")?;
                let id2 = get_toolchain_and_profile(rustc2.as_str(), "2")?;

                let diffs = generate_diffs(
                    &id1,
                    &id2,
                    &out_dir,
                    &benchmarks,
                    profiles,
                    scenarios,
                    &mut errors,
                    &profiler,
                );
                if let [diff] = &diffs[..] {
                    let short = out_dir.join(format!("{}-diff-latest", profiler.prefix()));
                    std::fs::copy(diff, &short).expect("copy to short path");
                    eprintln!("Original diff at: {}", diff.to_string_lossy());
                    eprintln!("Short path: {}", short.to_string_lossy());
                } else {
                    eprintln!("Diffs:");
                    for diff in diffs {
                        eprintln!("{}", diff.to_string_lossy());
                    }
                }
            } else {
                get_toolchain_and_profile(local.rustc.as_str(), "")?;
            }

            errors.fail_if_nonzero()?;
            Ok(0)
        }

        Commands::InstallNext { codegen_backends } => {
            let last_sha = get_latest_sha("https://github.com/rust-lang/rust").unwrap();
            let commit = get_commit_or_fake_it(&last_sha).expect("success");

            let rt = build_async_runtime();
            let mut sysroot = rt
                .block_on(Sysroot::install(
                    Path::new(TOOLCHAIN_CACHE_DIRECTORY),
                    commit.sha,
                    &require_host_target_tuple(),
                    &codegen_backends.0,
                ))
                .map_err(SysrootDownloadError::as_anyhow_error)?;
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
        Commands::BenchCmp {
            db,
            base,
            modified,
            metric,
        } => {
            let pool = Pool::open(&db.db);
            let rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());
            rt.block_on(compare_artifacts(conn, metric, base, modified))?;
            Ok(0)
        }

        Commands::AddCollector {
            db,
            collector_name,
            target,
            is_active,
            benchmark_set,
        } => {
            let pool = Pool::open(&db.db);
            let rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());

            let target = database::Target::from_str(&target).map_err(|e| anyhow::anyhow!(e))?;
            rt.block_on(conn.add_collector_config(
                &collector_name,
                target,
                benchmark_set,
                is_active,
            ))?;
            Ok(0)
        }

        Commands::BenchmarkJobQueue {
            collector_name,
            git_sha,
            check_git_sha,
            db,
        } => {
            log_db(&db);

            let git_sha = match git_sha {
                Some(sha) => sha,
                None => {
                    let mut cmd = Command::new("git");
                    cmd.args(["rev-parse", "HEAD"]);
                    let stdout = command_output(&mut cmd)
                        .context("Cannot determine current commit SHA")?
                        .stdout;
                    String::from_utf8(stdout).unwrap().trim().to_string()
                }
            };

            let pool = Pool::open(&db.db);
            let rt = build_async_runtime();
            let conn = rt.block_on(pool.connection());

            // Obtain the configuration and validate that it matches the
            // collector's host target
            let collector_config = rt
                .block_on(conn.start_collector(&collector_name, &git_sha))?
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "No active collector with the name `{collector_name}` not found"
                    )
                })?;

            let host_target_tuple = require_host_target_tuple();
            if collector_config.target().as_str() != host_target_tuple {
                return Err(anyhow::anyhow!(
                    "The collector `{collector_name}` is configured for target `{}`, but the current host target seems to be `{host_target_tuple}`",
                    collector_config.target()
                ));
            }

            log::info!(
                "Starting collector with target {}, benchmark set {} and commit {}",
                collector_config.target(),
                collector_config.benchmark_set().get_id(),
                collector_config.commit_sha().expect("missing commit SHA")
            );

            let benchmarks =
                get_compile_benchmarks(&compile_benchmark_dir, CompileBenchmarkFilter::All)?;

            rt.block_on(run_job_queue_benchmarks(
                pool,
                conn,
                &collector_config,
                benchmarks,
                check_git_sha,
            ))?;

            Ok(0)
        }
    }
}

fn get_host_tuple_from_rustc(rustc: &str) -> anyhow::Result<String> {
    Ok(
        String::from_utf8(command_output(Command::new(rustc).arg("--print=host-tuple"))?.stdout)?
            .trim()
            .to_string(),
    )
}

/// Maximum number of failures before a job will be marked as failed.
const MAX_JOB_FAILS: u32 = 3;

/// How long should the collector sleep for if it does not find any job in the job queue.
const JOB_WAIT_SLEEP_TIME: Duration = Duration::from_secs(30);

async fn run_job_queue_benchmarks(
    pool: Pool,
    mut conn: Box<dyn Connection>,
    collector: &CollectorConfig,
    all_compile_benchmarks: Vec<Benchmark>,
    check_git_sha: bool,
) -> anyhow::Result<()> {
    let _ = tidy_toolchain_cache_dir();

    let mut last_request_tag = None;

    // Outer loop - wait for some jobs to appear
    loop {
        if check_git_sha && needs_git_update(collector) {
            log::warn!("Exiting collector to update itself from git.");
            break;
        }

        while let Some((benchmark_job, artifact_id)) = conn
            .dequeue_benchmark_job(
                collector.name(),
                collector.target(),
                collector.benchmark_set(),
            )
            .await?
        {
            // Are we benchmarking a different benchmark request than in the previous iteration of the
            // loop?
            let is_new_request = last_request_tag.is_none()
                || (last_request_tag.as_deref() != Some(benchmark_job.request_tag()));
            if is_new_request {
                log::info!("Starting new request {}", benchmark_job.request_tag());
                let _ = tidy_toolchain_cache_dir();
            }

            // Here we check if we should update our commit SHA, if rustc-perf has been updated.
            // We only check for updates when we switch *benchmark requests*, not *benchmark jobs*,
            // to avoid changing code in the middle of benchmarking the same request.
            // Note that if an update happens, the job that we have just dequeued will have its deque
            // counter increased. But since updates are relatively rare, that shouldn't be a big deal,
            // it will be dequeued again when the collector starts again.
            if check_git_sha && is_new_request && needs_git_update(collector) {
                log::warn!("Exiting collector to update itself from git.");
                return Ok(());
            }

            last_request_tag = Some(benchmark_job.request_tag().to_string());

            log::info!("Dequeued job {benchmark_job:?}, artifact_id {artifact_id:?}");
            let result = run_benchmark_job(
                conn.as_mut(),
                &benchmark_job,
                artifact_id.clone(),
                &all_compile_benchmarks,
            )
            .await;
            match result {
                Ok(_) => {
                    log::info!("Job finished sucessfully");
                    conn.mark_benchmark_job_as_completed(
                        benchmark_job.id(),
                        BenchmarkJobConclusion::Success,
                    )
                    .await?;
                }
                Err(error) => {
                    match error {
                        BenchmarkJobError::Permanent(error) => {
                            log::error!("Job finished with permanent error: {error:?}");

                            // Store the error to the database
                            let artifact_row_id = conn.artifact_id(&artifact_id).await;
                            // Use a <job> placeholder to say that the error is associated with a job,
                            // not with a benchmark.
                            conn.record_error(
                                artifact_row_id,
                                "Job failure",
                                &format!(
                                    "Error while benchmarking job {benchmark_job:?}: {error:?}"
                                ),
                                Some(benchmark_job.id()),
                            )
                            .await;

                            // Something bad that probably cannot be retried has happened.
                            // Immediately mark the job as failed and continue with other jobs
                            log::info!("Marking the job as failed");
                            conn.mark_benchmark_job_as_completed(
                                benchmark_job.id(),
                                BenchmarkJobConclusion::Failure,
                            )
                            .await?;
                        }
                        BenchmarkJobError::Transient(error) => {
                            log::error!("Job finished with transient error: {error:?}");

                            // There was some transient (i.e. I/O, network or database) error.
                            // Let's retry the job later, with some sleep
                            log::info!("Retrying after 30s...");
                            tokio::time::sleep(Duration::from_secs(30)).await;

                            // Maybe there was a DB issue. Try to reconnect to the database.
                            conn = pool.connection().await;
                        }
                    }
                }
            }

            conn.update_collector_heartbeat(collector.name()).await?;
        }

        log::info!(
            "No job found, sleeping for {}s",
            JOB_WAIT_SLEEP_TIME.as_secs()
        );
        tokio::time::sleep(JOB_WAIT_SLEEP_TIME).await;
        conn.update_collector_heartbeat(collector.name()).await?;
    }
    Ok(())
}

/// Check the toolchain cache directory and delete it if it grows too large.
/// Currently, we just assume that "too large" means "has more than N toolchains".
fn tidy_toolchain_cache_dir() -> std::io::Result<()> {
    let dir_count = Path::new(TOOLCHAIN_CACHE_DIRECTORY)
        .read_dir()?
        .filter_map(|e| e.ok())
        .filter_map(|d| d.file_type().ok())
        .filter(|t| t.is_dir())
        .count();
    if dir_count > TOOLCHAIN_CACHE_MAX_TOOLCHAINS {
        log::warn!("Purging toolchain cache directory at {TOOLCHAIN_CACHE_DIRECTORY}");
        // Just remove the whole directory, to avoid having to figure out which toolchains are old
        std::fs::remove_dir_all(TOOLCHAIN_CACHE_DIRECTORY)?;
    }
    Ok(())
}

/// Returns true if the commit SHA of collector does not match the latest commit SHA of the master
/// branch of https://github.com/rust-lang/rustc-perf.
fn needs_git_update(collector: &CollectorConfig) -> bool {
    let Some(commit_sha) = collector.commit_sha() else {
        return false;
    };

    let Ok(upstream_sha) = get_latest_sha("https://github.com/rust-lang/rustc-perf") else {
        return false;
    };
    if commit_sha != upstream_sha {
        log::warn!(
            "Commit {commit_sha} of collector is outdated, latest commit is {upstream_sha}."
        );
        true
    } else {
        false
    }
}

/// Returns the latest known sha of the default branch of the specified `repo`.
fn get_latest_sha(repo: &str) -> anyhow::Result<String> {
    let mut cmd = Command::new("git");
    cmd.arg("ls-remote").arg(repo).arg("HEAD");
    match command_output(&mut cmd) {
        Ok(output) => Ok(String::from_utf8(output.stdout)?
            .split_whitespace()
            .next()
            .unwrap()
            .to_string()),
        Err(error) => {
            log::error!("Cannot determine latest SHA of {repo}: {error:?}");
            Err(error)
        }
    }
}

/// Error that happened during benchmarking of a job.
enum BenchmarkJobError {
    /// The error is non-recoverable.
    /// For example, a rustc toolchain does not exist on CI
    Permanent(anyhow::Error),
    Transient(anyhow::Error),
}

impl From<anyhow::Error> for BenchmarkJobError {
    fn from(error: anyhow::Error) -> Self {
        Self::Transient(error)
    }
}

async fn run_benchmark_job(
    conn: &mut dyn Connection,
    job: &BenchmarkJob,
    artifact_id: ArtifactId,
    all_compile_benchmarks: &[Benchmark],
) -> Result<(), BenchmarkJobError> {
    // Fail the job if it has been dequeued too many times
    if job.deque_count() > MAX_JOB_FAILS {
        return Err(BenchmarkJobError::Permanent(anyhow::anyhow!(
            "Job failed after being dequeued for {MAX_JOB_FAILS} times"
        )));
    }

    log::info!("Downloading sysroot");
    let toolchain = match &artifact_id {
        ArtifactId::Commit(commit) => {
            let mut sysroot = match Sysroot::install(
                Path::new(TOOLCHAIN_CACHE_DIRECTORY),
                commit.sha.clone(),
                job.target().as_str(),
                &[job.backend().into()],
            )
            .await
            {
                Ok(sysroot) => sysroot,
                Err(SysrootDownloadError::SysrootShaNotFound) => {
                    return Err(BenchmarkJobError::Permanent(anyhow::anyhow!(
                        "Artifacts for SHA `{}`, target `{}` and backend `{}` were not found on CI servers",
                        commit.sha,
                        job.target().as_str(),
                        job.backend().as_str()
                    )))
                }
                Err(SysrootDownloadError::IO(error)) => return Err(error.into()),
            };
            // Avoid redownloading the same sysroot multiple times for different jobs, even
            // across collector restarts.
            sysroot.preserve();
            Toolchain::from_sysroot(&sysroot, commit.sha.clone())
        }
        ArtifactId::Tag(tag) => {
            create_toolchain_from_published_version(tag, job.target().as_str())?
        }
    };
    log::info!("Sysroot download finished");

    let (compile_config, runtime_config) =
        create_benchmark_configs(conn, &toolchain, &artifact_id, job, all_compile_benchmarks)
            .await
            .map_err(|error| {
                BenchmarkJobError::Permanent(anyhow::anyhow!(
                    "Cannot prepare benchmark configs: {error:?}"
                ))
            })?;
    if compile_config.is_none() && runtime_config.is_none() {
        log::warn!("Nothing to benchmark");
        return Ok(());
    }

    let shared = SharedBenchmarkConfig {
        artifact_id,
        toolchain,
        job_id: Some(job.id()),
    };

    // A failure here means that it was not possible to compile something, that likely won't resolve
    // itself automatically.
    run_benchmarks(conn, shared, compile_config, runtime_config)
        .await
        .map_err(|error| {
            BenchmarkJobError::Permanent(anyhow::anyhow!("Cannot run benchmarks: {error:?}"))
        })?;
    Ok(())
}

async fn create_benchmark_configs(
    conn: &mut dyn Connection,
    toolchain: &Toolchain,
    artifact_id: &ArtifactId,
    job: &BenchmarkJob,
    all_compile_benchmarks: &[Benchmark],
) -> anyhow::Result<(
    Option<CompileBenchmarkConfig>,
    Option<RuntimeBenchmarkConfig>,
)> {
    // Expand the benchmark set and figure out which benchmarks should be executed
    let benchmark_set_id = BenchmarkSetId::new(job.target().into(), job.benchmark_set().get_id());
    let benchmark_set = get_benchmark_set(benchmark_set_id);
    log::debug!(
        "Expanded benchmark set members: {:?}",
        benchmark_set.members()
    );

    let mut bench_rustc = false;
    let mut bench_runtime = false;
    let mut bench_compile_benchmarks = HashSet::new();

    let is_release = match artifact_id {
        ArtifactId::Commit(_) => false,
        ArtifactId::Tag(_) => true,
    };

    match job.kind() {
        database::BenchmarkJobKind::Runtime => {
            bench_runtime = true;
        }
        database::BenchmarkJobKind::Compiletime => {
            for member in benchmark_set.members() {
                match member {
                    BenchmarkSetMember::CompileBenchmark(benchmark) => {
                        bench_compile_benchmarks.insert(benchmark);
                    }
                }
            }
        }
        database::BenchmarkJobKind::Rustc => {
            assert!(!is_release, "Release job should not benchmark rustc");
            bench_rustc = true;
        }
    }

    let benchmarks: Vec<Benchmark> = all_compile_benchmarks
        .iter()
        .filter(|b| {
            if !bench_compile_benchmarks.contains(&b.name) {
                return false;
            }
            // Run stable benchmarks for releases and non-stable benchmarks for everything
            // else.
            let is_stable = b.category().is_stable();
            is_release == is_stable
        })
        .cloned()
        .collect();

    let compile_config = if bench_rustc || !benchmarks.is_empty() {
        Some(CompileBenchmarkConfig {
            benchmarks,
            profiles: vec![job.profile().into()],
            scenarios: Scenario::all(),
            backends: vec![job.backend().into()],
            iterations: if is_release { Some(3) } else { None },
            self_profile_storage: if !is_release {
                Some(Box::new(S3SelfProfileStorage::new()))
            } else {
                None
            },
            bench_rustc,
            targets: vec![job.target().into()],
        })
    } else {
        None
    };

    let runtime_config = if bench_runtime {
        let runtime_suite = load_runtime_benchmarks(
            conn,
            &runtime_benchmark_dir(),
            CargoIsolationMode::Isolated,
            None,
            toolchain,
            artifact_id,
            Some(job.id()),
        )
        .await?;
        Some(RuntimeBenchmarkConfig {
            runtime_suite,
            filter: RuntimeBenchmarkFilter::keep_all(),
            iterations: DEFAULT_RUNTIME_ITERATIONS,
            target: job.target().into(),
        })
    } else {
        None
    };

    Ok((compile_config, runtime_config))
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
    let benchmarks = get_compile_benchmarks(&compile_benchmark_dir(), (&local).into())?;
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
        builder.push_record([
            name_header,
            "Size (before)",
            "Size (after)",
            "Diff",
            "Diff (%)",
        ]);
    } else {
        builder.push_record([name_header, "Size"]);
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

    table.with(Modify::new(Columns::first()).with(Width::wrap(80)));
    table.with(Modify::new(Columns::new(1..)).with(Alignment::right()));
    table.with(tabled::settings::Style::sharp());
    table.with(
        Modify::new(Rows::last()).with(
            Border::new()
                .top('─')
                .left('│')
                .right('│')
                .corner_top_left('│')
                .corner_top_right('│'),
        ),
    );
    println!("{table}");
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
    job_id: Option<u32>,
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

    record_runtime_compilation_errors(conn, artifact_id, failed_to_compile, job_id).await;
    Ok(suite)
}

async fn record_runtime_compilation_errors(
    connection: &mut dyn Connection,
    artifact_id: &ArtifactId,
    errors: HashMap<String, String>,
    job_id: Option<u32>,
) {
    let artifact_row_number = connection.artifact_id(artifact_id).await;
    for (krate, error) in errors {
        connection
            .record_error(artifact_row_number, &krate, &error, job_id)
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
    let mut builder = CollectorStepBuilder::new(shared.job_id);
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
async fn run_benchmarks(
    connection: &mut dyn Connection,
    shared: SharedBenchmarkConfig,
    compile: Option<CompileBenchmarkConfig>,
    runtime: Option<RuntimeBenchmarkConfig>,
) -> anyhow::Result<()> {
    record_toolchain_sizes(connection, &shared.artifact_id, &shared.toolchain).await;

    let collector = init_collection(connection, &shared, compile.as_ref(), runtime.as_ref()).await;

    // Compile benchmarks
    let compile_result = if let Some(compile) = compile {
        let errors = bench_compile(connection, &shared, compile, &collector).await;
        errors
            .fail_if_nonzero()
            .context("Compile benchmarks failed")
    } else {
        Ok(())
    };

    // Runtime benchmarks
    let runtime_result = if let Some(runtime) = runtime {
        bench_runtime(
            connection,
            runtime.runtime_suite,
            &collector,
            runtime.filter,
            runtime.iterations,
            runtime.target,
        )
        .await
        .context("Runtime benchmarks failed")
    } else {
        Ok(())
    };

    compile_result.and(runtime_result)
}

/// Perform benchmarks on a published artifact.
async fn bench_published_artifact(
    mut connection: Box<dyn Connection>,
    toolchain: Toolchain,
    dirs: &BenchmarkDirs<'_>,
    job_id: Option<u32>,
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
    let mut compile_benchmarks = get_compile_benchmarks(dirs.compile, CompileBenchmarkFilter::All)?;
    compile_benchmarks.retain(|b| b.category().is_stable());

    let runtime_suite = load_runtime_benchmarks(
        connection.as_mut(),
        dirs.runtime,
        CargoIsolationMode::Isolated,
        None,
        &toolchain,
        &artifact_id,
        job_id,
    )
    .await?;

    let shared = SharedBenchmarkConfig {
        artifact_id,
        toolchain,
        job_id: None,
    };
    run_benchmarks(
        connection.as_mut(),
        shared,
        Some(CompileBenchmarkConfig {
            benchmarks: compile_benchmarks,
            profiles,
            scenarios,
            backends: vec![CodegenBackend::Llvm],
            iterations: Some(3),
            self_profile_storage: None,
            bench_rustc: false,
            targets: vec![Target::default()],
        }),
        Some(RuntimeBenchmarkConfig::new(
            runtime_suite,
            RuntimeBenchmarkFilter::keep_all(),
            DEFAULT_RUNTIME_ITERATIONS,
            Target::default(),
        )),
    )
    .await
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
async fn bench_compile(
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

    #[allow(clippy::too_many_arguments)]
    async fn measure_and_record<F: AsyncFn(&mut BenchProcessor) -> anyhow::Result<()>>(
        collector: &CollectorCtx,
        shared: &SharedBenchmarkConfig,
        config: &CompileBenchmarkConfig,
        errors: &mut BenchmarkErrors,
        conn: &mut dyn Connection,
        benchmark_name: &BenchmarkName,
        category: Category,
        print_intro: &dyn Fn(),
        measure: F,
    ) {
        collector.start_compile_step(conn, benchmark_name).await;

        let mut tx = conn.transaction().await;
        let (supports_stable, category) = category.db_representation();
        tx.conn()
            .record_compile_benchmark(&benchmark_name.0, Some(supports_stable), category)
            .await;
        print_intro();
        let mut processor = BenchProcessor::new(
            tx.conn(),
            benchmark_name,
            &shared.artifact_id,
            collector,
            config.self_profile_storage.as_deref(),
        );
        let result = measure(&mut processor).await;
        if let Err(s) = result {
            eprintln!("collector error: Failed to benchmark '{benchmark_name}', recorded: {s:#}");
            errors.incr();
            tx.conn()
                .record_error(
                    collector.artifact_row_id,
                    &benchmark_name.0,
                    &format!("{s:?}"),
                    shared.job_id,
                )
                .await;
        };
        collector.end_compile_step(tx.conn(), benchmark_name).await;
        tx.commit().await.expect("committed");
    }

    // Normal benchmarks.
    for (nth_benchmark, benchmark) in config.benchmarks.iter().enumerate() {
        measure_and_record(
            collector,
            shared,
            &config,
            &mut errors,
            conn,
            &benchmark.name,
            benchmark.category(),
            &|| {
                eprintln!(
                    "{}",
                    n_normal_benchmarks_remaining(config.benchmarks.len() - nth_benchmark)
                )
            },
            async |processor| {
                with_timeout(benchmark.measure(
                    processor,
                    &config.profiles,
                    &config.scenarios,
                    &config.backends,
                    &shared.toolchain,
                    config.iterations,
                    &config.targets,
                    &collector.measured_compile_test_cases,
                ))
                .await
                .with_context(|| anyhow::anyhow!("Cannot compile {}", benchmark.name))
            },
        )
        .await;
    }

    // The special rustc benchmark, if requested.
    if bench_rustc {
        measure_and_record(
            collector,
            shared,
            &config,
            &mut errors,
            conn,
            &BenchmarkName("rustc".to_string()),
            Category::Primary,
            &|| eprintln!("Special benchmark commencing (due to `--bench-rustc`)"),
            async |processor| {
                with_timeout(processor.measure_rustc(&shared.toolchain))
                    .await
                    .context("measure rustc")
            },
        )
        .await;
    }

    let end = start.elapsed();

    eprintln!(
        "collection took {:?} with {} failed benchmarks",
        end, errors.0
    );

    // This ensures that we're good to go with the just updated data.
    conn.maybe_create_indices().await;
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
            .next_back()
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
