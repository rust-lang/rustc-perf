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
use std::path::Path;
use std::sync::Arc;

use anyhow::Context;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::db;
use crate::util;
use collector::Bound;
use database::Date;

use crate::api::github;
use collector;
use database::Pool;
pub use database::{ArtifactId, Commit, Crate};

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
    pub benchmarks: Vec<Crate>,
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

#[derive(Debug, Default, Deserialize)]
pub struct Keys {
    pub github: Option<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: Keys,
}

pub struct InputData {
    pub config: Config,

    pub index: ArcSwap<crate::db::Index>,
    pub pool: Pool,
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

    pub fn data_for(&self, is_left: bool, query: Bound) -> Option<ArtifactId> {
        crate::selector::data_for(&self.index.load(), is_left, query)
    }

    pub fn data_range(&self, range: RangeInclusive<Bound>) -> Vec<Commit> {
        crate::selector::range_subset(self.index.load().commits(), range)
    }

    /// Initialize `InputData from the file system.
    pub async fn from_fs(db: &str) -> anyhow::Result<InputData> {
        if Path::new(db).join("times").exists() {
            eprintln!("It looks like you're running the site off of the old data format");
            eprintln!(
                "Please utilize the ingest-json script to convert the data into the new database format."
            );
            eprintln!("This is intended to be a one-time operation; you can delete the JSON fiels once it is complete.");
            eprintln!(
                "    find rustc-timing/times/ -type f | xargs ./target/release/ingest perf-rlo.db finished-files/"
            );
            std::process::exit(1);
        }

        let pool = Pool::open(db);

        let mut conn = pool.connection().await;
        let index = db::Index::load(&mut *conn).await;

        let config = if let Ok(s) = fs::read_to_string("site-config.toml") {
            toml::from_str(&s)?
        } else {
            Config {
                keys: Keys {
                    github: std::env::var("GITHUB_API_TOKEN").ok(),
                    secret: std::env::var("GITHUB_WEBHOOK_SECRET").ok(),
                },
            }
        };

        Ok(InputData {
            config,
            index: ArcSwap::new(Arc::new(index)),
            pool,
        })
    }

    pub async fn conn(&self) -> Box<dyn database::pool::Connection> {
        self.pool.connection().await
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
                if have.contains(&c.sha) {
                    None
                } else {
                    Some((c, MissingReason::Sha))
                }
            })
            .collect::<Vec<_>>();

        let mut commits = self
            .conn()
            .await
            .queued_commits()
            .await
            .into_iter()
            .flat_map(
                |database::QueuedCommit {
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
            .filter(|c| !have.contains(&c.0.sha)) // we may have not updated the try-commits file
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
