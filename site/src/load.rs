// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Read;
use std::env;

use serde_json;
use antidote::Mutex;
use failure::{ResultExt, Error};
use failure::SyncFailure;
use rust_sysroot;
use rust_sysroot::git::Commit as GitCommit;
use chrono::{Duration, Utc};
use toml;

use util;
use util::Interpolate;
use git;
use collector::Date;

pub use collector::{RunId, Benchmark, CommitData, Commit, ArtifactData, Patch, Run, Stat};
use collector;

#[derive(Debug, Serialize, Deserialize)]
pub enum InterpolationSource {
    /// We interpolated the first commit in the data set from the commit
    /// here
    First(Commit),

    /// We interpolated the last commit in the data set from the commit
    /// here
    Last(Commit),

    /// We interpolated a commit in the middle from the two commits
    /// around it (but not necessarily directly adjacent -- generally
    /// ranges of commits don't have the data).
    ///
    /// Data is interpolated linearly between these two commits.
    Middle(Commit, Commit),
}

#[derive(Debug)]
pub struct Interpolation {
    pub benchmark: String,
    pub run: Option<RunId>,
    pub from: InterpolationSource,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CurrentState {
    pub commit: Commit,
    pub benchmarks: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TryCommit {
    Parent { sha: String, parent_sha: String, },
    UnkownParent { sha: String },
}

impl TryCommit {
    pub fn sha(&self) -> &str {
        match self {
            TryCommit::Parent { sha, .. } => &*sha,
            TryCommit::UnkownParent { sha } => &*sha,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Persistent {
    pub try_commits: Vec<TryCommit>,
    pub current: Option<CurrentState>,
}

impl Persistent {
    pub fn write(&self) -> Result<(), Error> {
        let s = serde_json::to_string(self)?;
        fs::write("persistent.json", &s).with_context(|_| format!("failed to write persistent DB"))?;
        Ok(())
    }

    fn load() -> Persistent {
        let p = Persistent::load_().unwrap_or_else(|| Persistent {
            try_commits: Vec::new(),
            current: None,
        });
        p.write().unwrap();
        p
    }

    fn load_() -> Option<Persistent> {
        let s = fs::read_to_string("persistent.json").ok()?;
        let persistent: Persistent = serde_json::from_str(&s).ok()?;

        Some(persistent)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Keys {
    pub github: Option<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub users: Vec<String>,
    pub keys: Keys,
}

#[derive(Debug)]
pub struct InputData {
    /// A set containing all crate names of the bootstrap kind.
    pub crate_list: BTreeSet<String>,

    /// All known statistics gathered for crates
    pub stats_list: BTreeSet<String>,

    /// The last date that was seen while loading files. The DateTime variant is
    /// used here since the date may or may not contain a time. Since the
    /// timezone is not important, it isn't stored, hence the Naive variant.
    pub last_date: Date,

    /// `data_real` is as-is, `data` has been interpolated.
    data_real: BTreeMap<Commit, CommitData>,
    data: BTreeMap<Commit, CommitData>,

    /// The benchmarks we interpolated for a given commit.
    ///
    /// Not all commits are in this map.
    pub interpolated: HashMap<String, Vec<Interpolation>>,

    /// The list of commits in the `data` map.
    pub data_commits: Vec<Commit>,
    /// A map from commit to index in the `commits` vector.
    pub commit_map: HashMap<Commit, usize>,

    pub artifact_data: BTreeMap<String, ArtifactData>,

    pub commits: Vec<GitCommit>,

    pub persistent: Mutex<Persistent>,

    pub config: Config,
}

impl InputData {
    pub fn data(&self, interpolate: Interpolate) -> &BTreeMap<Commit, CommitData> {
        match interpolate {
            Interpolate::Yes => &self.data,
            Interpolate::No => &self.data_real,
        }
    }

    /// Initialize `InputData from the file system.
    pub fn from_fs(repo_loc: &str) -> Result<InputData, Error> {
        let repo_loc = PathBuf::from(repo_loc);
        let mut skipped = 0;
        let mut artifact_data = BTreeMap::new();
        let mut data = BTreeMap::new();

        if !repo_loc.exists() {
            // If the repository doesn't yet exist, simplify clone it to the given location.
            info!(
                "cloning repository into {}, since it doesn't exist before",
                repo_loc.display()
            );
            git::execute_command(
                &env::current_dir()?,
                &[
                    "clone",
                    "https://github.com/rust-lang-nursery/rustc-timing.git",
                    repo_loc.to_str().unwrap(),
                ],
            )?;
        }

        // Read all files from repo_loc/processed
        let mut file_count = 0;
        for entry in fs::read_dir(repo_loc.join("times"))? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }
            file_count += 1;

            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();
            let mut file = File::open(entry.path())?;
            let mut file_contents = String::new();
            // Skip files whose size is 0.
            if file.read_to_string(&mut file_contents)? == 0 {
                warn!("Skipping empty file: {}", filename);
                skipped += 1;
                continue;
            }

            if filename.starts_with("artifact-") {
                let contents: ArtifactData = match serde_json::from_str(&file_contents) {
                    Ok(j) => j,
                    Err(err) => {
                        error!("Failed to parse JSON for {}: {:?}", filename, err);
                        skipped += 1;
                        continue;
                    }
                };
                if contents.benchmarks.is_empty() {
                    warn!("empty benchmarks hash for {}", filename);
                    skipped += 1;
                    continue;
                }

                artifact_data.insert(contents.id.clone(), contents);
            } else {
                let contents: CommitData = match serde_json::from_str(&file_contents) {
                    Ok(json) => json,
                    Err(err) => {
                        error!("Failed to parse JSON for {}: {:?}", filename, err);
                        skipped += 1;
                        continue;
                    }
                };
                if contents.benchmarks.is_empty() {
                    warn!("empty benchmarks hash for {}", filename);
                    skipped += 1;
                    continue;
                }

                data.insert(contents.commit.clone(), contents);
            }
        }

        info!("{} total files", file_count);
        info!("{} skipped files", skipped);
        info!("{} measured", data.len());

        let config = if let Ok(s) = fs::read_to_string("site-config.toml") {
            toml::from_str(&s)?
        } else {
            Config {
                users: Vec::new(),
                keys: Keys::default(),
            }
        };

        InputData::new(data, artifact_data, config)
    }

    pub fn new(
        data: BTreeMap<Commit, CommitData>,
        artifact_data: BTreeMap<String, ArtifactData>,
        config: Config,
    ) -> Result<InputData, Error> {
        let mut last_date = None;
        let mut crate_list = BTreeSet::new();
        let mut stats_list = BTreeSet::new();

        for commit_data in data.values() {
            if last_date.is_none() || last_date.as_ref().unwrap() < &commit_data.commit.date {
                last_date = Some(commit_data.commit.date);
            }

            let benchmarks = commit_data
                .benchmarks
                .values()
                .filter_map(|v| v.as_ref().ok());
            for benchmark in benchmarks {
                for run in &benchmark.runs {
                    let run_name = benchmark.name.clone() + &run.name();
                    crate_list.insert(run_name);
                    for stat in &run.stats {
                        stats_list.insert(stat.name.clone());
                    }
                }
            }
        }

        let last_date = last_date.expect("No dates found");
        println!("Updating rust.git clone...");
        let commits = rust_sysroot::get_commits(rust_sysroot::EPOCH_COMMIT, "master").map_err(SyncFailure::new)?;
        println!("Update of rust.git complete");

        let data_commits = data.keys().cloned().collect::<Vec<_>>();

        let mut commit_map = HashMap::with_capacity(data_commits.len());
        for (idx, commit) in data_commits.iter().enumerate() {
            commit_map.insert(commit.clone(), idx);
        }

        eprintln!("Starting interpolation...");
        let mut latest_section_start = ::std::time::Instant::now();
        let start = ::std::time::Instant::now();
        let data_real = data.clone();
        let mut interpolated = HashMap::new();
        let mut data_next = data;

        let current_benchmarks = data_real.iter().rev().take(20)
            .flat_map(|(_, cd)| {
                cd.benchmarks.keys().cloned()
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let mut known_runs: HashMap<String, HashSet<RunId>> = HashMap::new();
        for (_, cd) in data_real.iter().rev().take(20) {
            for (name, benchmark) in &cd.benchmarks {
                if let Ok(benchmark) = benchmark {
                    let mut entry = known_runs.entry(name.clone())
                        .or_insert_with(HashSet::new);
                    for run in &benchmark.runs {
                        entry.insert(run.id());
                    }
                }
            }
        }
        trace!("computed current benchmarks and runs, in {:?}", latest_section_start.elapsed());
        latest_section_start = ::std::time::Instant::now();

        let mut last_commit = Vec::with_capacity(data_next.len());
        let mut next_commit = Vec::with_capacity(data_next.len());

        let mut last_seen = HashMap::new();
        for needle in data_commits.iter() {
            for (name, value) in &data_real[needle].benchmarks {
                if value.is_ok() {
                    last_seen.insert(name.clone(), needle.clone());
                }
            }
            last_commit.push(last_seen.clone());
        }
        last_seen.clear();
        for needle in data_commits.iter().rev() {
            for (name, value) in &data_real[needle].benchmarks {
                if value.is_ok() {
                    last_seen.insert(name.clone(), needle.clone());
                }
            }
            next_commit.push(last_seen.clone());
        }
        next_commit.reverse();

        trace!("computed start/ends of benchmark holes in {:?}", latest_section_start.elapsed());
        latest_section_start = ::std::time::Instant::now();

        // Find the earliest and latest (scanning from left and from right) runs for every
        // benchmark

        let mut last_run = Vec::with_capacity(data_next.len());
        let mut next_run = Vec::with_capacity(data_next.len());

        let mut last_seen = HashMap::new();
        for (idx, needle) in data_commits.iter().enumerate() {
            for (name, value) in &data_real[needle].benchmarks {
                if let Ok(bench) = value {
                    let mut e = last_seen
                        .entry(name.clone())
                        .or_insert_with(HashMap::new);
                    for run in bench.runs.iter() {
                        e.insert(run.id(), (idx, run.clone()));
                    }
                }
            }
            last_run.push(last_seen.clone());
        }
        last_seen.clear();
        for (idx, needle) in data_commits.iter().enumerate().rev() {
            for (name, value) in &data_real[needle].benchmarks {
                if let Ok(bench) = value {
                    let mut e = last_seen
                        .entry(name.clone())
                        .or_insert_with(HashMap::new);
                    for run in bench.runs.iter() {
                        e.insert(run.id(), (idx, run.clone()));
                    }
                }
            }
            next_run.push(last_seen.clone());
        }
        next_run.reverse();

        trace!("computed start/ends of run holes in {:?}", latest_section_start.elapsed());
        latest_section_start = ::std::time::Instant::now();

        // The data holds this tree:
        //  [commit] -> [benchmark] -> [run] -> [stat]

        let mut dur = ::std::time::Duration::new(0, 0);
        for (commit, cd) in &mut data_next {
            for benchmark_name in &current_benchmarks {
                // We do not interpolate try commits today
                // because we don't track their parents so it's
                // difficult to add that data in.
                if commit.is_try() {
                    continue;
                }

                let mut assoc = AssociatedData {
                    commit: &commit,
                    data: &data_real,
                    commits: &data_commits,
                    commit_map: &commit_map,
                    interpolated: &mut interpolated,
                    last_seen_commit: &last_commit,
                    next_seen_commit: &next_commit,
                    last_seen_run: &last_run,
                    next_seen_run: &next_run,
                    dur: &mut dur,
                };

                let entry = cd.benchmarks.entry(benchmark_name.to_owned())
                    .or_insert_with(|| Err(String::from("dummy bench")));

                // benchmark did not run successfully at this commit
                // or benchmark did not attempt to run at this commit
                if entry.is_err() {
                    let runs = fill_benchmark_data(benchmark_name, &mut assoc);
                    // If we couldn't do this then do nothing
                    if let Some(runs) = runs {
                        *entry = Ok(Benchmark {
                            name: benchmark_name.to_owned(),
                            runs: runs,
                        });
                    }
                }

                // benchmark exists, but might have runs missing
                if let Ok(benchmark) = entry {
                    // If we've not had a benchmark at all in the last few
                    // commits then just skip run interpolation for it; the
                    // benchmark should get total-benchmark interpolated.
                    if let Some(known_runs) = known_runs.get(benchmark_name) {
                        let missing_runs = known_runs
                            .iter()
                            .filter(|rname| !benchmark.runs.iter().any(|r| *r == **rname))
                            .collect::<Vec<_>>();
                        if !missing_runs.is_empty() {
                            let before = benchmark.runs.len();
                            fill_benchmark_runs(benchmark, missing_runs, &mut assoc);
                            assert_ne!(before, benchmark.runs.len(), "made progress");
                        }
                    }
                }
            }
        }
        trace!("total time finding runs: {:?}", dur);

        let interpolated = interpolated.into_iter()
            .filter(|(_, v)| !v.is_empty())
            .collect::<HashMap<_, _>>();

        trace!("finished primary interpolation in {:?}", latest_section_start.elapsed());
        eprintln!("Interpolation of {} commits complete in {:?}", interpolated.len(), start.elapsed());
        let data = data_next;

        Ok(InputData {
            crate_list: crate_list,
            stats_list: stats_list,
            data_commits: data_commits,
            commit_map: commit_map,
            interpolated,
            last_date: last_date,
            data_real: data_real,
            data: data,
            artifact_data,
            commits,
            persistent: Mutex::new(Persistent::load()),
            config,
        })
    }

    pub fn missing_commits(&self) -> Result<Vec<Commit>, Error> {
        let known_benchmarks = self.data.values()
            .rev()
            .take(10)
            .flat_map(|v| v.benchmarks.keys())
            .collect::<HashSet<_>>();
        let have = self.data.iter().map(|(key, value)| (key.sha.clone(), value))
            .collect::<HashMap<_, _>>();
        let mut missing = self.commits
            .iter()
            .map(|commit| Commit { sha: commit.sha.clone(), date: Date(commit.date.clone()) })
            .filter(|c| Utc::now().signed_duration_since(c.date.0) < Duration::days(29))
            .filter(|c| {
                if let Some(cd) = have.get(&c.sha) {
                    // If we've missed any benchmark, we also want this commit
                    known_benchmarks
                        .iter()
                        .all(|b| cd.benchmarks.contains_key(&**b))
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();
        missing.reverse();

        let mut commits = self.persistent.lock().try_commits.iter()
            .flat_map(|c| match c {
                TryCommit::Parent { sha, parent_sha } => {
                    let mut ret = Vec::new();
                    // Schedule the parent first so that we have a more up to date master (in
                    // theory)
                    if let Some(commit_data) = have.get(&*parent_sha) {
                        ret.push(commit_data.commit.clone());
                    }
                    ret.push(
                        Commit { sha: sha.clone(), date: Date::ymd_hms(2001, 01, 01, 0, 0, 0) }
                    );
                    ret
                }
                TryCommit::UnkownParent { sha } => {
                    vec![
                        Commit { sha: sha.clone(), date: Date::ymd_hms(2001, 01, 01, 0, 0, 0) },
                    ]
                }
            })
            .filter(|c| !have.contains_key(&c.sha)) // we may have not updated the try-commits file
            .collect::<Vec<_>>();

        for missing_commit in missing {
            if commits.iter().any(|c| c.sha == missing_commit.sha) {
                continue;
            }

            commits.push(missing_commit);
        }

        let mut seen = HashSet::with_capacity(commits.len());

        // FIXME: replace with Vec::drain_filter when it stabilizes
        let mut i = 0;
        while i != commits.len() {
            if !seen.insert(commits[i].sha.clone()) {
                commits.remove(i);
            } else {
                i += 1;
            }
        }

        Ok(commits)
    }
}

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "util::round_float")] pub f64);

struct AssociatedData<'a> {
    commit: &'a Commit,
    data: &'a BTreeMap<Commit, CommitData>,
    commits: &'a [Commit],
    commit_map: &'a HashMap<Commit, usize>,
    interpolated: &'a mut HashMap<String, Vec<Interpolation>>,

    // By benchmark name
    last_seen_commit: &'a [HashMap<String, Commit>],
    next_seen_commit: &'a [HashMap<String, Commit>],

    last_seen_run: &'a [HashMap<String, HashMap<RunId, (usize, Run)>>],
    next_seen_run: &'a [HashMap<String, HashMap<RunId, (usize, Run)>>],

    dur: &'a mut ::std::time::Duration,
}

// This function can assume that the benchmark exists and is restricted to filling in runs within
// the benchmark.
fn fill_benchmark_runs(benchmark: &mut Benchmark, missing_runs: Vec<&RunId>, data: &mut AssociatedData) {
    let commit_idx = data.commit_map[data.commit];
    for missing_run in missing_runs {
        let time_start = ::std::time::Instant::now();
        let start = data.last_seen_run[commit_idx]
            .get(&benchmark.name).and_then(|b| b.get(missing_run));
        let end = data.next_seen_run[commit_idx]
            .get(&benchmark.name).and_then(|b| b.get(missing_run));
        let start_commit = start.map(|(idx, _)| data.commits[*idx].clone());
        let end_commit = end.map(|(idx, _)| data.commits[*idx].clone());
        *data.dur += time_start.elapsed();

        assert_ne!(start_commit.as_ref(), Some(data.commit));
        assert_ne!(end_commit.as_ref(), Some(data.commit));

        let mut interpolations = data.interpolated.entry(data.commit.sha.clone()).or_insert_with(Vec::new);
        let run = match (start, end) {
            (Some(srun), Some(erun)) => {
                let distance = (commit_idx - srun.0 - 1) + (erun.0 - commit_idx - 1);
                let from_start = commit_idx - srun.0 - 1;
                let interpolated_stats = interpolate_stats(&srun.1, &erun.1, distance, from_start);
                let mut interpolated_run = srun.1.clone();
                interpolated_run.stats = interpolated_stats;
                interpolations.push(Interpolation {
                    benchmark: benchmark.name.clone(),
                    run: Some(missing_run.clone()),
                    from: InterpolationSource::Middle(
                        start_commit.unwrap(),
                        end_commit.unwrap(),
                    ),
                });
                interpolated_run
            }
            (Some(srun), None) => {
                interpolations.push(Interpolation {
                    benchmark: benchmark.name.clone(),
                    run: Some(missing_run.clone()),
                    from: InterpolationSource::First(
                        start_commit.unwrap(),
                    ),
                });
                srun.1.clone()
            }
            (None, Some(erun)) => {
                interpolations.push(Interpolation {
                    benchmark: benchmark.name.clone(),
                    run: Some(missing_run.clone()),
                    from: InterpolationSource::Last(
                        end_commit.unwrap(),
                    ),
                });
                erun.1.clone()
            }
            (None, None) => {
                unreachable!("{} run in benchmark {} has no entries, but it's missing!",
                    missing_run, benchmark.name)
            }
        };
        benchmark.runs.push(run);
    }
}

fn fill_benchmark_data(benchmark_name: &str, data: &mut AssociatedData) -> Option<Vec<Run>> {
    let commit_idx = data.commit_map[data.commit];
    let interpolation_entry =
        data.interpolated.entry(data.commit.sha.clone()).or_insert_with(Vec::new);

    let start = if let Some(needle) = data.last_seen_commit[commit_idx].get(benchmark_name) {
        let commit = needle.clone();
        let bench = data.data[&commit].benchmarks[benchmark_name].as_ref().unwrap().clone();
        Some((commit, bench))
    } else {
        None
    };
    let end = if let Some(needle) = data.next_seen_commit[commit_idx].get(benchmark_name) {
        let commit = needle.clone();
        let bench = data.data[&commit].benchmarks[benchmark_name].as_ref().unwrap().clone();
        Some((commit, bench))
    } else {
        None
    };

    match (start, end) {
        // This hole is bounded on both left and
        // right, so we want to linearly interpolate
        // each run between these two data points.
        //
        // This code ignores the case where a run is
        // absent in start or end. This is handled later.
        (Some(start), Some(end)) => {
            let distance = data.commit_map[&end.0] - data.commit_map[&start.0];
            let from_start = commit_idx - data.commit_map[&start.0];
            let start_runs = &start.1.runs;
            let end_runs = &end.1.runs;

            let mut interpolated_runs = Vec::with_capacity(start_runs.len());

            for srun in start_runs {
                for erun in end_runs {
                    // Found pair
                    if srun == erun {
                        let interpolated_stats = interpolate_stats(&srun, &erun, distance, from_start);
                        let mut interpolated_run = srun.clone();
                        interpolated_run.stats = interpolated_stats;
                        interpolated_runs.push(interpolated_run);
                    }
                }
            }

            interpolation_entry.push(
                Interpolation {
                    benchmark: benchmark_name.to_owned(),
                    run: None,
                    from: InterpolationSource::Middle(start.0, end.0),
                });
            return Some(interpolated_runs);
        }

        // This hole is unbounded to the right, so
        // fill in directly with data from the
        // left.
        (Some(start), None) => {
            interpolation_entry.push(
                Interpolation {
                    benchmark: benchmark_name.to_owned(),
                    run: None,
                    from: InterpolationSource::Last(start.0),
                });
            return Some(start.1.runs);
        }

        // This hole is unbounded to the left, so
        // fill in directly with data from the
        // right.
        (None, Some(end)) => {
            interpolation_entry.push(
                Interpolation {
                    benchmark: benchmark_name.to_owned(),
                    run: None,
                    from: InterpolationSource::First(end.0),
                });
            return Some(end.1.runs);
        }

        // No data for this benchmark was found to
        // either side. No data exists for this
        // benchmark. Bail out and return the
        // original (missing) data.
        (None, None) => {
            warn!("giving up on finding {} data for commit {:?}",
                benchmark_name, data.commit);
            return None;
        }
    }

    // we never reach here
}

fn interpolate_stats(srun: &Run, erun: &Run, distance: usize, from_start: usize) -> Vec<Stat> {
    let mut interpolated_stats = Vec::with_capacity(srun.stats.len());
    for sstat in &srun.stats {
        if let Some(estat) = erun.get_stat(&sstat.name) {
            let slope = (estat - sstat.cnt) / (distance as f64);
            let interpolated = slope * (from_start as f64) + sstat.cnt;
            interpolated_stats.push(collector::Stat {
                name: sstat.name.clone(),
                cnt: interpolated,
            });
        }
    }
    interpolated_stats
}
