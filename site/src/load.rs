// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use arc_swap::ArcSwap;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use chrono::{Duration, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::util;
use collector::{Bound, Date};

use crate::api::github;
use collector;
pub use collector::{BenchmarkName, Commit, Patch, Sha, StatId, Stats};

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
    static ref PERSISTENT_PATH: PathBuf = std::env::var_os("PERSISTENT_PATH").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("persistent.json")
    });
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
    pub persistent: Mutex<Persistent>,

    pub config: Config,

    pub index: ArcSwap<crate::db::Index>,
    pub conn: r2d2::Pool<db::pool::sqlite::Sqlite>,
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

    pub fn data_for(&self, is_left: bool, query: Bound) -> Option<Commit> {
        crate::db::data_for(&self.index.load().commits(), is_left, query)
    }

    pub fn data_range(&self, range: RangeInclusive<Bound>) -> Vec<Commit> {
        crate::db::range_subset(self.index.load().commits(), range)
    }

    /// Initialize `InputData from the file system.
    pub async fn from_fs(db: &str) -> anyhow::Result<InputData> {
        if Path::new(db).join("times").exists() {
            eprintln!("It looks like you're running the site off of the old data format");
            eprintln!(
                "Please run the ingestion script pointing at a different directory, like so:"
            );
            eprintln!(
                "    find rustc-timing/times/ -type f | xargs ./target/release/ingest perf-rlo.db"
            );
            eprintln!();
            eprintln!("You can run the ingestion script repeatedly over all the files,");
            eprintln!("or you can run it on just some newly collected data.");
            std::process::exit(1);
        }

        let pool = r2d2::Pool::builder()
            .max_size(16)
            .connection_timeout(std::time::Duration::from_secs(1))
            .build(db::pool::sqlite::Sqlite::new(db.into()))
            .unwrap();

        let mut conn = db::pool::sqlite::SqliteConnection::new(pool.get().unwrap());
        let index = db::Index::load(&mut conn).await;

        let config = if let Ok(s) = fs::read_to_string("site-config.toml") {
            toml::from_str(&s)?
        } else {
            Config {
                keys: Keys {
                    github: std::env::var("GITHUB_API_TOKEN").ok(),
                    secret: std::env::var("GITHUB_WEBHOOK_SECRET").ok(),
                },
                skip: HashSet::default(),
            }
        };

        let persistent = Persistent::load();
        Ok(InputData {
            persistent: Mutex::new(persistent),
            config,
            index: ArcSwap::new(Arc::new(index)),
            conn: pool,
        })
    }

    pub fn conn(&self) -> Box<dyn db::pool::Connection> {
        Box::new(db::pool::sqlite::SqliteConnection::new(
            self.conn.get().unwrap(),
        ))
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
            .index
            .load()
            .commits()
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
