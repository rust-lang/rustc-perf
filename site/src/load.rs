// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::io::Read;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use chrono::{Duration, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::git;
use crate::util;
use collector::{Bound, Date};

use crate::api::github;
use crate::db::{ArtifactData, CommitData};
use crate::selector::PathComponent;
use collector;
pub use collector::{BenchmarkName, Commit, Patch, Sha, StatId, Stats};
use log::{error, info, warn};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MissingReason {
    /// This commmit has not yet been benchmarked
    Sha,
    TryParent,
    TryCommit,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CurrentState {
    pub commit: Commit,
    pub issue: Option<github::Issue>,
    pub benchmarks: Vec<BenchmarkName>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct TryCommit {
    pub sha: String,
    pub parent_sha: String,
    pub issue: github::Issue,
}

impl TryCommit {
    pub fn sha(&self) -> &str {
        self.sha.as_str()
    }

    pub fn comparison_url(&self) -> String {
        format!(
            "https://perf.rust-lang.org/compare.html?start={}&end={}",
            self.parent_sha, self.sha
        )
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Persistent {
    pub try_commits: Vec<TryCommit>,
    pub current: Option<CurrentState>,
    // this is a list of pr numbers for which we expect to run
    // a perf build once the try build completes.
    // This only persists for one try build (so should not be long at any point).
    #[serde(default)]
    pub pending_try_builds: HashSet<u32>,
    // Set of commit hashes for which we've completed benchmarking.
    #[serde(default)]
    pub posted_ends: Vec<Sha>,
}

lazy_static::lazy_static! {
    static ref PERSISTENT_PATH: &'static Path = Path::new("persistent.json");
}

impl Persistent {
    pub fn write(&self) -> anyhow::Result<()> {
        if PERSISTENT_PATH.exists() {
            let _ = fs::copy(&*PERSISTENT_PATH, "persistent.json.previous");
        }
        let s = serde_json::to_string(self)?;
        fs::write(&*PERSISTENT_PATH, &s)
            .with_context(|| format!("failed to write persistent DB"))?;
        Ok(())
    }

    fn load() -> Persistent {
        let p = Persistent::load_().unwrap_or_else(|| Persistent {
            try_commits: Vec::new(),
            current: None,
            pending_try_builds: HashSet::new(),
            posted_ends: Vec::new(),
        });
        p.write().unwrap();
        p
    }

    fn load_() -> Option<Persistent> {
        let s = fs::read_to_string(&*PERSISTENT_PATH).ok()?;
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
    pub keys: Keys,
    #[serde(default)]
    pub skip: HashSet<Sha>,
}

pub struct InputData {
    /// All known statistics gathered for crates
    pub stats_list: Vec<&'static str>,

    pub all_paths: Vec<crate::selector::Path>,

    data: Vec<Arc<CommitData>>,

    pub commits: Vec<Commit>,

    /// This is only for the last commit, with Some(..) if the benchmark
    /// errored.
    ///
    /// Names are unique and sorted.
    pub errors: Vec<(BenchmarkName, Option<String>)>,

    pub artifact_data: Vec<ArtifactData>,

    pub persistent: Mutex<Persistent>,

    pub config: Config,

    pub index: crate::db::Index,
    pub db: rocksdb::DB,
}

impl InputData {
    pub fn summary_patches(&self) -> Vec<crate::db::Cache> {
        vec![
            crate::db::Cache::Empty,
            crate::db::Cache::IncrementalEmpty,
            crate::db::Cache::IncrementalFresh,
            crate::db::Cache::IncrementalPatch("println".into()),
        ]
    }

    pub fn data(&self) -> &[Arc<CommitData>] {
        &self.data
    }

    pub fn data_for(&self, is_left: bool, query: Bound) -> Option<Commit> {
        crate::db::data_for(&self.commits, is_left, query)
    }

    pub fn data_range(&self, range: RangeInclusive<Bound>) -> &[Commit] {
        crate::db::range_subset(&self.commits, range)
    }

    /// Initialize `InputData from the file system.
    pub fn from_fs(repo_loc: &str) -> anyhow::Result<InputData> {
        let repo_loc = PathBuf::from(repo_loc);
        let mut artifact_data = HashMap::new();
        let mut data = Vec::new();
        let mut commits = HashSet::new();

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
                    "https://github.com/rust-lang/rustc-timing.git",
                    repo_loc.to_str().unwrap(),
                ],
            )?;
        }

        eprintln!("Loading files from directory...");

        // Read all files from repo_loc/processed
        let latest_section_start = ::std::time::Instant::now();
        let mut file_contents = Vec::new();
        for entry in fs::read_dir(repo_loc.join("times"))? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }
            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();
            let mut file = fs::File::open(entry.path())
                .with_context(|| format!("Failed to open {}", entry.path().display()))?;
            file_contents.truncate(0);
            if filename.ends_with(".sz") {
                let mut szip_reader = snap::read::FrameDecoder::new(std::io::BufReader::new(file));
                szip_reader
                    .read_to_end(&mut file_contents)
                    .with_context(|| format!("Failed to read {}", entry.path().display()))?;
            } else {
                file.read_to_end(&mut file_contents)
                    .with_context(|| format!("Failed to read {}", entry.path().display()))?;
            };
            let file_contents = std::str::from_utf8(&file_contents).unwrap();

            if filename.starts_with("artifact-") {
                let contents: ArtifactData = match serde_json::from_str(&file_contents) {
                    Ok(j) => j,
                    Err(err) => {
                        error!("Failed to parse JSON for {}: {:?}", filename, err);
                        continue;
                    }
                };
                if contents.benchmarks.is_empty() {
                    warn!("empty benchmarks hash for {}", filename);
                    continue;
                }

                artifact_data.insert(contents.id.clone(), contents);
            } else {
                let contents: CommitData = match serde_json::from_str(&file_contents) {
                    Ok(json) => json,
                    Err(err) => {
                        error!("Failed to parse JSON for {}: {:?}", filename, err);
                        continue;
                    }
                };
                if contents.benchmarks.is_empty() {
                    warn!("empty benchmarks hash for {}", filename);
                    continue;
                }

                if commits.insert(contents.commit.clone()) {
                    data.push(Arc::new(contents));
                }
            }
        }
        std::mem::drop(file_contents);

        eprintln!(
            "{} commits/artifacts loaded in {:?}",
            data.len(),
            latest_section_start.elapsed()
        );

        let config = if let Ok(s) = fs::read_to_string("site-config.toml") {
            toml::from_str(&s)?
        } else {
            Config {
                keys: Keys::default(),
                skip: HashSet::default(),
            }
        };

        data.sort_unstable_by_key(|d| d.commit.clone());
        InputData::new(data, artifact_data, config)
    }

    pub fn new(
        data: Vec<Arc<CommitData>>,
        mut artifact_data: HashMap<String, ArtifactData>,
        config: Config,
    ) -> anyhow::Result<InputData> {
        let commits = data.iter().map(|cd| cd.commit.clone()).collect::<Vec<_>>();
        let errors = data
            .last()
            .unwrap()
            .benchmarks
            .iter()
            .map(|(name, res)| (*name, res.as_ref().err().cloned()))
            .collect::<Vec<_>>();
        let mut stats_list = BTreeSet::new();

        for commit_data in data.iter() {
            let benchmarks = commit_data
                .benchmarks
                .values()
                .filter_map(|v| v.as_ref().ok());
            for benchmark in benchmarks {
                for run in &benchmark.runs {
                    for (stat, _) in run.stats.iter() {
                        stats_list.insert(stat.as_str());
                    }
                }
            }
        }

        let mut data_commits = Vec::with_capacity(data.len());
        for cd in data.iter() {
            data_commits.push(cd.commit);
        }

        let mut versions = artifact_data.keys().cloned().collect::<Vec<_>>();
        versions.sort_by(|a, b| {
            match (
                a.parse::<semver::Version>().ok(),
                b.parse::<semver::Version>().ok(),
            ) {
                (Some(a), Some(b)) => a.cmp(&b),
                (_, _) => {
                    if a == "beta" {
                        std::cmp::Ordering::Greater
                    } else if b == "beta" {
                        std::cmp::Ordering::Less
                    } else {
                        panic!("unexpected version")
                    }
                }
            }
        });

        let artifact_data = versions
            .into_iter()
            .map(|v| artifact_data.remove(&v).unwrap())
            .collect::<Vec<_>>();

        let all_paths = data
            .iter()
            .flat_map(|cd| {
                cd.benchmarks
                    .values()
                    .filter_map(|b| b.as_ref().ok())
                    .flat_map(|bench| {
                        bench.runs.iter().flat_map(move |r| {
                            r.stats
                                .iter()
                                .map(move |(stat, _)| {
                                    crate::selector::Path::new()
                                        .set(PathComponent::Crate(bench.name))
                                        .set(PathComponent::Profile(r.profile))
                                        .set(PathComponent::Cache(r.state))
                                        .set(PathComponent::ProcessStatistic(stat.as_pstat()))
                                })
                                .chain(std::iter::once(
                                    crate::selector::Path::new()
                                        .set(PathComponent::Crate(bench.name))
                                        .set(PathComponent::Profile(r.profile))
                                        .set(PathComponent::Cache(r.state)),
                                ))
                        })
                    })
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let db = crate::db::open("data", false);
        let persistent = Persistent::load();
        Ok(InputData {
            stats_list: stats_list.into_iter().collect(),
            all_paths,
            data,
            commits,
            errors,
            artifact_data,
            persistent: Mutex::new(persistent),
            config,
            index: crate::db::Index::load(&db),
            db,
        })
    }

    pub async fn missing_commits(&self) -> Vec<(Commit, MissingReason)> {
        if self.config.keys.github.is_none() {
            println!("Skipping collection of missing commits, no github token configured");
            return Vec::new();
        }
        let commits = rustc_artifacts::master_commits()
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))
            .context("getting master commit list")
            .unwrap();

        let have = self
            .commits
            .iter()
            .map(|commit| commit.sha.clone())
            .collect::<HashSet<_>>();
        let now = Utc::now();
        let missing = commits
            .iter()
            .cloned()
            .filter(|c| now.signed_duration_since(c.time) < Duration::days(29))
            .filter_map(|c| {
                let sha = c.sha.as_str().into();
                if have.contains(&sha) || self.config.skip.contains(&sha) {
                    None
                } else {
                    Some((c, MissingReason::Sha))
                }
            })
            .collect::<Vec<_>>();

        let mut commits = self
            .persistent
            .lock()
            .try_commits
            .iter()
            .flat_map(
                |TryCommit {
                     sha, parent_sha, ..
                 }| {
                    let mut ret = Vec::new();
                    // Enqueue the `TryParent` commit before the `TryCommit` itself, so that
                    // all of the `try` run's data is complete when the benchmark results
                    // of that commit are available.
                    if let Some(commit) = commits.iter().find(|c| c.sha == *parent_sha.as_str()) {
                        ret.push((commit.clone(), MissingReason::TryParent));
                    } else {
                        // could not find parent SHA
                        // Unfortunately this just means that the parent commit is older than 168
                        // days for the most part so we don't have artifacts for it anymore anyway;
                        // in that case, just ignore this "error".
                    }
                    ret.push((
                        rustc_artifacts::Commit {
                            sha: sha.to_string(),
                            time: Date::ymd_hms(2001, 01, 01, 0, 0, 0).0,
                        },
                        MissingReason::TryCommit,
                    ));
                    ret
                },
            )
            .filter(|c| !have.contains(&c.0.sha.as_str().into())) // we may have not updated the try-commits file
            .chain(missing)
            .collect::<Vec<_>>();

        let mut seen = HashSet::with_capacity(commits.len());

        // FIXME: replace with Vec::drain_filter when it stabilizes
        let mut i = 0;
        while i != commits.len() {
            if !seen.insert(commits[i].0.sha.clone()) {
                commits.remove(i);
            } else {
                i += 1;
            }
        }

        commits
            .into_iter()
            .map(|(c, mr)| {
                (
                    Commit {
                        sha: c.sha.as_str().into(),
                        date: Date(c.time),
                    },
                    mr,
                )
            })
            .collect()
    }
}

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "util::round_float")] pub f64);
