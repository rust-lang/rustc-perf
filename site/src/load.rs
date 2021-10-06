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
        parent_sha: String,
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

    /// Returns the not yet tested commits
    pub async fn missing_commits(&self) -> Vec<(Commit, MissingReason)> {
        let conn = self.conn().await;
        let (master_commits, queued_pr_commits, in_progress_artifacts) = futures::join!(
            collector::master_commits(),
            conn.queued_commits(),
            conn.in_progress_artifacts()
        );
        let master_commits = master_commits
            .map_err(|e| anyhow::anyhow!("{:?}", e))
            .context("getting master commit list")
            .unwrap();

        let index = self.index.load();
        let all_commits = index
            .commits()
            .iter()
            .map(|commit| commit.sha.clone())
            .collect::<HashSet<_>>();

        calculate_missing(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
        )
    }
}

/// Calculating the missing commits.
fn calculate_missing(
    master_commits: Vec<collector::MasterCommit>,
    queued_pr_commits: Vec<database::QueuedCommit>,
    in_progress_artifacts: Vec<ArtifactId>,
    all_commits: HashSet<String>,
) -> Vec<(Commit, MissingReason)> {
    calculate_missing_from(
        master_commits,
        queued_pr_commits,
        in_progress_artifacts,
        all_commits,
        Utc::now(),
    )
}

/// Calculate the missing commits filtering out any that are 29 days or older than the supplied time.
///
/// This is used by `calculate_missing` is exists as a separate function for testing purposes.
fn calculate_missing_from(
    master_commits: Vec<collector::MasterCommit>,
    queued_pr_commits: Vec<database::QueuedCommit>,
    in_progress_artifacts: Vec<ArtifactId>,
    mut all_commits: HashSet<String>,
    time: chrono::DateTime<chrono::Utc>,
) -> Vec<(Commit, MissingReason)> {
    let mut master_commits = master_commits
        .into_iter()
        .filter(|c| time.signed_duration_since(c.time) < Duration::days(29))
        .map(|c| {
            (
                Commit {
                    sha: c.sha,
                    date: Date(c.time),
                },
                // All recent master commits should have an associated PR
                MissingReason::Master {
                    pr: c.pr.unwrap_or(0),
                    parent_sha: c.parent_sha,
                },
            )
        })
        .collect::<Vec<_>>();
    master_commits.reverse();
    let mut missing = Vec::with_capacity(queued_pr_commits.len() * 2 + master_commits.len());
    for database::QueuedCommit {
        sha,
        parent_sha,
        pr,
        include,
        exclude,
        runs,
    } in queued_pr_commits
        .into_iter()
        // filter out any queued PR master commits (leaving only try commits)
        .filter(|c| !master_commits.iter().any(|(mc, _)| mc.sha == c.sha))
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

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "collector::round_float")] pub f64);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use collector::MasterCommit;
    use database::QueuedCommit;

    use super::*;
    #[test]
    fn calculates_missing_correct() {
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![
            // A not yet tested commit
            MasterCommit {
                sha: "123".into(),
                parent_sha: "345".into(),
                pr: Some(11),
                time,
            },
            // An already tested commit
            MasterCommit {
                sha: "abc".into(),
                parent_sha: "def".into(),
                pr: Some(90),
                time,
            },
            // A queued PR commit
            MasterCommit {
                sha: "foo".into(),
                parent_sha: "bar".into(),
                pr: Some(77),
                time,
            },
        ];
        let queued_pr_commits = vec![
            // A master commit
            QueuedCommit {
                sha: "foo".into(),
                parent_sha: "bar".into(),
                pr: 77,
                include: None,
                exclude: None,
                runs: None,
            },
            // A try run
            QueuedCommit {
                sha: "baz".into(),
                parent_sha: "foo".into(),
                pr: 101,
                include: None,
                exclude: None,
                runs: None,
            },
        ];
        let in_progress_artifacts = vec![];
        let mut all_commits = HashSet::new();
        all_commits.insert(master_commits[1].sha.clone());

        let expected = vec![
            (
                Commit {
                    sha: "foo".into(),
                    date: database::Date(time),
                },
                MissingReason::TryParent,
            ),
            (
                Commit {
                    sha: "baz".into(),
                    date: database::Date(time),
                },
                MissingReason::Try {
                    pr: 101,
                    include: None,
                    exclude: None,
                    runs: None,
                },
            ),
            (
                Commit {
                    sha: "123".into(),
                    date: database::Date(time),
                },
                MissingReason::Master {
                    pr: 11,
                    parent_sha: "345".into(),
                },
            ),
        ];
        assert_eq!(
            expected,
            calculate_missing_from(
                master_commits,
                queued_pr_commits,
                in_progress_artifacts,
                all_commits,
                time
            )
        );
    }
}
