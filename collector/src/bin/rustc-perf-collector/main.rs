#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate clap;

use chrono::{Timelike, Utc};
use collector::api::collected;
use collector::git::get_commit_or_fake_it;
use collector::git::get_rust_commits as get_commits;
use collector::{ArtifactData, Commit, CommitData, Date};
use failure::{Error, ResultExt, SyncFailure};
use log::{debug, error, info, warn};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{stderr, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use std::sync::Arc;

mod background_worker;
mod execute;
mod outrepo;
mod sysroot;

use background_worker::send_home;
use execute::{Benchmark, Profiler};
use sysroot::Sysroot;

#[derive(Debug, Copy, Clone)]
pub struct Compiler<'a> {
    pub rustc: &'a Path,
    pub cargo: &'a Path,
    pub triple: &'a str,
    pub is_nightly: bool,
}

impl<'a> Compiler<'a> {
    fn from_sysroot(sysroot: &'a Sysroot) -> Compiler<'a> {
        Compiler {
            rustc: &sysroot.rustc,
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
    Opt,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RunKind {
    Clean,
    BaseIncr,
    CleanIncr,
    PatchedIncrs,
}

impl RunKind {
    fn all() -> Vec<RunKind> {
        vec![
            RunKind::Clean,
            RunKind::BaseIncr,
            RunKind::CleanIncr,
            RunKind::PatchedIncrs,
        ]
    }

    fn all_non_incr() -> Vec<RunKind> {
        vec![RunKind::Clean]
    }
}

#[derive(Fail, PartialEq, Eq, Debug)]
pub enum KindError {
    #[fail(display = "'{:?}' is not a known {} kind", _1, _0)]
    UnknownKind(&'static str, String),
}

// How the --builds arg maps to BuildKinds.
const STRINGS_AND_BUILD_KINDS: &[(&str, BuildKind)] = &[
    ("Check", BuildKind::Check),
    ("Debug", BuildKind::Debug),
    ("Opt", BuildKind::Opt),
];

// How the --runs arg maps to RunKinds.
const STRINGS_AND_RUN_KINDS: &[(&str, RunKind)] = &[
    ("Clean", RunKind::Clean),
    ("BaseIncr", RunKind::BaseIncr),
    ("CleanIncr", RunKind::CleanIncr),
    ("PatchedIncrs", RunKind::PatchedIncrs),
];

pub fn build_kinds_from_arg(arg: &Option<&str>) -> Result<Vec<BuildKind>, KindError> {
    if let Some(arg) = arg {
        kinds_from_arg(STRINGS_AND_BUILD_KINDS, arg)
    } else {
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

fn bench_commit(
    repo: Option<&outrepo::Repo>,
    commit: &Commit,
    build_kinds: &[BuildKind],
    run_kinds: &[RunKind],
    compiler: Compiler<'_>,
    benchmarks: &[Benchmark],
    iterations: usize,
    call_home: bool,
) -> CommitData {
    info!(
        "benchmarking commit {} ({}) for triple {}",
        commit.sha, commit.date, compiler.triple
    );

    if call_home {
        send_home(collected::Request::BenchmarkCommit {
            commit: commit.clone(),
            benchmarks: benchmarks.iter().map(|b| b.name.clone()).collect(),
        });
    }
    let existing_data = repo.and_then(|r| r.load_commit_data(&commit, &compiler.triple).ok());

    let mut results = BTreeMap::new();
    if let Some(ref data) = existing_data {
        for benchmark in benchmarks {
            if let Some(result) = data.benchmarks.get(&benchmark.name) {
                if call_home {
                    send_home(collected::Request::BenchmarkDone {
                        benchmark: benchmark.name.clone(),
                        commit: commit.clone(),
                    });
                }
                results.insert(benchmark.name.clone(), result.clone());
            }
        }
    }

    for benchmark in benchmarks {
        if results.contains_key(&benchmark.name) {
            continue;
        }

        let mut processor = execute::MeasureProcessor::new(&benchmark.name);
        let result =
            benchmark.measure(&mut processor, build_kinds, run_kinds, compiler, iterations);
        let result = match result {
            Ok(()) => Ok(processor.collected),
            Err(ref s) => {
                info!("failed to benchmark {}, recorded: {}", benchmark.name, s);
                Err(format!("{:?}", s))
            }
        };

        if call_home {
            send_home(collected::Request::BenchmarkDone {
                benchmark: benchmark.name.clone(),
                commit: commit.clone(),
            });
        }

        results.insert(benchmark.name.clone(), result);
        info!("{} benchmarks left", benchmarks.len() - results.len());
    }

    CommitData {
        commit: commit.clone(),
        triple: compiler.triple.to_string(),
        benchmarks: results,
    }
}

fn get_benchmarks(
    benchmark_dir: &Path,
    filter: Option<&str>,
    exclude: Option<&str>,
) -> Result<Vec<Benchmark>, Error> {
    let mut benchmarks = Vec::new();
    'outer: for entry in fs::read_dir(benchmark_dir).context("failed to list benchmarks")? {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e),
        };

        if path.ends_with(".git") || path.ends_with("scripts") || !entry.file_type()?.is_dir() {
            debug!("benchmark {} - ignored", name);
            continue;
        }

        if let Some(filter) = filter {
            if !name.contains(filter) {
                debug!(
                    "benchmark {} - doesn't match --filter argument, skipping",
                    name
                );
                continue;
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
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn main_result() -> Result<i32, Error> {
    env_logger::init();

    let matches = clap_app!(rustc_perf_collector =>
       (version: "0.1")
       (author: "The Rust Compiler Team")
       (about: "Collects Rust performance data")

       (@arg filter: --filter +takes_value "Run only benchmarks that contain this")
       (@arg exclude: --exclude +takes_value "Ignore all benchmarks that contain this")
       (@arg sync_git: --("sync-git") "Synchronize repository with remote")
       (@arg output_repo: --("output-repo") +required +takes_value "Output repository/directory")

       (@subcommand bench_commit =>
           (about: "benchmark a bors merge from AWS")
           (@arg COMMIT: +required +takes_value "Commit hash to bench")
       )
       (@subcommand bench_local =>
           (about: "benchmark a local rustc")
           (@arg RUSTC: --rustc +required +takes_value "The path to the local rustc to benchmark")
           (@arg CARGO: --cargo +required +takes_value "The path to the local Cargo to use")
           (@arg BUILDS: --builds +takes_value
            "One or more (comma-separated) of: 'Check', 'Debug',\n\
            'Opt', 'All'")
           (@arg RUNS: --runs +takes_value
            "One or more (comma-separated) of: 'Clean',\n\
            'BaseIncr', 'CleanIncr', 'PatchedIncrs', 'All'")
           (@arg ID: +required +takes_value "Identifier to associate benchmark results with")
       )
       (@subcommand bench_published =>
           (about: "bench an artifact from static.r-l.o")
           (@arg ID: +required +takes_value "id to install (e.g., stable, beta, 1.26.0)")
       )
       (@subcommand process =>
           (about: "syncs to git and collects performance data for all versions")
       )
       (@subcommand profile =>
           (about: "profile a local rustc")
           (@arg RUSTC: --rustc +required +takes_value "The path to the local rustc to benchmark")
           (@arg CARGO: --cargo +required +takes_value "The path to the local Cargo to use")
           (@arg BUILDS: --builds +takes_value
            "One or more (comma-separated) of: 'Check', 'Debug',\n\
            'Opt', 'All'")
           (@arg RUNS: --runs +takes_value
            "One or more (comma-separated) of: 'Clean',\n\
            'BaseIncr', 'CleanIncr', 'PatchedIncrs', 'All'")
           (@arg PROFILER: +required +takes_value
            "One of: 'time-passes', 'perf-record', 'cachegrind',\n\
            'callgrind', 'exp-dhat', 'dhat', 'massif', 'eprintln'")
           (@arg ID: +required +takes_value "Identifier to associate benchmark results with")
       )
       (@subcommand remove_benchmark =>
           (about: "remove data for a benchmark")
           (@arg BENCHMARK: --benchmark +required +takes_value "benchmark name to remove data for")
       )
       (@subcommand remove_errs =>
           (about: "remove errored data")
       )
       (@subcommand test_benchmarks =>
           (about: "test benchmark the most recent commit")
       )
    )
    .get_matches();

    let benchmark_dir = PathBuf::from("collector/benchmarks");
    let filter = matches.value_of("filter");
    let exclude = matches.value_of("exclude");
    let mut benchmarks = get_benchmarks(&benchmark_dir, filter, exclude)?;
    let use_remote = matches.is_present("sync_git");

    let get_out_dir = || {
        let path = PathBuf::from(matches.value_of_os("output_repo").unwrap());
        fs::create_dir_all(&path).unwrap();
        path
    };

    let get_out_repo =
        |allow_new_dir| outrepo::Repo::open(get_out_dir(), allow_new_dir, use_remote);

    let ret = match matches.subcommand() {
        ("bench_commit", Some(sub_m)) => {
            let commit = sub_m.value_of("COMMIT").unwrap();
            let commit = get_commit_or_fake_it(&commit)?;
            let out_repo = get_out_repo(false)?;
            let sysroot = Sysroot::install(&commit.sha, commit.date.0, "x86_64-unknown-linux-gnu")?;
            let build_kinds = &[BuildKind::Check, BuildKind::Debug, BuildKind::Opt];
            let run_kinds = RunKind::all();
            out_repo.success(&bench_commit(
                Some(&out_repo),
                &commit,
                build_kinds,
                &run_kinds,
                Compiler::from_sysroot(&sysroot),
                &benchmarks,
                3,
                false,
            ))?;
            Ok(0)
        }

        ("bench_local", Some(sub_m)) => {
            let rustc = sub_m.value_of("RUSTC").unwrap();
            let cargo = sub_m.value_of("CARGO").unwrap();
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let run_kinds = run_kinds_from_arg(&sub_m.value_of("RUNS"))?;
            let id = sub_m.value_of("ID").unwrap();

            // This isn't a true representation of a commit, because `id` is an
            // arbitrary identifier, not a commit SHA. But that's ok for local
            // runs, because `commit` is only used when producing the output
            // files, not for interacting with a repo.
            let commit = Commit {
                sha: id.to_string(),
                // Drop the nanoseconds; we don't want that level of precision.
                date: Date(Utc::now().with_nanosecond(0).unwrap()),
            };
            let rustc_path = PathBuf::from(rustc).canonicalize()?;
            let cargo_path = PathBuf::from(cargo).canonicalize()?;
            // We don't pass `out_repo` here. `commit` is unique because
            // `commit.date` is unique, so there's no point even trying to load
            // prior data.
            let result = bench_commit(
                None,
                &commit,
                &build_kinds,
                &run_kinds,
                Compiler {
                    rustc: &rustc_path,
                    cargo: &cargo_path,
                    triple: "x86_64-unknown-linux-gnu",
                    is_nightly: true,
                },
                &benchmarks,
                1,
                false,
            );
            get_out_repo(true)?.add_commit_data(&result)?;
            Ok(0)
        }

        ("bench_published", Some(sub_m)) => {
            let id = sub_m.value_of("ID").unwrap();
            let repo = get_out_repo(false)?;
            let commit = Commit {
                sha: String::from("<none>"),
                date: Date::ymd_hms(2010, 01, 01, 0, 0, 0),
            };
            let cfg = rustup::Cfg::from_env(Arc::new(|_| {})).map_err(SyncFailure::new)?;
            let toolchain = rustup::Toolchain::from(&cfg, id)
                .map_err(SyncFailure::new)
                .with_context(|_| format!("creating toolchain for id: {}", id))?;
            toolchain
                .install_from_dist_if_not_installed()
                .map_err(SyncFailure::new)?;

            // Remove benchmarks that don't work with a stable compiler.
            benchmarks.retain(|b| b.supports_stable());

            let run_kinds = if collector::version_supports_incremental(id) {
                RunKind::all()
            } else {
                RunKind::all_non_incr()
            };
            let CommitData {
                benchmarks: benchmark_data,
                ..
            } = bench_commit(
                None,
                &commit,
                &[BuildKind::Check, BuildKind::Debug, BuildKind::Opt],
                &run_kinds,
                Compiler {
                    rustc: &toolchain.binary_file("rustc"),
                    cargo: &toolchain.binary_file("cargo"),
                    is_nightly: false,
                    triple: "x86_64-unknown-linux-gnu",
                },
                &benchmarks,
                3,
                false,
            );
            repo.success_artifact(&ArtifactData {
                id: id.to_string(),
                benchmarks: benchmark_data,
            })?;
            Ok(0)
        }

        ("process", Some(_)) => {
            let out_repo = get_out_repo(false)?;
            println!("processing commits");
            let client = reqwest::Client::new();
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
                return Ok(0);
            };

            let commit = get_commit_or_fake_it(&commit)?;
            match Sysroot::install(&commit.sha, commit.date.0, "x86_64-unknown-linux-gnu") {
                Ok(sysroot) => {
                    let result = out_repo.success(&bench_commit(
                        Some(&out_repo),
                        &commit,
                        &[BuildKind::Check, BuildKind::Debug, BuildKind::Opt],
                        &RunKind::all(),
                        Compiler::from_sysroot(&sysroot),
                        &benchmarks,
                        3,
                        true,
                    ));
                    if let Err(err) = result {
                        out_repo.write_broken_commit(&commit, err)?;
                    }
                }
                Err(err) => {
                    error!("failed to install sysroot for {:?}: {:?}", commit, err);
                }
            }
            Ok(0)
        }

        ("profile", Some(sub_m)) => {
            let rustc = sub_m.value_of("RUSTC").unwrap();
            let cargo = sub_m.value_of("CARGO").unwrap();
            let build_kinds = build_kinds_from_arg(&sub_m.value_of("BUILDS"))?;
            let run_kinds = run_kinds_from_arg(&sub_m.value_of("RUNS"))?;
            let profiler = Profiler::from_name(sub_m.value_of("PROFILER").unwrap())?;
            let id = sub_m.value_of("ID").unwrap();

            info!("Profile with {:?}", profiler);

            let rustc_path = PathBuf::from(rustc).canonicalize()?;
            let cargo_path = PathBuf::from(cargo).canonicalize()?;
            let compiler = Compiler {
                rustc: &rustc_path,
                cargo: &cargo_path,
                is_nightly: true,
                triple: "x86_64-unknown-linux-gnu", // XXX: Technically not necessarily true
            };

            for (i, benchmark) in benchmarks.iter().enumerate() {
                let out_dir = get_out_dir();
                let mut processor = execute::ProfileProcessor::new(profiler, &out_dir, &id);
                let result =
                    benchmark.measure(&mut processor, &build_kinds, &run_kinds, compiler, 1);
                if let Err(ref s) = result {
                    info!(
                        "failed to profile {} with {:?}, recorded: {:?}",
                        benchmark.name, profiler, s
                    );
                }
                info!("{} benchmarks left", benchmarks.len() - i - 1);
            }
            Ok(0)
        }

        ("remove_benchmark", Some(sub_m)) => {
            let benchmark = sub_m.value_of("BENCHMARK").unwrap();
            let out_repo = get_out_repo(false)?;
            for commit in &get_commits()? {
                if let Ok(mut data) = out_repo.load_commit_data(&commit, "x86_64-unknown-linux-gnu")
                {
                    if data.benchmarks.remove(&*benchmark).is_none() {
                        warn!("could not remove {} from {}", benchmark, commit.sha);
                    }
                    out_repo.add_commit_data(&data)?;
                }
            }
            Ok(0)
        }

        ("remove_errs", Some(_)) => {
            for commit in &get_commits()? {
                let out_repo = get_out_repo(false)?;
                if let Ok(mut data) = out_repo.load_commit_data(&commit, "x86_64-unknown-linux-gnu")
                {
                    let benchmarks = data
                        .benchmarks
                        .into_iter()
                        .filter(|&(_, ref v)| v.is_ok())
                        .collect();
                    data.benchmarks = benchmarks;
                    out_repo.add_commit_data(&data)?;
                }
            }
            Ok(0)
        }

        ("test_benchmarks", Some(_)) => {
            if let Some(commit) = get_commits()?.last() {
                let sysroot =
                    Sysroot::install(&commit.sha, commit.date.0, "x86_64-unknown-linux-gnu")?;
                // filter out servo benchmarks as they simply take too long
                bench_commit(
                    None,
                    commit,
                    &[BuildKind::Check], // no Debug or Opt builds
                    &RunKind::all(),
                    Compiler::from_sysroot(&sysroot),
                    &benchmarks,
                    1,
                    false,
                );
            } else {
                panic!("no commits");
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
