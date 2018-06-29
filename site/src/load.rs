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
use git;
use collector::Date;

pub use collector::{Benchmark, CommitData, Commit, ArtifactData, Patch, Run, Stat};
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
    pub from: InterpolationSource,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CurrentState {
    pub commit: Commit,
    pub benchmarks: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Persistent {
    pub try_commits: Vec<String>,
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

    /// `data_real` has holes, `data` does not.
    pub data_real: BTreeMap<Commit, CommitData>,
    pub data: BTreeMap<Commit, CommitData>,

    /// The benchmarks we interpolated for a given commit.
    ///
    /// Not all commits are in this map.
    pub interpolated: BTreeMap<Commit, Vec<Interpolation>>,

    /// The list of commits in the `data` map.
    pub commits: Vec<Commit>,
    /// A map from commit to index in the `commits` vector.
    pub commit_map: HashMap<Commit, usize>,

    pub artifact_data: BTreeMap<String, ArtifactData>,

    pub commits: Vec<GitCommit>,

    pub persistent: Mutex<Persistent>,

    pub config: Config,
}

impl InputData {
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

        let commits = data.keys().cloned().collect::<Vec<_>>();

        let mut commit_map = HashMap::with_capacity(commits.len());
        for (idx, commit) in commits.iter().enumerate() {
            commit_map.insert(commit.clone(), idx);
        }

        let data_real = data.clone();

        let mut interpolated = BTreeMap::new();
        let mut data_next = BTreeMap::new();
        for (commit, cd) in data {
            let commit_idx = commit_map[&commit];

            let benchmarks = cd.benchmarks.into_iter()
                .map(|(name, res)| {
                    let benchmark = if let Ok(b) = res {
                        b
                    } else {
                        // We do not interpolate try commits today
                        // because we don't track their parents so it's
                        // difficult to add that data in.
                        if commit.date.0.year() == 2000 {
                            return (name, res);
                        }
                        let mut interpolation_entry =
                            interpolated.entry(commit.clone()).or_insert_with(Vec::new);

                        let mut start = None;
                        let mut end = None;
                        for needle_commit in commits[..commit_idx].iter().rev() {
                            let bench = data_real[needle_commit].benchmarks.get(&name).cloned();
                            if let Some(Ok(bench)) = bench {
                                start = Some((needle_commit.clone(), bench));
                                break;
                            }
                        }
                        for needle_commit in commits[commit_idx + 1..].iter() {
                            let bench = data_real[needle_commit].benchmarks.get(&name).cloned();
                            if let Some(Ok(bench)) = bench {
                                end = Some((needle_commit.clone(), bench));
                                break;
                            }
                        }

                        match (start, end) {
                            // This hole is bounded on both left and
                            // right, so we want to linearly interpolate
                            // each run between these two data points.
                            //
                            // This code handles the case where a run is
                            // absent in start or end by
                            // flat-interpolating that run through this
                            // region.
                            //
                            // Essentially, we apply the same algorithm
                            // again, but instead of applying it to
                            // benchmark names we apply it to run names.
                            (Some(start), Some(end)) => {
                                let distance = commit_map[&end.0] - commit_map[&start.0];
                                let from_start = commit_idx - commit_map[&start.0];
                                let start_runs = &start.1.runs;
                                let end_runs = &end.1.runs;

                                let mut run_pairs = Vec::with_capacity(start_runs.len());

                                for srun in start_runs {
                                    for erun in end_runs {
                                        // Found pair
                                        if srun.name() == erun.name() {
                                            run_pairs.push((srun.clone(), erun.clone()));
                                        }
                                    }
                                }

                                let mut interpolated_runs = Vec::with_capacity(run_pairs.len());
                                for (srun, erun) in run_pairs {
                                    let mut stat_pairs = Vec::with_capacity(srun.stats.len());
                                    for sstat in &srun.stats {
                                        for estat in &erun.stats {
                                            if sstat.name == estat.name {
                                                stat_pairs.push((sstat.clone(), estat.clone()));
                                            }
                                        }
                                    }
                                    let mut interpolated_stats = Vec::with_capacity(stat_pairs.len());
                                    for (sstat, estat) in stat_pairs {
                                        let slope = (estat.cnt - sstat.cnt) / (distance as f64);
                                        let interpolated = slope * (from_start as f64) + sstat.cnt;
                                        trace!("
                                            stat {} went from {} to {} over {},
                                            slope = {},
                                            interpolated = {}; from_start = {}",
                                            sstat.name, sstat.cnt, estat.cnt,
                                            distance, slope, interpolated, from_start);
                                        interpolated_stats.push(collector::Stat {
                                            name: sstat.name.clone(),
                                            cnt: interpolated,
                                        });
                                    }
                                    let mut interpolated_run = srun.clone();
                                    interpolated_run.stats = interpolated_stats;
                                    interpolated_runs.push(interpolated_run);
                                }
                                interpolation_entry.push(
                                    Interpolation {
                                        benchmark: name.clone(),
                                        from: InterpolationSource::Middle(start.0.clone(), end.0.clone()),
                                    });
                                return (start.1.name.clone(), Ok(Benchmark {
                                    runs: interpolated_runs,
                                    name: start.1.name.clone(),
                                }));
                            }

                            // This hole is unbounded to the right, so
                            // fill in directly with data from the
                            // left.
                            (Some(start), None) => {
                                interpolation_entry.push(
                                    Interpolation {
                                        benchmark: name.clone(),
                                        from: InterpolationSource::Last(start.0),
                                    });
                                return (name, Ok(start.1));
                            }

                            // This hole is unbounded to the left, so
                            // fill in directly with data from the
                            // right.
                            (None, Some(end)) => {
                                interpolation_entry.push(
                                    Interpolation {
                                        benchmark: name.clone(),
                                        from: InterpolationSource::First(end.0),
                                    });
                                return (name, Ok(end.1));
                            }

                            // No data for this benchmark was found to
                            // either side. No data exists for this
                            // benchmark. Bail out and return the
                            // original (missing) data.
                            (None, None) => {
                                trace!("giving up on finding {} data for commit {:?}",
                                    name, commit);
                                return (name, res);
                            }
                        }

                        // we never reach here
                    };

                    (name, Ok(benchmark))
                }).collect::<BTreeMap<String, Result<Benchmark, String>>>();
            data_next.insert(commit, CommitData {
                commit: cd.commit.clone(),
                triple: cd.triple.clone(),
                benchmarks: benchmarks,
            });
        }

        let interpolated = interpolated.into_iter()
            .filter(|(_, v)| !v.is_empty())
            .collect::<BTreeMap<_, _>>();

        info!("interpolated {} commits", interpolated.len());
        let data = data_next;

        Ok(InputData {
            crate_list: crate_list,
            stats_list: stats_list,
            commits: commits,
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
                        .any(|b| !cd.benchmarks.contains_key(&**b))
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();
        missing.reverse();
        Ok(self.persistent.lock().try_commits.iter()
            .map(|sha| Commit { sha: sha.clone(), date: Date::ymd_hms(2001, 01, 01, 0, 0, 0) })
            .filter(|c| !have.contains_key(&c.sha)) // we may have not updated the try-commits file
            .chain(missing)
            .collect())
    }
}

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "util::round_float")] pub f64);
