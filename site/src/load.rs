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
use collector::Bound;
use database::Date;

use crate::api::github;
use collector;
use database::Pool;
pub use database::{ArtifactId, Benchmark, Commit};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MissingReason {
    /// This commmit has not yet been benchmarked
    Master {
        pr: u32,
    },
    TryParent,
    Try {
        pr: u32,
        include: Option<String>,
        exclude: Option<String>,
        runs: Option<i32>,
    },
    InProgress(Option<Box<MissingReason>>),
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

/// Keys for accessing various services
///
/// At the moment only used for accessing GitHub
#[derive(Debug, Default, Deserialize)]
pub struct Keys {
    /// GitHub API token from the `GITHUB_API_TOKEN` env variable
    #[serde(rename = "github")]
    pub github_api_token: Option<String>,
    /// GitHub webhook secret from the `GITHUB_WEBHOOK_SECRET` env variable
    #[serde(rename = "secret")]
    pub github_webhook_secret: Option<String>,
}

/// Site configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub keys: Keys,
}

/// Site context object that contains global data
pub struct SiteCtxt {
    /// Site configuration
    pub config: Config,
    /// Cached site landing page
    pub landing_page: ArcSwap<Option<Arc<crate::api::graph::Response>>>,
    /// Index of various common queries
    pub index: ArcSwap<crate::db::Index>,
    /// Database connection pool
    pub pool: Pool,
}

impl SiteCtxt {
    pub fn summary_scenarios(&self) -> Vec<crate::db::Scenario> {
        vec![
            crate::db::Scenario::Empty,
            crate::db::Scenario::IncrementalEmpty,
            crate::db::Scenario::IncrementalFresh,
            crate::db::Scenario::IncrementalPatch("println".into()),
        ]
    }

    pub fn artifact_id_for_bound(&self, query: Bound, is_left: bool) -> Option<ArtifactId> {
        crate::selector::artifact_id_for_bound(&self.index.load(), query, is_left)
    }

    pub fn data_range(&self, range: RangeInclusive<Bound>) -> Vec<Commit> {
        crate::selector::range_subset(self.index.load().commits(), range)
    }

    /// Initialize `SiteCtxt` from database url
    pub async fn from_db_url(db_url: &str) -> anyhow::Result<Self> {
        if Path::new(db_url).join("times").exists() {
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

        let pool = Pool::open(db_url);

        let mut conn = pool.connection().await;
        let index = db::Index::load(&mut *conn).await;

        let config = if let Ok(s) = fs::read_to_string("site-config.toml") {
            toml::from_str(&s)?
        } else {
            Config {
                keys: Keys {
                    github_api_token: std::env::var("GITHUB_API_TOKEN").ok(),
                    github_webhook_secret: std::env::var("GITHUB_WEBHOOK_SECRET").ok(),
                },
            }
        };

        Ok(Self {
            config,
            index: ArcSwap::new(Arc::new(index)),
            pool,
            landing_page: ArcSwap::new(Arc::new(None)),
        })
    }

    pub async fn conn(&self) -> Box<dyn database::pool::Connection> {
        self.pool.connection().await
    }

    pub async fn missing_commits(&self) -> Vec<(Commit, MissingReason)> {
        let conn = self.conn().await;
        let (master_commits, queued_try_commits, in_progress_artifacts) = futures::join!(
            collector::master_commits(),
            conn.queued_commits(),
            conn.in_progress_artifacts()
        );
        let master_commits = master_commits
            .map_err(|e| anyhow::anyhow!("{:?}", e))
            .context("getting master commit list")
            .unwrap();

        let index = self.index.load();
        let mut all_commits = index
            .commits()
            .iter()
            .map(|commit| commit.sha.clone())
            .collect::<HashSet<_>>();

        let now = Utc::now();
        let mut master_commits = master_commits
            .into_iter()
            .filter(|c| now.signed_duration_since(c.time) < Duration::days(29))
            .map(|c| {
                (
                    Commit {
                        sha: c.sha,
                        date: Date(c.time),
                    },
                    // All recent master commits should have an associated PR
                    MissingReason::Master {
                        pr: c.pr.unwrap_or(0),
                    },
                )
            })
            .collect::<Vec<_>>();
        master_commits.reverse();

        let mut missing = Vec::with_capacity(queued_try_commits.len() * 2 + master_commits.len()); // Two commits per every try commit and all master commits
        for database::QueuedCommit {
            sha,
            parent_sha,
            pr,
            include,
            exclude,
            runs,
        } in queued_try_commits
        {
            // Enqueue the `TryParent` commit before the `TryCommit` itself, so that
            // all of the `try` run's data is complete when the benchmark results
            // of that commit are available.
            if let Some((try_parent, _)) = master_commits
                .iter()
                .find(|(m, _)| m.sha == parent_sha.as_str())
            {
                missing.push((try_parent.clone(), MissingReason::TryParent));
            }
            missing.push((
                Commit {
                    sha: sha.to_string(),
                    date: Date::ymd_hms(2001, 01, 01, 0, 0, 0),
                },
                MissingReason::Try {
                    pr,
                    include,
                    exclude,
                    runs,
                },
            ));
        }
        missing.extend(master_commits);

        for aid in in_progress_artifacts {
            match aid {
                ArtifactId::Commit(c) => {
                    let previous = missing
                        .iter()
                        .find(|(i, _)| i.sha == c.sha)
                        .map(|v| Box::new(v.1.clone()));
                    all_commits.remove(&c.sha);
                    missing.insert(0, (c, MissingReason::InProgress(previous)));
                }
                ArtifactId::Tag(_) => {
                    // do nothing, for now, though eventually we'll want an artifact queue
                }
            }
        }

        let mut already_tested = HashSet::with_capacity(all_commits.len());
        already_tested.extend(all_commits);

        // Remove commits from missing that have already been tested
        // FIXME: replace with Vec::drain_filter when it stabilizes
        let mut i = 0;
        while i != missing.len() {
            if !already_tested.insert(missing[i].0.sha.clone()) {
                missing.remove(i);
            } else {
                i += 1;
            }
        }

        missing
    }
}

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "collector::round_float")] pub f64);
