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
use std::env;
use std::str;
use std::path::{Path, PathBuf};
use std::io::{stdout, stderr, Read, Write, BufReader};
use std::process::Command;
use std::collections::HashMap;

use rustc_perf_collector::{Pass, Run, Patch, Commit, CommitData};

use tar::Archive;
use flate2::bufread::GzDecoder;
use serde_json::Value;
use reqwest::header::Authorization;

use chrono::{TimeZone, UTC};

const BASE_PATH: &'static str = "https://s3.amazonaws.com/rust-lang-ci/rustc-builds";
const GH_API_TOKEN: &'static str = env!("GH_API_TOKEN");

struct Benchmark {
    name: String,
    path: PathBuf
}

impl Benchmark {
    /// Run a specific benchmark on a specific commit
    fn run(&self, rustc: &Path, cargo: &Path) -> Result<Vec<Patch>> {
        info!("processing {}", self.name);
        let output = command("make", &cargo, &rustc).arg("patches").current_dir(&self.path).output()?;
        let mut patches = str::from_utf8(&output.stdout)
            .chain_err(|| format!("make patches in {} returned non UTF-8 output", self.path.display()))?
            .split_whitespace()
            .collect::<Vec<_>>();
        if patches.is_empty() {
            patches.push("");
        }

        let mut patch_runs: Vec<Patch> = Vec::new();
        for _ in 0..3 {
            for patch in &patches {
                info!("running `make all{}`", patch);
                let output = command("make", &cargo, &rustc).arg(&format!("all{}", patch))
                    .current_dir(&self.path)
                    .env("CARGO_OPTS", "")
                    .env("CARGO_RUSTC_OPTS", "-Z time-passes")
                    .output()?;

                if !output.status.success() {
                    bail!("stderr non empty: {}", String::from_utf8_lossy(&output.stderr));
                }

                let patch_index = if let Some(p) = patch_runs.iter().position(|p_run| p_run.patch == *patch) {
                    p
                } else {
                    patch_runs.push(Patch {
                        patch: patch.to_string(),
                        name: self.name.clone(),
                        runs: Vec::new(),
                    });
                    patch_runs.len() - 1
                };

                let combined_name = format!("{}{}", self.name, patch);
                patch_runs[patch_index].runs.push(Run {
                    passes: process_output(&combined_name, output.stdout)?,
                    name: combined_name,
                });
            }
            if !command("make", &cargo, &rustc).arg("touch").current_dir(&self.path).status()?.success() {
                bail!("{}: make touch failed.", self.path.display());
            }
        }

        let mut patches = Vec::new();
        for patch_run in patch_runs {
            let mut runs = patch_run.runs.into_iter();
            let mut pa = PassAverager::new(runs.next().unwrap().passes);
            for run in runs {
                pa.average_with(run.passes)?;
            }
            patches.push(Patch {
                name: patch_run.name.clone(),
                patch: patch_run.patch.clone(),
                runs: vec![
                    Run {
                        name: patch_run.name + &patch_run.patch,
                        passes: pa.state,
                    }
                ],
            });
        }

        if !command("make", &cargo, &rustc).arg("clean").current_dir(&self.path).status()?.success() {
            bail!("{}: make touch failed.", self.path.display());
        }

        Ok(patches)
    }
}

fn get_and_extract(url: &str, into: &str, is_std: bool) -> Result<()> {
    info!("requesting: {}", url);
    let resp = reqwest::get(url)?;
    info!("{}", resp.status());
    let mut resp = BufReader::new(resp);

    let decoder = GzDecoder::new(&mut resp)?;
    let mut archive = Archive::new(decoder);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let mut components = path.components();
        assert!(components.next().is_some(), "strip container directory");
        let path = components.as_path();
        let path = if is_std {
            if let Ok(path) = path.strip_prefix("rust-std-x86_64-unknown-linux-gnu/lib/rustlib") {
                Path::new("rustc/lib/rustlib").join(path)
            } else {
                continue;
            }
        } else {
            path.into()
        };
        let path = Path::new(into).join(path);
        fs::create_dir_all(&path.parent().unwrap())
            .chain_err(|| format!("could not create intermediate directories for {}",
                    path.display()))?;
        entry.unpack(path)?;
    }
    Ok(())
}

