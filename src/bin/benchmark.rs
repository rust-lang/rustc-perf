#![recursion_limit = "1024"]

#[macro_use] extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate error_chain;
extern crate flate2;
extern crate tar;
extern crate rustc_perf_collector;
extern crate env_logger;
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

use std::fs::{self, OpenOptions, File};
use std::str;
use std::path::{Path, PathBuf};
use std::io::{stdout, stderr, Read, Write};
use std::collections::HashMap;

use rustc_perf_collector::{Patch, Commit, CommitData};

mod execute;
mod github;
mod sysroot;
mod time_passes;

use execute::Benchmark;

/// Download a commit from AWS and run benchmarks on it.
fn bench_commit(sha: &str, triple: &str, benchmarks: &[Benchmark])
                -> Result<HashMap<String, Vec<Patch>>>
{
    let sysroot = sysroot::Sysroot::install(sha, triple)?;

    let results : Result<Vec<_>> = benchmarks.iter().map(|benchmark| {
        Ok((benchmark.name.clone(), benchmark.run(&sysroot)?))
    }).collect();

    Ok(results?.into_iter().collect())
}

const COMMIT_FILE: &'static str = "last-commit-sha";

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

fn process_commit2(commit: &Commit, benchmarks: &[Benchmark]) -> Result<()> {
    let triple = "x86_64-unknown-linux-gnu";
    let results = bench_commit(&commit.sha, triple, benchmarks)?;

    let file_data = CommitData {
        commit: commit.clone(),
        benchmarks: results
    };

    let filepath = format!("times/{}-{}-{}.json", commit.date, commit.sha, triple);
    info!("creating file {}", filepath);
    let mut file = File::create(&filepath)?;
    serde_json::to_writer(&mut file, &file_data)?;

    Ok(())
}

fn process_commit(commit: &Commit, benchmarks: &[Benchmark]) -> Result<()> {
    if let Err(e) = process_commit2(commit, &benchmarks) {
        let mut file = OpenOptions::new().append(true).create(true).open("broken-commits")?;
        writeln!(file, "{}: {:?}", commit.sha, e)?;
        info!("running {} failed: {:?}", commit.sha, e);
    }

    // Even if we failed with this commit, we should still update the SHA in the file so
    // future runs can continue and don't have to re-run.
    let mut commit_file = File::create(COMMIT_FILE)?;
    commit_file.write_all(commit.sha.as_bytes())?;
    Ok(())
}

fn get_last_commit() -> Result<String> {
    let mut commit_file = File::open(COMMIT_FILE)
        .chain_err(|| format!("expected {} to exist, and contain the last tested commit sha", COMMIT_FILE))?;

    let mut last_commit = String::new();
    commit_file.read_to_string(&mut last_commit)?;

    if last_commit.is_empty() {
        bail!("{} was empty", COMMIT_FILE);
    }

    Ok(last_commit.trim().to_owned())
}

fn process(benchmarks: &[Benchmark]) -> Result<()> {
    let commits = github::get_new_commits(&get_last_commit()?)?;
    if !commits.is_empty() {
        info!("new commits: {:?}", commits);
        // We need to reverse the commits in order to have the first commit be the one directly
        // after the commit in the commit file
        for commit in commits.iter().rev() {
            process_commit(commit, &benchmarks)?;
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
       (@subcommand process =>
           (about: "syncs to git and collects performance data for all versions")
       )
       (@subcommand bench_commit =>
           (about: "benchmark a bors merge from AWS and output data to stdout")
           (@arg COMMIT: +required +takes_value "Commit hash to bench")
        )
    ).get_matches();
    let benchmark_dir = PathBuf::from(matches.value_of_os("benchmarks_dir").unwrap());
    let filter = matches.value_of("filter");
    let benchmarks = get_benchmarks(&benchmark_dir, filter)?;

    match matches.subcommand() {
        ("process", Some(_)) => {
            process(&benchmarks)?;
            Ok(0)
        }
        ("bench_commit", Some(sub_m)) => {
            let commit = sub_m.value_of("COMMIT").unwrap();
            let result = bench_commit(commit, "x86_64-unknown-linux-gnu", &benchmarks)?;
            serde_json::to_writer(&mut stdout(), &result)?;
            Ok(0)
        }
        _ => {
            let _ = writeln!(stderr(), "{}", matches.usage());
            Ok(2)
        }
    }
}
