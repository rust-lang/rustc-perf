#![recursion_limit = "1024"]

#[macro_use]
extern crate clap;

use anyhow::{bail, Context};
use collector::api::collected;
use database::{pool::Connection, ArtifactId, Commit};
use log::{debug, error};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{stderr, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use std::str;
use tokio::runtime::Runtime;

mod background_worker;
mod execute;
mod sysroot;

use background_worker::send_home;
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RunKind {
    Full,
    IncrFull,
    IncrUnchanged,
    IncrPatched,
}

impl RunKind {
    fn all() -> Vec<RunKind> {
        vec![
            RunKind::Full,
            RunKind::IncrFull,
            RunKind::IncrUnchanged,
            RunKind::IncrPatched,
        ]
    }

    fn all_non_incr() -> Vec<RunKind> {
        vec![RunKind::Full]
    }
}

#[derive(thiserror::Error, PartialEq, Eq, Debug)]
pub enum KindError {
    #[error("'{:?}' is not a known {} kind", .1, .0)]
    UnknownKind(&'static str, String),
}

// How the --builds arg maps to BuildKinds.
const STRINGS_AND_BUILD_KINDS: &[(&str, BuildKind)] = &[
    ("Check", BuildKind::Check),
    ("Debug", BuildKind::Debug),
    ("Doc", BuildKind::Doc),
    ("Opt", BuildKind::Opt),
];

// How the --runs arg maps to RunKinds.
const STRINGS_AND_RUN_KINDS: &[(&str, RunKind)] = &[
    ("Full", RunKind::Full),
    ("IncrFull", RunKind::IncrFull),
    ("IncrUnchanged", RunKind::IncrUnchanged),
    ("IncrPatched", RunKind::IncrPatched),
];

pub fn build_kinds_from_arg(arg: &Option<&str>) -> Result<Vec<BuildKind>, KindError> {
    if let Some(arg) = arg {
        kinds_from_arg(STRINGS_AND_BUILD_KINDS, arg)
    } else {
        // don't run rustdoc by default
        Ok(vec![BuildKind::Check, BuildKind::Debug, BuildKind::Opt])
    }
}

pub fn run_kinds_from_arg(arg: &Option<&str>) -> Result<Vec<RunKind>, KindError> {
    if let Some(arg) = arg {
        kinds_from_arg(STRINGS_AND_RUN_KINDS, arg)
    } else {
        Ok(RunKind::all())
    }
}

// Converts a comma-separated list of kind names to a vector of kinds with no
// duplicates.
fn kinds_from_arg<K>(strings_and_kinds: &[(&str, K)], arg: &str) -> Result<Vec<K>, KindError>
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
            return Err(KindError::UnknownKind("build", s.to_string()));
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

fn process(
    rt: &mut Runtime,
    pool: &database::Pool,
    benchmarks: &[Benchmark],
    self_profile: bool,
) -> anyhow::Result<()> {
    println!("processing commits");
    let client = reqwest::blocking::Client::new();
    let commit: Option<String> = client
        .get(&format!(
            "{}/perf/next_commit",
            env::var("SITE_URL").expect("SITE_URL defined")
        ))
        .send()?
        .json()?;
    let commit = if let Some(c) = commit {
        c
    } else {
        // no missing commits
        return Ok(());
    };

    let commit = get_commit_or_fake_it(&commit)?;
    match Sysroot::install(commit.sha.to_string(), "x86_64-unknown-linux-gnu") {
        Ok(sysroot) => {
            let conn = rt.block_on(pool.connection());
            bench(
                rt,
                conn,
                &ArtifactId::Commit(commit),
                &BuildKind::all(),
                &RunKind::all(),
                Compiler::from_sysroot(&sysroot),
                &benchmarks,
                3,
                true,
                self_profile,
            );
        }
        Err(err) => {
            error!("failed to install sysroot for {:?}: {:?}", commit, err);
        }
    }

    client
        .post(&format!(
            "{}/perf/onpush",
            env::var("SITE_URL").expect("SITE_URL defined")
        ))
        .send()?;

    Ok(())
}

fn n_benchmarks_remaining(n: usize) -> String {
    let suffix = if n == 1 { "" } else { "s" };
    format!("{} benchmark{} remaining", n, suffix)
}

struct BenchmarkErrors(usize);

impl BenchmarkErrors {
    fn fail_if_error(self) -> anyhow::Result<()> {
        if self.0 > 0 {
            anyhow::bail!("{} benchmarks failed", self.0)
        }
        Ok(())
    }
}

fn bench(
    rt: &mut Runtime,
    mut conn: Box<dyn Connection>,
    cid: &ArtifactId,
    build_kinds: &[BuildKind],
    run_kinds: &[RunKind],
    compiler: Compiler<'_>,
    benchmarks: &[Benchmark],
    iterations: usize,
    call_home: bool,
    self_profile: bool,
) -> BenchmarkErrors {
    if compiler.rustdoc.is_none() && build_kinds.iter().any(|b| *b == BuildKind::Doc) {
        eprintln!("Rustdoc build specified but rustdoc path not provided");
        std::process::exit(1);
    }

    let mut errors_recorded = 0;
    eprintln!("Benchmarking {} for triple {}", cid, compiler.triple);

    if call_home {
        if let ArtifactId::Commit(commit) = cid {
            send_home(collected::Request::BenchmarkCommit {
                commit: commit.clone(),
                benchmarks: benchmarks.iter().map(|b| b.name.to_string()).collect(),
            });
        }
    }

    let has_measureme = Command::new("summarize").output().is_ok();
    if self_profile {
        assert!(
            has_measureme,
            "needs `summarize` in PATH for self profile.\n\
             Omit --self-profile` to opt out"
        );
    }

    let mut tx = rt.block_on(conn.transaction());
    let index = rt.block_on(database::Index::load(tx.conn()));
    let has_collected = match cid {
        ArtifactId::Commit(commit) => index.commits().iter().any(|c| c == commit),
        ArtifactId::Artifact(id) => index.artifacts().any(|a| a == id),
    };
    if has_collected {
        eprintln!("{} has previously been collected, skipping.", cid);
        eprintln!(
            "Note that this behavior is likely to change in the future \
            to collect and append the data instead."
        );
        return BenchmarkErrors(errors_recorded);
    }
    let interned_cid = rt.block_on(tx.conn().artifact_id(&cid));

    for (nth_benchmark, benchmark) in benchmarks.iter().enumerate() {
        rt.block_on(
            tx.conn()
                .record_benchmark(benchmark.name.0.as_str(), benchmark.supports_stable()),
        );
        eprintln!(
            "{}",
            n_benchmarks_remaining(benchmarks.len() - nth_benchmark)
        );

        let mut processor = execute::MeasureProcessor::new(
            rt,
            tx.conn(),
            &benchmark.name,
            interned_cid,
            self_profile,
        );
        let result =
            benchmark.measure(&mut processor, build_kinds, run_kinds, compiler, iterations);
        if let Err(s) = result {
            eprintln!("Failed to benchmark {}, recorded: {}", benchmark.name, s);
            errors_recorded += 1;
            rt.block_on(tx.conn().record_error(
                interned_cid,
                benchmark.name.0.as_str(),
                &format!("{:?}", s),
            ));
        };

        if call_home {
            if let ArtifactId::Commit(commit) = cid {
                send_home(collected::Request::BenchmarkDone {
                    benchmark: benchmark.name.to_string(),
                    commit: commit.clone(),
                });
            }
        }
    }

    // Publish results now that we've finished fully with this commit.
    rt.block_on(tx.commit()).unwrap();

    rt.block_on(async move {
        // This ensures that we're good to go with the just updated data.
        conn.maybe_create_indices().await;
    });
    BenchmarkErrors(errors_recorded)
}

fn get_benchmarks(
    benchmark_dir: &Path,
    include: Option<&str>,
    exclude: Option<&str>,
) -> anyhow::Result<Vec<Benchmark>> {
    let mut benchmarks = Vec::new();
    'outer: for entry in fs::read_dir(benchmark_dir)
        .with_context(|| format!("failed to list benchmark dir {}", benchmark_dir.display()))?
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
    Ok(benchmarks)
}

fn main() {
    match main_result() {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("{:#}\n{}", err, err.backtrace());
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

       (@arg include: --include +takes_value "Include benchmarks matching these")
       (@arg exclude: --exclude +takes_value "Exclude benchmarks matching these")
       (@arg db: --("db") +takes_value "Database file")
       (@arg self_profile: --("self-profile") "Collect self-profile")

       (@subcommand bench_local =>
           (about: "Benchmarks a local rustc")
           (@arg RUSTC: --rustc +required +takes_value "The path to the local rustc to benchmark")
           (@arg RUSTDOC: --rustdoc +takes_value "The path to the local rustdoc to benchmark")
           (@arg CARGO: --cargo +required +takes_value "The path to the local Cargo to use")
           (@arg BUILDS: --builds +takes_value
            "One or more (comma-separated) of: 'Check', 'Debug',\n\
            'Doc', 'Opt', 'All'")
           (@arg RUNS: --runs +takes_value
            "One or more (comma-separated) of: 'Full',\n\
            'IncrFull', 'IncrUnchanged', 'IncrPatched', 'All'")
           (@arg ID: +required +takes_value "Identifier to associate benchmark results with")
       )
       (@subcommand bench_published =>
           (about: "Benchmarks a specified toolchain")
           (@arg TOOLCHAIN: +required +takes_value "Toolchain to install (e.g. stable, beta, 1.26.0)")
       )
       (@subcommand bench_test =>
           (about: "Benchmarks the most recent commit for testing purposes")
       )
       (@subcommand process =>
           (about: "Syncs to git and collects performance data for all versions")
       )
       (@subcommand profile =>
           (about: "Profiles a local rustc")
           (@arg output_dir: --("output") +required +takes_value "Output directory")
           (@arg RUSTC: --rustc +required +takes_value "The path to the local rustc to benchmark")
           (@arg RUSTDOC: --rustdoc +takes_value "The path to the local rustdoc to benchmark")
           (@arg CARGO: --cargo +required +takes_value "The path to the local Cargo to use")
           (@arg BUILDS: --builds +takes_value
            "One or more (comma-separated) of: 'Check', 'Debug',\n\
            'Opt', 'All'")
           (@arg RUNS: --runs +takes_value
            "One or more (comma-separated) of: 'Full',\n\
            'IncrFull', 'IncrUnchanged', 'IncrPatched', 'All'")
           (@arg PROFILER: +required +takes_value
            "One of: 'self-profile', 'time-passes', 'perf-record',\n\
            'cachegrind', 'callgrind', ''dhat', 'massif', 'eprintln'")
           (@arg ID: +required +takes_value "Identifier to associate benchmark results with")
       )
    )
    .get_matches();

    let benchmark_dir = PathBuf::from("collector/benchmarks");
    let include = matches.value_of("include");
    let exclude = matches.value_of("exclude");
    let mut benchmarks = get_benchmarks(&benchmark_dir, include, exclude)?;
    let self_profile = matches.is_present("self_profile");

    let mut builder = tokio::runtime::Builder::new();
    // We want to minimize noise from the runtime
    builder
        .core_threads(1)
        .max_threads(1)
        .enable_io()
        .basic_scheduler();
    let mut rt = builder.build().expect("built runtime");

    let pool = matches.value_of("db").map(|db| database::Pool::open(db));

    let ret = match matches.subcommand() {
        ("bench_local", Some(sub_m)) => {
            let rustc = sub_m.value_of("RUSTC").unwrap();
            let rustdoc = sub_m.value_of("RUSTDOC");
            let cargo = sub_m.value_of("CARGO").unwrap();
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let run_kinds = run_kinds_from_arg(&sub_m.value_of("RUNS"))?;
            let id = sub_m.value_of("ID").unwrap();

            let rustc_path = PathBuf::from(rustc).canonicalize()?;
            let rustdoc_path = if let Some(r) = rustdoc {
                Some(PathBuf::from(r).canonicalize()?)
            } else {
                None
            };
            let cargo_path = PathBuf::from(cargo).canonicalize()?;
            let conn = rt.block_on(pool.expect("--db passed").connection());
            bench(
                &mut rt,
                conn,
                &ArtifactId::Artifact(id.to_string()),
                &build_kinds,
                &run_kinds,
                Compiler {
                    rustc: &rustc_path,
                    rustdoc: rustdoc_path.as_deref(),
                    cargo: &cargo_path,
                    triple: "x86_64-unknown-linux-gnu",
                    is_nightly: true,
                },
                &benchmarks,
                1,
                false,
                self_profile,
            );
            Ok(0)
        }

        ("bench_published", Some(sub_m)) => {
            let toolchain = sub_m.value_of("TOOLCHAIN").unwrap();
            let status = Command::new("rustup")
                .args(&["install", "--profile=minimal", &toolchain])
                .status()
                .context("rustup install")?;
            if !status.success() {
                anyhow::bail!("failed to install toolchain for {}", toolchain);
            }

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

            // Remove benchmarks that don't work with a stable compiler.
            benchmarks.retain(|b| b.supports_stable());

            let run_kinds = if collector::version_supports_incremental(toolchain) {
                RunKind::all()
            } else {
                RunKind::all_non_incr()
            };
            let conn = rt.block_on(pool.expect("--db passed").connection());
            bench(
                &mut rt,
                conn,
                &ArtifactId::Artifact(toolchain.to_string()),
                &BuildKind::all(),
                &run_kinds,
                Compiler {
                    rustc: Path::new(rustc.trim()),
                    rustdoc: Some(Path::new(rustdoc.trim())),
                    cargo: Path::new(cargo.trim()),
                    is_nightly: false,
                    triple: "x86_64-unknown-linux-gnu",
                },
                &benchmarks,
                3,
                false,
                false,
            );
            Ok(0)
        }

        ("bench_test", Some(_)) => {
            let last_sha = Command::new("git")
                .arg("ls-remote")
                .arg("https://github.com/rust-lang/rust.git")
                .arg("master")
                .output()
                .unwrap();
            let last_sha = String::from_utf8(last_sha.stdout).expect("utf8");
            let last_sha = last_sha.split_whitespace().next().expect(&last_sha);
            let commit = get_commit_or_fake_it(&last_sha).expect("success");
            let sysroot = Sysroot::install(commit.sha.to_string(), "x86_64-unknown-linux-gnu")?;
            let conn = rt.block_on(pool.expect("--db passed").connection());
            let res = bench(
                &mut rt,
                conn,
                &ArtifactId::Commit(commit),
                &[BuildKind::Check, BuildKind::Doc], // no Debug or Opt builds
                &RunKind::all(),
                Compiler::from_sysroot(&sysroot),
                &benchmarks,
                1,
                false,
                self_profile,
            );
            res.fail_if_error()?;
            Ok(0)
        }

        ("process", Some(_)) => {
            process(
                &mut rt,
                &pool.expect("--db passed"),
                &benchmarks,
                self_profile,
            )?;
            Ok(0)
        }

        ("profile", Some(sub_m)) => {
            let rustc = sub_m.value_of("RUSTC").unwrap();
            let rustdoc = sub_m.value_of("RUSTDOC");
            let cargo = sub_m.value_of("CARGO").unwrap();
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let run_kinds = run_kinds_from_arg(&sub_m.value_of("RUNS"))?;
            let profiler = Profiler::from_name(sub_m.value_of("PROFILER").unwrap())?;
            let id = sub_m.value_of("ID").unwrap();
            let out_dir = PathBuf::from(sub_m.value_of_os("output_dir").unwrap());

            eprintln!("Profiling with {:?}", profiler);

            let rustc_path = PathBuf::from(rustc).canonicalize()?;
            let rustdoc_path = if let Some(r) = rustdoc {
                Some(PathBuf::from(r).canonicalize()?)
            } else {
                None
            };
            let cargo_path = PathBuf::from(cargo).canonicalize()?;
            let compiler = Compiler {
                rustc: &rustc_path,
                rustdoc: rustdoc_path.as_deref(),
                cargo: &cargo_path,
                is_nightly: true,
                triple: "x86_64-unknown-linux-gnu", // XXX: Technically not necessarily true
            };

            for (i, benchmark) in benchmarks.iter().enumerate() {
                eprintln!("{}", n_benchmarks_remaining(benchmarks.len() - i));
                let mut processor = execute::ProfileProcessor::new(profiler, &out_dir, &id);
                let result =
                    benchmark.measure(&mut processor, &build_kinds, &run_kinds, compiler, 1);
                if let Err(ref s) = result {
                    eprintln!(
                        "Failed to profile {} with {:?}, recorded: {:?}",
                        benchmark.name, profiler, s
                    );
                }
            }
            Ok(0)
        }

        _ => {
            let _ = writeln!(stderr(), "{}", matches.usage());
            Ok(2)
        }
    };
    background_worker::shut_down();
    ret
}

pub fn get_commit_or_fake_it(sha: &str) -> anyhow::Result<Commit> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    Ok(rt
        .block_on(rustc_artifacts::master_commits())
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
