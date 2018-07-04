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

use util;
use git;
use collector::Date;

pub use collector::{Commit, CommitData, ArtifactData, Patch, Run, Stat};

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

    pub data: BTreeMap<Commit, CommitData>,

    pub artifact_data: BTreeMap<String, ArtifactData>,

    pub commits: Vec<GitCommit>,

    pub persistent: Mutex<Persistent>,
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

        InputData::new(data, artifact_data)
    }

    pub fn new(
        data: BTreeMap<Commit, CommitData>,
        artifact_data: BTreeMap<String, ArtifactData>,
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
        let commits = rust_sysroot::get_commits(rust_sysroot::EPOCH_COMMIT, "master").map_err(SyncFailure::new)?;

        Ok(InputData {
            crate_list: crate_list,
            stats_list: stats_list,
            last_date: last_date,
            data: data,
            artifact_data,
            commits,
            persistent: Mutex::new(Persistent::load()),
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
