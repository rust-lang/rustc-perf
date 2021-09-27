#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;

use anyhow::{bail, Context};
use database::{ArtifactId, Commit};
use log::debug;
use std::collections::HashSet;
use std::fs;
use std::io::{stderr, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::process::{Command, Stdio};
use std::{str, time::Instant};
use tokio::runtime::Runtime;

mod execute;
mod sysroot;

use execute::{Benchmark, Profiler};
use sysroot::Sysroot;

#[derive(Debug, Copy, Clone)]
pub struct Compiler<'a> {
    pub rustc: &'a Path,
    pub rustdoc: Option<&'a Path>,
    pub cargo: &'a Path,
    pub triple: &'a str,
    pub is_nightly: bool,
}

impl<'a> Compiler<'a> {
    fn from_sysroot(sysroot: &'a Sysroot) -> Compiler<'a> {
        Compiler {
            rustc: &sysroot.rustc,
            rustdoc: Some(&sysroot.rustdoc),
            cargo: &sysroot.cargo,
            triple: &sysroot.triple,
            is_nightly: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BuildKind {
    Check,
    Debug,
    Doc,
    Opt,
}

impl BuildKind {
    fn all() -> Vec<Self> {
        vec![
            BuildKind::Check,
            BuildKind::Debug,
            BuildKind::Doc,
            BuildKind::Opt,
        ]
    }

    fn default() -> Vec<Self> {
        // Don't run rustdoc by default.
        vec![BuildKind::Check, BuildKind::Debug, BuildKind::Opt]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScenarioKind {
    Full,
    IncrFull,
    IncrUnchanged,
    IncrPatched,
}

impl ScenarioKind {
    fn all() -> Vec<ScenarioKind> {
        vec![
            ScenarioKind::Full,
            ScenarioKind::IncrFull,
            ScenarioKind::IncrUnchanged,
            ScenarioKind::IncrPatched,
        ]
    }

    fn all_non_incr() -> Vec<ScenarioKind> {
        vec![ScenarioKind::Full]
    }

    fn default() -> Vec<ScenarioKind> {
        Self::all()
    }
}

// How the --builds arg maps to BuildKinds.
const STRINGS_AND_BUILD_KINDS: &[(&str, BuildKind)] = &[
    ("Check", BuildKind::Check),
    ("Debug", BuildKind::Debug),
    ("Doc", BuildKind::Doc),
    ("Opt", BuildKind::Opt),
];

// How the --runs arg maps to ScenarioKinds.
const STRINGS_AND_SCENARIO_KINDS: &[(&str, ScenarioKind)] = &[
    ("Full", ScenarioKind::Full),
    ("IncrFull", ScenarioKind::IncrFull),
    ("IncrUnchanged", ScenarioKind::IncrUnchanged),
    ("IncrPatched", ScenarioKind::IncrPatched),
];

fn build_kinds_from_arg(arg: &Option<&str>) -> anyhow::Result<Vec<BuildKind>> {
    if let Some(arg) = arg {
        kinds_from_arg("build", STRINGS_AND_BUILD_KINDS, arg)
    } else {
        Ok(BuildKind::default())
    }
}

fn scenario_kinds_from_arg(arg: Option<&str>) -> anyhow::Result<Vec<ScenarioKind>> {
    if let Some(arg) = arg {
        kinds_from_arg("run", STRINGS_AND_SCENARIO_KINDS, arg)
    } else {
        Ok(ScenarioKind::default())
    }
}

fn iterations_from_arg(arg: Option<&str>) -> anyhow::Result<usize> {
    if let Some(arg) = arg {
        if let Ok(iterations) = usize::from_str_radix(arg, 10) {
            Ok(iterations)
        } else {
            anyhow::bail!(
                "cannot parse iteration count '{}'. Must be a decimal number.",
                arg
            );
        }
    } else {
        Ok(1)
    }
}

// Converts a comma-separated list of kind names to a vector of kinds with no
// duplicates.
fn kinds_from_arg<K>(
    name: &str,
    strings_and_kinds: &[(&str, K)],
    arg: &str,
) -> anyhow::Result<Vec<K>>
where
    K: Copy + Eq + ::std::hash::Hash,
{
    let mut kind_set = HashSet::new();

    for s in arg.split(',') {
        if let Some((_s, k)) = strings_and_kinds.iter().find(|(str, _k)| s == *str) {
            kind_set.insert(k);
        } else if s == "All" {
            for (_, k) in strings_and_kinds.iter() {
                kind_set.insert(k);
            }
        } else {
            anyhow::bail!("'{}' is not a known {} kind", s, name);
        }
    }

    // Nb: the element order of `v` must match that of `strings_and_kinds`.
    let mut v = vec![];
    for (_s, k) in strings_and_kinds.iter() {
        if kind_set.contains(k) {
            v.push(*k);
        }
    }
    Ok(v)
}

fn n_benchmarks_remaining(n: usize) -> String {
    let suffix = if n == 1 { "" } else { "s" };
    format!("{} benchmark{} remaining", n, suffix)
}

struct BenchmarkErrors(usize);

impl BenchmarkErrors {
    fn new() -> BenchmarkErrors {
        BenchmarkErrors(0)
    }

    fn incr(&mut self) {
        self.0 += 1;
    }

    fn fail_if_nonzero(self) -> anyhow::Result<()> {
        if self.0 > 0 {
            anyhow::bail!("{} benchmarks failed", self.0)
        }
        Ok(())
    }
}

fn bench(
    rt: &mut Runtime,
    pool: database::Pool,
    artifact_id: &ArtifactId,
    build_kinds: &[BuildKind],
    scenario_kinds: &[ScenarioKind],
    compiler: Compiler<'_>,
    benchmarks: &[Benchmark],
    iterations: Option<usize>,
    is_self_profile: bool,
) -> BenchmarkErrors {
    let mut conn = rt.block_on(pool.connection());
    let mut errors = BenchmarkErrors::new();
    eprintln!(
        "Benchmarking {} for triple {}",
        artifact_id, compiler.triple
    );

    let has_measureme = Command::new("summarize").output().is_ok();
    if is_self_profile {
        assert!(
            has_measureme,
            "needs `summarize` in PATH for self profile.\n\
             Omit --self-profile` to opt out"
        );
    }

    let steps = benchmarks
        .iter()
        .map(|b| b.name.to_string())
        .collect::<Vec<_>>();

    // Make sure there is no observable time when the artifact ID is available
    // but the in-progress steps are not.
    let artifact_row_id = {
        let mut tx = rt.block_on(conn.transaction());
        let artifact_row_id = rt.block_on(tx.conn().artifact_id(&artifact_id));
        rt.block_on(tx.conn().collector_start(artifact_row_id, &steps));

        rt.block_on(tx.commit()).unwrap();
        artifact_row_id
    };

    let start = Instant::now();
    let mut skipped = false;
    for (nth_benchmark, benchmark) in benchmarks.iter().enumerate() {
        let is_fresh =
            rt.block_on(conn.collector_start_step(artifact_row_id, &benchmark.name.to_string()));
        if !is_fresh {
            skipped = true;
            eprintln!("skipping {} -- already benchmarked", benchmark.name);
            continue;
        }
        let mut tx = rt.block_on(conn.transaction());
        rt.block_on(
            tx.conn()
                .record_benchmark(benchmark.name.0.as_str(), Some(benchmark.supports_stable())),
        );
        eprintln!(
            "{}",
            n_benchmarks_remaining(benchmarks.len() - nth_benchmark)
        );

        let mut processor = execute::MeasureProcessor::new(
            rt,
            tx.conn(),
            &benchmark.name,
            &artifact_id,
            artifact_row_id,
            is_self_profile,
        );
        let result = benchmark.measure(
            &mut processor,
            build_kinds,
            scenario_kinds,
            compiler,
            iterations,
        );
        if let Err(s) = result {
            eprintln!(
                "collector error: Failed to benchmark '{}', recorded: {:#}",
                benchmark.name, s
            );
            errors.incr();
            rt.block_on(tx.conn().record_error(
                artifact_row_id,
                benchmark.name.0.as_str(),
                &format!("{:?}", s),
            ));
        };
        rt.block_on(
            tx.conn()
                .collector_end_step(artifact_row_id, &benchmark.name.to_string()),
        );
        rt.block_on(tx.commit()).expect("committed");
    }
    let end = start.elapsed();

    eprintln!(
        "collection took {:?} with {} failed benchmarks",
        end, errors.0
    );

    if skipped {
        log::info!("skipping duration record -- skipped parts of run");
    } else {
        rt.block_on(conn.record_duration(artifact_row_id, end));
    }

    rt.block_on(async move {
        // This ensures that we're good to go with the just updated data.
        conn.maybe_create_indices().await;
    });
    errors
}

fn get_benchmarks(
    benchmark_dir: &Path,
    include: Option<&str>,
    exclude: Option<&str>,
) -> anyhow::Result<Vec<Benchmark>> {
    let mut benchmarks = Vec::new();

    let mut paths = Vec::new();
    for entry in fs::read_dir(benchmark_dir)
        .with_context(|| format!("failed to list benchmark dir '{}'", benchmark_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e),
        };

        if path.ends_with(".git")
            || path.ends_with("scripts")
            || !entry.file_type()?.is_dir()
            || path.ends_with("native-tls-0.1.5")
            || path.ends_with("native-tls-0.2.3")
            || path.ends_with("rust-mozjs")
        {
            debug!("benchmark {} - ignored", name);
            continue;
        }

        paths.push((path, name));
    }
    paths.push((PathBuf::from("rustc"), String::from("rustc")));

    'outer: for (path, name) in paths {
        if let Some(include) = include {
            if !include
                .split(',')
                .any(|to_include| name.contains(to_include))
            {
                debug!(
                    "benchmark {} - doesn't match --include argument, skipping",
                    name
                );
                continue 'outer;
            }
        }

        if let Some(exclude) = exclude {
            for exc in exclude.split(',') {
                if name.contains(exc) {
                    debug!("benchmark {} - matches --exclude argument, skipping", name);
                    continue 'outer;
                }
            }
        }

        debug!("benchmark `{}`- registered", name);
        benchmarks.push(Benchmark::new(name, path)?);
    }
    benchmarks.sort_by_key(|benchmark| benchmark.name.clone());

    if benchmarks.is_empty() {
        eprintln!("Warning: no benchmarks selected! Try less strict filters.");
    }

    Ok(benchmarks)
}

/// Get a toolchain from the input.
/// - `rustc`: check if the given one is acceptable.
/// - `rustdoc`: if one is given, check if it is acceptable. Otherwise, if
///   `Doc` builds are requested, look for one next to the given `rustc`.
/// - `cargo`: if one is given, check if it is acceptable. Otherwise, look
///   for the nightly Cargo via `rustup`.
fn get_local_toolchain(
    build_kinds: &[BuildKind],
    rustc: &str,
    rustdoc: Option<&str>,
    cargo: Option<&str>,
) -> anyhow::Result<(PathBuf, Option<PathBuf>, PathBuf)> {
    // + prefixed rustc is an indicator to fetch the rustc of the toolchain
    // specified. This follows the similar pattern used by rustup's binaries
    // (e.g., `rustc +stage1`).
    let rustc = if let Some(toolchain) = rustc.strip_prefix('+') {
        let output = Command::new("rustup")
            .args(&["which", "rustc", "--toolchain", &toolchain])
            .output()
            .context("failed to run `rustup which rustc`")?;

        // Looks like a commit hash? Try to install it...
        if output.status.code() == Some(101) && toolchain.len() == 40 {
            // No such toolchain exists, so let's try to install it with
            // rustup-toolchain-install-master.

            if !Command::new("rustup-toolchain-install-master")
                .arg(&toolchain)
                .status()
                .context("failed to run `rustup which rustc`")?
                .success()
            {
                anyhow::bail!(
                    "commit-like toolchain {} did not install successfully",
                    toolchain
                )
            }
        }

        let output = Command::new("rustup")
            .args(&["which", "rustc", "--toolchain", &toolchain])
            .output()
            .context("failed to run `rustup which rustc`")?;

        if !output.status.success() {
            anyhow::bail!("did not manage to obtain toolchain {}", toolchain);
        }

        let s = String::from_utf8(output.stdout)
            .context("failed to convert `rustup which rustc` output to utf8")?;

        let rustc = PathBuf::from(s.trim());
        debug!("found rustc: {:?}", &rustc);
        rustc
    } else {
        PathBuf::from(rustc)
            .canonicalize()
            .with_context(|| format!("failed to canonicalize rustc executable '{}'", rustc))?
    };

    let rustdoc =
        if let Some(rustdoc) = rustdoc {
            Some(PathBuf::from(rustdoc).canonicalize().with_context(|| {
                format!("failed to canonicalize rustdoc executable '{}'", rustdoc)
            })?)
        } else if build_kinds.contains(&BuildKind::Doc) {
            // We need a `rustdoc`. Look for one next to `rustc`.
            if let Ok(rustdoc) = rustc.with_file_name("rustdoc").canonicalize() {
                debug!("found rustdoc: {:?}", &rustdoc);
                Some(rustdoc)
            } else {
                anyhow::bail!(
                    "'Doc' build specified but '--rustdoc' not specified and no 'rustdoc' found \
                    next to 'rustc'"
                );
            }
        } else {
            // No `rustdoc` provided, but none needed.
            None
        };

    let cargo = if let Some(cargo) = cargo {
        PathBuf::from(cargo)
            .canonicalize()
            .with_context(|| format!("failed to canonicalize cargo executable '{}'", cargo))?
    } else {
        // Use the nightly cargo from `rustup`.
        let s = String::from_utf8(
            Command::new("rustup")
                .args(&["which", "cargo", "--toolchain=nightly"])
                .output()
                .context("failed to run `rustup which cargo`")?
                .stdout,
        )
        .context("failed to convert `rustup which cargo` output to utf8")?;

        let cargo = PathBuf::from(s.trim());
        debug!("found cargo: {:?}", &cargo);
        cargo
    };

    Ok((rustc, rustdoc, cargo))
}

fn generate_cachegrind_diffs(
    id1: &str,
    id2: &str,
    out_dir: &Path,
    benchmarks: &[Benchmark],
    build_kinds: &[BuildKind],
    scenario_kinds: &[ScenarioKind],
    errors: &mut BenchmarkErrors,
) {
    for benchmark in benchmarks {
        for &build_kind in build_kinds {
            for &scenario_kind in scenario_kinds {
                if let ScenarioKind::IncrPatched = scenario_kind {
                    continue;
                }
                let filename = |prefix, id| {
                    format!(
                        "{}-{}-{}-{:?}-{:?}",
                        prefix, id, benchmark.name, build_kind, scenario_kind
                    )
                };
                let id_diff = format!("{}-{}", id1, id2);
                let cgout1 = out_dir.join(filename("cgout", id1));
                let cgout2 = out_dir.join(filename("cgout", id2));
                let cgdiff = out_dir.join(filename("cgdiff", &id_diff));
                let cgann = out_dir.join(filename("cgann", &id_diff));

                if let Err(e) = cg_diff(&cgout1, &cgout2, &cgdiff) {
                    errors.incr();
                    eprintln!("collector error: {:?}", e);
                    continue;
                }
                if let Err(e) = cg_annotate(&cgdiff, &cgann) {
                    errors.incr();
                    eprintln!("collector error: {:?}", e);
                    continue;
                }
            }
        }
    }
}

/// Compares two Cachegrind output files using cg_diff and writes result to path.
fn cg_diff(cgout1: &Path, cgout2: &Path, path: &Path) -> anyhow::Result<()> {
    let output = Command::new("cg_diff")
        .arg("--mod-filename=s#/rustc/[^/]*/##")
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

fn profile(
    compiler: Compiler,
    id: &str,
    profiler: Profiler,
    out_dir: &Path,
    benchmarks: &[Benchmark],
    build_kinds: &[BuildKind],
    scenario_kinds: &[ScenarioKind],
    errors: &mut BenchmarkErrors,
) {
    eprintln!("Profiling {} with {:?}", id, profiler);
    for (i, benchmark) in benchmarks.iter().enumerate() {
        eprintln!("{}", n_benchmarks_remaining(benchmarks.len() - i));
        let mut processor = execute::ProfileProcessor::new(profiler, out_dir, id);
        let result = benchmark.measure(
            &mut processor,
            &build_kinds,
            &scenario_kinds,
            compiler,
            Some(1),
        );
        if let Err(ref s) = result {
            errors.incr();
            eprintln!(
                "collector error: Failed to profile '{}' with {:?}, recorded: {:?}",
                benchmark.name, profiler, s
            );
        }
    }
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

fn main_result() -> anyhow::Result<i32> {
    env_logger::init();

    let matches = clap_app!(rustc_perf_collector =>
        (version: "0.1")
        (author: "The Rust Compiler Team")
        (about: "Collects Rust performance data")

         // For each subcommand we list the mandatory arguments in the required
         // order, followed by the options in alphabetical order.

        (@subcommand bench_local =>
            (about: "Benchmarks a local rustc")

            // Mandatory arguments
            (@arg RUSTC: +required +takes_value "The path to the local rustc to benchmark")
            (@arg ID:    +required +takes_value "Identifier to associate benchmark results with")

            // Options
            (@arg BUILDS:  --builds  +takes_value
             "One or more (comma-separated) of: 'Check', 'Debug',\n\
             'Doc', 'Opt', 'All'")
            (@arg CARGO:   --cargo   +takes_value "The path to the local Cargo to use")
            (@arg DB:      --db      +takes_value "Database output file")
            (@arg EXCLUDE: --exclude     +takes_value
             "Exclude all benchmarks matching anything in\n\
             this comma-separated list of patterns")
            (@arg INCLUDE: --include     +takes_value
             "Include only benchmarks matching something in\n\
             this comma-separated list of patterns")
            (@arg RUNS:    --runs    +takes_value
             "One or more (comma-separated) of: 'Full',\n\
             'IncrFull', 'IncrUnchanged', 'IncrPatched', 'All'")
            (@arg ITERATIONS: --iterations    +takes_value
                "The number of iterations to do for each benchmark")
            (@arg RUSTDOC: --rustdoc +takes_value "The path to the local rustdoc to benchmark")
            (@arg SELF_PROFILE: --("self-profile") "Collect self-profile data")
        )

        (@subcommand bench_next =>
            (about: "Benchmarks the next commit for perf.rust-lang.org")

            // Mandatory arguments
            (@arg SITE_URL: +required +takes_value "Site URL")

            // Options
            (@arg DB:           --db  +takes_value "Database output file")
            (@arg SELF_PROFILE: --("self-profile") "Collect self-profile data")
        )

        (@subcommand bench_published =>
            (about: "Benchmarks a published toolchain for perf.rust-lang.org's dashboard")

            // Mandatory arguments
            (@arg TOOLCHAIN: +required +takes_value "Toolchain (e.g. stable, beta, 1.26.0)")

            // Options
            (@arg DB: --db +takes_value "Database output file")
        )

        (@subcommand profile_local =>
            (about: "Profiles a local rustc with one of several profilers")

            // Mandatory arguments
            (@arg PROFILER: +required +takes_value
             "One of: 'self-profile', 'time-passes', 'perf-record',\n\
             'oprofile', 'cachegrind', 'callgrind', 'dhat', 'massif',\n\
             'eprintln', 'llvm-lines', 'mono-items'")
            (@arg RUSTC:    +required +takes_value "The path to the local rustc to benchmark")
            (@arg ID:       +required +takes_value "Identifier to associate benchmark results with")

             // Options
            (@arg BUILDS: --builds       +takes_value
             "One or more (comma-separated) of: 'Check', \n\
             'Debug', 'Doc', 'Opt', 'All'")
            (@arg CARGO:   --cargo       +takes_value "The path to the local Cargo to use")
            (@arg EXCLUDE: --exclude     +takes_value
             "Exclude all benchmarks matching anything in\n\
             this comma-separated list of patterns")
            (@arg INCLUDE: --include     +takes_value
             "Include only benchmarks matching something in\n\
             this comma-separated list of patterns")
            (@arg OUT_DIR: --("out-dir") +takes_value "Output directory")
            (@arg RUNS:    --runs        +takes_value
             "One or more (comma-separated) of: 'Full',\n\
             'IncrFull', 'IncrUnchanged', 'IncrPatched', 'All'")
            (@arg RUSTDOC: --rustdoc +takes_value "The path to the local rustdoc to benchmark")
        )

        (@subcommand diff_local =>
            (about: "Profiles and compares two toolchains with one of several profilers")

            // Mandatory arguments
            (@arg PROFILER: +required +takes_value
             "One of: 'self-profile', 'time-passes', 'perf-record',\n\
             'oprofile', 'cachegrind', 'callgrind', 'dhat', 'massif',\n\
             'eprintln', 'llvm-lines', 'mono-items'")
            (@arg RUSTC_BEFORE: +required +takes_value "The path to the local rustc to benchmark")
            (@arg RUSTC_AFTER:  +required +takes_value "The path to the local rustc to benchmark")

             // Options
            (@arg BUILDS: --builds       +takes_value
             "One or more (comma-separated) of: 'Check', \n\
             'Debug', 'Doc', 'Opt', 'All'")
            (@arg CARGO:   --cargo       +takes_value "The path to the local Cargo to use")
            (@arg EXCLUDE: --exclude     +takes_value
             "Exclude all benchmarks matching anything in\n\
             this comma-separated list of patterns")
            (@arg INCLUDE: --include     +takes_value
             "Include only benchmarks matching something in\n\
             this comma-separated list of patterns")
            (@arg OUT_DIR: --("out-dir") +takes_value "Output directory")
            (@arg RUNS:    --runs        +takes_value
             "One or more (comma-separated) of: 'Full',\n\
             'IncrFull', 'IncrUnchanged', 'IncrPatched', 'All'")
            (@arg RUSTDOC: --rustdoc +takes_value "The path to the local rustdoc to benchmark")
        )

        (@subcommand install_next =>
            (about: "Installs the next commit for perf.rust-lang.org")

            // Mandatory arguments: (none)

            // Options: (none)
        )
    )
    .get_matches();

    let benchmark_dir = PathBuf::from("collector/benchmarks");

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    // We want to minimize noise from the runtime
    builder
        .worker_threads(1)
        .max_blocking_threads(1)
        .enable_io();
    let mut rt = builder.build().expect("built runtime");

    let default_db = "results.db";
    let default_out_dir = std::ffi::OsStr::new("results");

    // XXX: This doesn't necessarily work for all archs
    let target_triple = format!("{}-unknown-linux-gnu", std::env::consts::ARCH);

    let ret = match matches.subcommand() {
        ("bench_local", Some(sub_m)) => {
            // Mandatory arguments
            let rustc = sub_m.value_of("RUSTC").unwrap();
            let id = sub_m.value_of("ID").unwrap();

            // Options
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let cargo = sub_m.value_of("CARGO");
            let db = sub_m.value_of("DB").unwrap_or(default_db);
            let exclude = sub_m.value_of("EXCLUDE");
            let include = sub_m.value_of("INCLUDE");
            let scenario_kinds = scenario_kinds_from_arg(sub_m.value_of("RUNS"))?;
            let iterations = iterations_from_arg(sub_m.value_of("ITERATIONS"))?;
            let rustdoc = sub_m.value_of("RUSTDOC");
            let is_self_profile = sub_m.is_present("SELF_PROFILE");

            let pool = database::Pool::open(db);

            let (rustc, rustdoc, cargo) = get_local_toolchain(&build_kinds, rustc, rustdoc, cargo)?;

            let benchmarks = get_benchmarks(&benchmark_dir, include, exclude)?;

            let res = bench(
                &mut rt,
                pool,
                &ArtifactId::Tag(id.to_string()),
                &build_kinds,
                &scenario_kinds,
                Compiler {
                    rustc: &rustc,
                    rustdoc: rustdoc.as_deref(),
                    cargo: &cargo,
                    triple: &target_triple,
                    is_nightly: true,
                },
                &benchmarks,
                Some(iterations),
                is_self_profile,
            );
            res.fail_if_nonzero()?;
            Ok(0)
        }

        ("bench_next", Some(sub_m)) => {
            // Mandatory arguments
            let site_url = sub_m.value_of("SITE_URL").unwrap();

            // Options
            let db = sub_m.value_of("DB").unwrap_or(default_db);
            let is_self_profile = sub_m.is_present("SELF_PROFILE");

            println!("processing commits");
            let client = reqwest::blocking::Client::new();
            let response: collector::api::next_commit::Response = client
                .get(&format!("{}/perf/next_commit", site_url))
                .send()?
                .json()?;
            let next = if let Some(c) = response.commit {
                c
            } else {
                println!("no commit to benchmark");
                // no missing commits
                return Ok(0);
            };
            let commit = get_commit_or_fake_it(&next.sha)?;

            let pool = database::Pool::open(db);

            let sysroot = Sysroot::install(commit.sha.to_string(), &target_triple)
                .with_context(|| format!("failed to install sysroot for {:?}", commit))?;

            let benchmarks = get_benchmarks(
                &benchmark_dir,
                next.include.as_deref(),
                next.exclude.as_deref(),
            )?;

            let res = bench(
                &mut rt,
                pool,
                &ArtifactId::Commit(commit),
                &BuildKind::all(),
                &ScenarioKind::all(),
                Compiler::from_sysroot(&sysroot),
                &benchmarks,
                next.runs.map(|v| v as usize),
                is_self_profile,
            );

            client.post(&format!("{}/perf/onpush", site_url)).send()?;

            res.fail_if_nonzero()?;
            Ok(0)
        }

        ("bench_published", Some(sub_m)) => {
            // Mandatory arguments
            let toolchain = sub_m.value_of("TOOLCHAIN").unwrap();

            // Options
            let db = sub_m.value_of("DB").unwrap_or(default_db);

            let status = Command::new("rustup")
                .args(&["install", "--profile=minimal", &toolchain])
                .status()
                .context("rustup install")?;
            if !status.success() {
                anyhow::bail!("failed to install toolchain for {}", toolchain);
            }

            let pool = database::Pool::open(db);

            let scenario_kinds = if collector::version_supports_incremental(toolchain) {
                ScenarioKind::all()
            } else {
                ScenarioKind::all_non_incr()
            };
            let build_kinds = if collector::version_supports_doc(toolchain) {
                BuildKind::all()
            } else {
                let mut all = BuildKind::all();
                let doc = all.iter().position(|bk| *bk == BuildKind::Doc).unwrap();
                all.remove(doc);
                all
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
            let mut benchmarks = get_benchmarks(&benchmark_dir, None, None)?;
            benchmarks.retain(|b| b.supports_stable());

            let res = bench(
                &mut rt,
                pool,
                &ArtifactId::Tag(toolchain.to_string()),
                &build_kinds,
                &scenario_kinds,
                Compiler {
                    rustc: Path::new(rustc.trim()),
                    rustdoc: Some(Path::new(rustdoc.trim())),
                    cargo: Path::new(cargo.trim()),
                    is_nightly: false,
                    triple: &target_triple,
                },
                &benchmarks,
                Some(3),
                /* is_self_profile */ false,
            );
            res.fail_if_nonzero()?;
            Ok(0)
        }

        ("profile_local", Some(sub_m)) => {
            // Mandatory arguments
            let profiler = Profiler::from_name(sub_m.value_of("PROFILER").unwrap())?;
            let rustc = sub_m.value_of("RUSTC").unwrap();
            let id = sub_m.value_of("ID").unwrap();

            // Options
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let cargo = sub_m.value_of("CARGO");
            let exclude = sub_m.value_of("EXCLUDE");
            let include = sub_m.value_of("INCLUDE");
            let out_dir = PathBuf::from(sub_m.value_of_os("OUT_DIR").unwrap_or(default_out_dir));
            let scenario_kinds = scenario_kinds_from_arg(sub_m.value_of("RUNS"))?;
            let rustdoc = sub_m.value_of("RUSTDOC");

            let (rustc, rustdoc, cargo) = get_local_toolchain(&build_kinds, rustc, rustdoc, cargo)?;
            let compiler = Compiler {
                rustc: &rustc,
                rustdoc: rustdoc.as_deref(),
                cargo: &cargo,
                triple: &target_triple,
                is_nightly: true,
            };
            let benchmarks = get_benchmarks(&benchmark_dir, include, exclude)?;
            let mut errors = BenchmarkErrors::new();
            profile(
                compiler,
                id,
                profiler,
                &out_dir,
                &benchmarks,
                &build_kinds,
                &scenario_kinds,
                &mut errors,
            );
            errors.fail_if_nonzero()?;
            Ok(0)
        }

        ("diff_local", Some(sub_m)) => {
            // Mandatory arguments
            let profiler = Profiler::from_name(sub_m.value_of("PROFILER").unwrap())?;
            let rustc1 = sub_m.value_of("RUSTC_BEFORE").unwrap();
            let rustc2 = sub_m.value_of("RUSTC_AFTER").unwrap();

            // Options
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let cargo = sub_m.value_of("CARGO");
            let exclude = sub_m.value_of("EXCLUDE");
            let include = sub_m.value_of("INCLUDE");
            let out_dir = PathBuf::from(sub_m.value_of_os("OUT_DIR").unwrap_or(default_out_dir));
            let scenario_kinds = scenario_kinds_from_arg(sub_m.value_of("RUNS"))?;
            let rustdoc = sub_m.value_of("RUSTDOC");

            let id1 = rustc1.strip_prefix('+').unwrap_or("before");
            let id2 = rustc2.strip_prefix('+').unwrap_or("after");
            let mut toolchains = Vec::new();
            for (id, rustc) in [(id1, rustc1), (id2, rustc2)] {
                let (rustc, rustdoc, cargo) =
                    get_local_toolchain(&build_kinds, rustc, rustdoc, cargo)?;
                toolchains.push((id.to_owned(), rustc, rustdoc, cargo));
            }

            let benchmarks = get_benchmarks(&benchmark_dir, include, exclude)?;
            let mut errors = BenchmarkErrors::new();
            for (id, rustc, rustdoc, cargo) in &toolchains {
                let compiler = Compiler {
                    rustc: &rustc,
                    rustdoc: rustdoc.as_deref(),
                    cargo: &cargo,
                    triple: &target_triple,
                    is_nightly: true,
                };
                profile(
                    compiler,
                    id,
                    profiler,
                    &out_dir,
                    &benchmarks,
                    &build_kinds,
                    &scenario_kinds,
                    &mut errors,
                );
            }

            if let Profiler::Cachegrind = profiler {
                generate_cachegrind_diffs(
                    id1,
                    id2,
                    &out_dir,
                    &benchmarks,
                    &build_kinds,
                    &scenario_kinds,
                    &mut errors,
                );
            }

            errors.fail_if_nonzero()?;
            Ok(0)
        }

        ("install_next", Some(_sub_m)) => {
            // Mandatory arguments: (none)

            // Options: (none)

            let last_sha = Command::new("git")
                .arg("ls-remote")
                .arg("https://github.com/rust-lang/rust.git")
                .arg("master")
                .output()
                .unwrap();
            let last_sha = String::from_utf8(last_sha.stdout).expect("utf8");
            let last_sha = last_sha.split_whitespace().next().expect(&last_sha);
            let commit = get_commit_or_fake_it(&last_sha).expect("success");
            let mut sysroot = Sysroot::install(commit.sha.to_string(), &target_triple)?;
            sysroot.preserve(); // don't delete it

            // Print the directory containing the toolchain.
            sysroot.rustc.pop();
            let s = format!("{:?}", sysroot.rustc);
            println!("{}", &s[1..s.len() - 1]);

            Ok(0)
        }

        _ => {
            let _ = writeln!(stderr(), "{}", matches.usage());
            Ok(2)
        }
    };
    ret
}

pub fn get_commit_or_fake_it(sha: &str) -> anyhow::Result<Commit> {
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
        })
        .unwrap_or_else(|| {
            log::warn!("utilizing fake commit!");
            Commit {
                sha: sha.into(),
                date: database::Date::ymd_hms(2000, 01, 01, 0, 0, 0),
            }
        }))
}