fn process_output(name: &str, output: Vec<u8>) -> Result<Vec<Pass>> {
    let output = String::from_utf8(output)
        .chain_err(|| format!("unable to convert output of {} to UTF-8", name))?;
    let mut passes = Vec::new();

    for line in output.lines() {
        if line.starts_with("time: ") {
            let time = &line["time: ".len()..line.find(";").unwrap()];
            let time: f64 = time.parse().chain_err(|| format!("parsed time: {:?}", time))?;

            let mem = &line[line.find("rss: ").unwrap() + 5..line.find("MB").unwrap()];
            let mem: u64 = mem.parse().chain_err(|| format!("parsed memory: {:?}", mem))?;

            let name = line[line.find("MB\t").unwrap() + 3..].to_string();
            passes.push(Pass {
                name: name,
                time: time,
                mem: mem,
            });
        } else if line.starts_with("  time: ") {
            // XXX: sub passes currently cause problems because their names are inconsistent in
            // incremental runs, especially codegen passes
            //let time = &line["  time: ".len()..line.find(";").unwrap()];
            //let time: f64 = time.parse().chain_err(|| format!("parsed time: {:?}", time))?;

            //let mem = &line[line.find("rss: ").unwrap() + 5..line.find("MB").unwrap()];
            //let mem: u64 = mem.parse().chain_err(|| format!("parsed memory: {:?}", mem))?;

            //let name = line[line.find("MB\t").unwrap() + 3..].to_string();
            //sub_passes.insert(name.clone(), Pass {
            //    name: name,
            //    time: time,
            //    mem: mem,
            //    sub_passes: HashMap::new(),
            //});
        } else {
            //info!("unhandled line: {}", line);
        }
    }

    Ok(passes)
}

// if running with cargo run, we want to avoid things like CARGO_INCREMENTAL
// sneaking into the command's environment, but we do need the PATH to
// find linkers and other things that cargo and rust needs.
fn command<P: AsRef<Path>>(path: P, cargo: &Path, rustc: &Path) -> Command {
    let mut command = Command::new(path.as_ref().as_os_str());
    command
        .env_clear()
        .env("PATH", env::var("PATH").unwrap_or_default())
        .env("CARGO", &cargo)
        .env("RUSTC", &rustc);
    command
}

struct PassAverager {
    state: Vec<Pass>,
    runs: u64
}

impl PassAverager {
    fn new(state: Vec<Pass>) -> Self {
        Self {
            state: state,
            runs: 0,
        }
    }

    fn average_with(
        &mut self,
        b: Vec<Pass>,
    ) -> Result<()> {
        self.runs += 1;

        for a in &mut self.state {
            let b = match b.iter().find(|p| p.name == a.name) {
                Some(b) => b,
                None => bail!("expected name {} to exist in both a and b", a.name),
            };
            a.time = a.time + ((b.time - a.time) / (self.runs as f64));
            a.mem = a.mem + ((b.mem - b.mem) / self.runs);
        }

        Ok(())
    }
}

