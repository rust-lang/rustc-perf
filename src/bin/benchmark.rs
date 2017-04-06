#![recursion_limit = "1024"]

#[macro_use] extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate error_chain;
extern crate flate2;
extern crate tar;
extern crate rustc_perf_collector;
extern crate env_logger;
extern crate tempdir;
#[macro_use] extern crate log;
extern crate reqwest;
extern crate chrono;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Reqwest(::reqwest::Error);
            Serde(::serde_json::Error);
            Io(::std::io::Error);
        }
    }
}

use errors::*;

quick_main!(run);

use std::fs;
use std::str;
use std::path::{Path, PathBuf};
use std::io::{stdout, stderr, Write};
use std::collections::HashMap;

use rustc_perf_collector::{Patch, Commit, CommitData};

mod execute;
mod github;
mod outrepo;
mod sysroot;
mod time_passes;

use outrepo::CommitKind;

use execute::Benchmark;

/// Download a commit from AWS and run benchmarks on it.
fn bench_commit(commit: &Commit, triple: &str, benchmarks: &[Benchmark], preserve_sysroot: bool)
                -> Result<HashMap<String, Vec<Patch>>>
{
    info!("benchmarking commit {} ({}) for triple {}", commit.sha, commit.date, triple);
    let sysroot = sysroot::Sysroot::install(commit, triple, preserve_sysroot)?;

    let results : Result<Vec<_>> = benchmarks.iter().map(|benchmark| {
        Ok((benchmark.name.clone(), benchmark.run(&sysroot)?))
    }).collect();

    Ok(results?.into_iter().collect())
}

fn get_benchmarks(benchmark_dir: &Path, filter: Option<&str>) -> Result<Vec<Benchmark>> {
    let mut benchmarks = Vec::new();
    for entry in fs::read_dir(benchmark_dir).chain_err(|| "failed to list benchmarks")? {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e)
        };

        if path.ends_with(".git") || path.ends_with("scripts") || !entry.file_type()?.is_dir() {
            info!("benchmark {} - ignored", name);
            continue;
        }

        if let Some(filter) = filter {
            if !name.contains(filter) {
                info!("benchmark {} - filtered", name);
                continue;
            }
        }

        info!("benchmark {} - REGISTERED", name);
        benchmarks.push(Benchmark {
            path: path,
            name: name
        });
    }
    Ok(benchmarks)
}

fn process_commit(repo: &outrepo::Repo, commit: &Commit, benchmarks: &[Benchmark],
                  preserve_sysroot: bool, kind: CommitKind) -> Result<()> {
    let triple = "x86_64-unknown-linux-gnu";
    match bench_commit(commit, triple, benchmarks, preserve_sysroot) {
        Ok(results) => {
            let file_data = CommitData {
                commit: commit.clone(),
                benchmarks: results
            };
            repo.success(triple, &file_data, kind)
        }
        Err(error) => {
            info!("running {} failed: {:?}", commit.sha, error);
            repo.failure(commit, &error, kind)
        }
    }
}


fn process_retries(repo: &mut outrepo::Repo, benchmarks: &[Benchmark], preserve_sysroot: bool)
                   -> Result<()>
{
    while let Some(retry) = repo.next_retry() {
        info!("retrying {}", retry);
        // we don't want to reuse the `client` between commits, because it will time out
        // in the time it takes to benchmark a commit.
        let client = reqwest::Client::new()?;
        let commit = github::commit_from_sha(&client, &retry)?;
        process_commit(repo, &commit, benchmarks, preserve_sysroot, CommitKind::Retry)?;
    }
    Ok(())
}

fn process_commits(repo: &outrepo::Repo, benchmarks: &[Benchmark], preserve_sysroot: bool)
                   -> Result<()>
{
    let last_commit = repo.get_last_commit()?;
    let commits = github::get_new_commits(&last_commit)?;
    if !commits.is_empty() {
        info!("new commits: {:?}", commits);
        // We need to reverse the commits in order to have the first commit be the one directly
        // after the commit in the commit file
        for commit in commits.iter().rev() {
            process_commit(repo, commit, &benchmarks, preserve_sysroot, CommitKind::Progress)?;
        }
    } else {
        info!("Nothing to do; no new commits.");
    }
    Ok(())
}

fn run() -> Result<i32> {
    env_logger::init().expect("logger initialization successful");

    let matches = clap_app!(rustc_perf_collector =>
       (version: "0.1")
       (author: "The Rust Compiler Team")
       (about: "Collects Rust performance data")
       (@arg benchmarks_dir: --benchmarks-dir +required +takes_value "Sets the directory benchmarks are found in")
       (@arg filter: --filter +takes_value "Run only benchmarks that contain this")
       (@arg preserve_sysroots: -p --preserve "Don't delete sysroots after running.")
       (@subcommand process =>
           (about: "syncs to git and collects performance data for all versions")
           (@arg OUTPUT_REPOSITORY: +required +takes_value "Repository to output to")
       )
       (@subcommand bench_commit =>
           (about: "benchmark a bors merge from AWS and output data to stdout")
           (@arg COMMIT: +required +takes_value "Commit hash to bench")
       )
    ).get_matches();
    let benchmark_dir = PathBuf::from(matches.value_of_os("benchmarks_dir").unwrap());
    let filter = matches.value_of("filter");
    let benchmarks = get_benchmarks(&benchmark_dir, filter)?;
    let preserve_sysroots = matches.is_present("preserve_sysroots");

    match matches.subcommand() {
        ("process", Some(sub_m)) => {
            let out_repo = PathBuf::from(sub_m.value_of_os("OUTPUT_REPOSITORY").unwrap());
            let mut out_repo = outrepo::Repo::open(out_repo)?;
            process_retries(&mut out_repo, &benchmarks, preserve_sysroots)?;
            process_commits(&out_repo, &benchmarks, preserve_sysroots)?;
            Ok(0)
        }
        ("bench_commit", Some(sub_m)) => {
            let commit = sub_m.value_of("COMMIT").unwrap();
            let client = reqwest::Client::new()?;
            let commit = github::commit_from_sha(&client, commit)?;
            let result = bench_commit(&commit, "x86_64-unknown-linux-gnu", &benchmarks,
                                      preserve_sysroots)?;
            serde_json::to_writer(&mut stdout(), &result)?;
            Ok(0)
        }
        _ => {
            let _ = writeln!(stderr(), "{}", matches.usage());
            Ok(2)
        }
    }
}