/// Download a commit from AWS and run benchmarks on it.
fn bench_commit(sha: &str, triple: &str, benchmarks: &[Benchmark])
                -> Result<HashMap<String, Vec<Patch>>>
{
    let unpack_into = format!("rust-{}", sha);
    get_and_extract(
        &format!("{}/{}/rustc-nightly-{}.tar.gz", BASE_PATH, sha, triple),
        &unpack_into,
        false,
    )?;
    get_and_extract(
        &format!("{}/{}/rust-std-nightly-{}.tar.gz", BASE_PATH, sha, triple),
        &unpack_into,
        true,
    )?;
    get_and_extract(
        &format!("{}/{}/cargo-nightly-{}.tar.gz", BASE_PATH, sha, triple),
        &unpack_into,
        false,
    )?;
    let rustc = PathBuf::from(format!("rust-{}/rustc/bin/rustc", sha)).canonicalize()
        .chain_err(|| "failed to canonicalize rustc path")?;
    let cargo = PathBuf::from(format!("rust-{}/cargo/bin/cargo", sha)).canonicalize()
        .chain_err(|| "failed to canonicalize cargo path")?;

    let version = String::from_utf8(command(&rustc, &cargo, &rustc).arg("--version").output()
        .chain_err(|| format!("{} --version", rustc.display()))?.stdout).unwrap();
    info!("version: {}", version);

    let results : Result<Vec<_>> = benchmarks.iter().map(|benchmark| {
        Ok((benchmark.name.clone(), benchmark.run(&rustc, &cargo)?))
    }).collect();

    fs::remove_dir_all(&unpack_into).unwrap_or_else(|err| {
        info!("failed to remove {}, please do so manually: {:?}", unpack_into, err);
    });

    Ok(results?.into_iter().collect())
}

const COMMIT_FILE: &'static str = "last-commit-sha";

fn get_new_commits(client: &reqwest::Client) -> Result<Vec<Commit>> {
    let mut commit_file = File::open(COMMIT_FILE)
        .chain_err(|| format!("expected {} to exist, and contain the last tested commit sha", COMMIT_FILE))?;

    let mut last_commit = String::new();
    commit_file.read_to_string(&mut last_commit)?;

    if last_commit.is_empty() {
        bail!("{} was empty", COMMIT_FILE);
    }

    let last_commit = last_commit.trim();

    fn request(client: &reqwest::Client, url: &str, last_commit: &str) -> Result<Vec<Commit>> {
        fn convert_to_str_array(url: &str, value: Value) -> Result<Vec<Commit>> {
            let value = if let Value::Array(arr) = value {
                arr.into_iter().map(|value| {
                    if let Value::Object(mut map) = value {
                        return Ok(Commit {
                            sha: map.remove("sha").expect("sha to be present").as_str().unwrap().to_string(),
                            date: UTC.datetime_from_str(map.remove("commit")
                                .and_then(|mut commit| commit.as_object_mut()
                                    .and_then(|commit| commit.remove("committer")))
                                .and_then(|mut committer| committer.as_object_mut()
                                    .and_then(|committer| committer.remove("date")))
                                .expect("commit.comitter.date to be present").as_str().unwrap(),
                                "%+").expect("failed to parse date"),
                        });
                    }
                    bail!("{} returned element without string sha key in array element", url)
                }).collect::<Result<Vec<Commit>>>()?
            } else {
                bail!("{} returned non-array response: {}", url, value);
            };

            Ok(value)
        }

        info!("Requesting: {}", url);

        let mut response = client.get(url)
            .header(Authorization(format!("token {}", GH_API_TOKEN)))
            .send().chain_err(|| format!("API request to {}", url))?;
        let value: Value = serde_json::from_reader(&mut response)
            .chain_err(|| format!("API request to {} deserialization", url))?;
        let mut commits = convert_to_str_array(url, value)?;

        if let Some(pos) = commits.iter().position(|commit| commit.sha == last_commit) {
            commits.truncate(pos);
            return Ok(commits);
        }

        if let Some(headers) = response.headers().get_raw("Link") {
            if let Some(next) = headers.iter().map(|s| str::from_utf8(s).unwrap()).flat_map(|s| s.split(", ")).find(|s| s.contains(r#"rel="next"#)) {
                let url = &next[1..next.find(">").unwrap()];
                let next_value = request(client, url, last_commit)?;
                commits.extend(next_value);
                if let Some(_) = commits.iter().find(|commit| commit.sha == last_commit) {
                    return Ok(commits);
                }

            }
        }

        Ok(commits)
    }

    let commits = request(
        client,
        "https://api.github.com/repos/rust-lang/rust/commits?author=bors&per_page=100",
        &last_commit,
    )?;

    Ok(commits)
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

fn process(benchmarks: &[Benchmark]) -> Result<()> {
    let client = reqwest::Client::new()?;
    let commits = get_new_commits(&client)?;
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
