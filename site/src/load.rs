use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::Instant;

use arc_swap::{ArcSwap, Guard};
use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use log::error;
use parking_lot::Mutex;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::self_profile::SelfProfileCache;
use collector::compile::benchmark::category::Category;
use collector::{Bound, MasterCommit};
use database::Pool;
pub use database::{ArtifactId, Benchmark, Commit};
use database::{CommitType, Date};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MissingReason {
    /// This commmit has not yet been benchmarked
    Master {
        pr: u32,
        parent_sha: String,
        is_try_parent: bool,
    },
    Try {
        pr: u32,
        parent_sha: String,
        include: Option<String>,
        exclude: Option<String>,
        runs: Option<i32>,
    },
    InProgress(Option<Box<MissingReason>>),
}

impl MissingReason {
    fn pr(&self) -> Option<u32> {
        let mut this = self;
        loop {
            match this {
                MissingReason::Master { pr, .. } => return Some(*pr),
                MissingReason::Try { pr, .. } => return Some(*pr),
                MissingReason::InProgress(Some(s)) => this = s,
                MissingReason::InProgress(None) => return None,
            }
        }
    }
    fn parent_sha(&self) -> Option<&str> {
        let mut this = self;
        loop {
            match this {
                MissingReason::Master { parent_sha, .. } => return Some(parent_sha.as_str()),
                MissingReason::Try { parent_sha, .. } => return Some(parent_sha.as_str()),
                MissingReason::InProgress(Some(s)) => this = s,
                MissingReason::InProgress(None) => return None,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryCommit {
    pub sha: String,
    pub parent_sha: String,
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

#[derive(Debug)]
pub struct MasterCommitCache {
    pub commits: Vec<MasterCommit>,
    pub updated: Instant,
}

impl MasterCommitCache {
    /// Download the master-branch Rust commit list
    pub async fn download() -> anyhow::Result<Self> {
        let commits = collector::master_commits().await?;
        Ok(Self {
            commits,
            updated: Instant::now(),
        })
    }
}

// How many analyzed self profiles should be stored in memory
const CACHED_SELF_PROFILE_COUNT: usize = 1000;

/// Site context object that contains global data
pub struct SiteCtxt {
    /// Site configuration
    pub config: Config,
    /// Cached site landing page
    pub landing_page: ArcSwap<Option<Arc<crate::api::graphs::Response>>>,
    /// Index of various common queries
    pub index: ArcSwap<database::Index>,
    /// Cached master-branch Rust commits
    pub master_commits: Arc<ArcSwap<MasterCommitCache>>, // outer Arc enables mutation in background task
    /// Cache for self profile data
    pub self_profile_cache: Mutex<SelfProfileCache>,
    /// Database connection pool
    pub pool: Pool,
}

impl SiteCtxt {
    pub fn summary_scenarios(&self) -> Vec<database::Scenario> {
        vec![
            database::Scenario::Empty,
            database::Scenario::IncrementalEmpty,
            database::Scenario::IncrementalFresh,
            database::Scenario::IncrementalPatch("println".into()),
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
        let pool = Pool::open(db_url);

        let mut conn = pool.connection().await;
        let index = database::Index::load(&mut *conn).await;

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

        let master_commits = MasterCommitCache::download().await?;

        Ok(Self {
            config,
            index: ArcSwap::new(Arc::new(index)),
            master_commits: Arc::new(ArcSwap::new(Arc::new(master_commits))),
            pool,
            landing_page: ArcSwap::new(Arc::new(None)),
            self_profile_cache: Mutex::new(SelfProfileCache::new(CACHED_SELF_PROFILE_COUNT)),
        })
    }

    pub async fn conn(&self) -> Box<dyn database::pool::Connection> {
        self.pool.connection().await
    }

    /// Returns the not yet tested commits
    pub async fn missing_commits(&self) -> Vec<(Commit, MissingReason)> {
        let conn = self.conn().await;
        let (queued_pr_commits, in_progress_artifacts) =
            futures::join!(conn.queued_commits(), conn.in_progress_artifacts());
        let master_commits = &self.get_master_commits().commits;

        let index = self.index.load();
        let all_commits = index
            .commits()
            .iter()
            .map(|commit| commit.sha.clone())
            .collect::<HashSet<_>>();

        calculate_missing(
            master_commits.clone(),
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
        )
    }

    /// Returns the not yet tested published artifacts, sorted from newest to oldest.
    pub async fn missing_published_artifacts(&self) -> anyhow::Result<Vec<String>> {
        let artifact_list: String = reqwest::get("https://static.rust-lang.org/manifests.txt")
            .await?
            .text()
            .await?;

        lazy_static! {
            static ref VERSION_REGEX: Regex = Regex::new(r"(\d+\.\d+.\d+)").unwrap();
        }

        let conn = self.conn().await;

        let index = self.index.load();
        let tested_artifacts: HashSet<_> = index.artifacts().collect();
        let in_progress_tagged_artifacts: HashSet<_> = conn
            .in_progress_artifacts()
            .await
            .into_iter()
            .filter_map(|artifact| match artifact {
                ArtifactId::Commit(_) => None,
                ArtifactId::Tag(tag) => Some(tag),
            })
            .collect();

        // Gather at most last 20 published artifacts that are not yet tested and
        // are not in progress.
        let artifacts: Vec<_> = artifact_list
            .lines()
            .rev()
            .filter_map(parse_published_artifact_tag)
            .take(20)
            .filter(|artifact| {
                !tested_artifacts.contains(artifact.as_str())
                    && !in_progress_tagged_artifacts.contains(artifact.as_str())
            })
            .collect();

        Ok(artifacts)
    }

    pub async fn get_benchmark_category_map(&self) -> HashMap<Benchmark, Category> {
        let benchmarks = self.pool.connection().await.get_compile_benchmarks().await;
        benchmarks
            .into_iter()
            .map(|bench| {
                (
                    bench.name.as_str().into(),
                    Category::from_db_representation(&bench.category).unwrap(),
                )
            })
            .collect()
    }

    /// Get cached master-branch Rust commits.  
    /// Returns cached results immediately, but if the cached value is older than one minute,
    /// updates in a background task for next time.
    pub fn get_master_commits(&self) -> Guard<Arc<MasterCommitCache>> {
        let commits = self.master_commits.load();

        if commits.updated.elapsed() > std::time::Duration::from_secs(60) {
            let master_commits = self.master_commits.clone();
            tokio::task::spawn(async move {
                // if another update happens before this one is done, we will download the data twice, but that's it
                match MasterCommitCache::download().await {
                    Ok(commits) => master_commits.store(Arc::new(commits)),
                    Err(e) => {
                        // couldn't get the data, keep serving cached results for now
                        error!("error retrieving master commit list: {}", e)
                    }
                }
            });
        }

        commits
    }
}

/// Parses an artifact tag like `1.63.0` or `beta-2022-08-19` from a line taken from
/// `https://static.rust-lang.org/manifests.txt`.
fn parse_published_artifact_tag(line: &str) -> Option<String> {
    lazy_static! {
        static ref VERSION_REGEX: Regex = Regex::new(r"(\d+\.\d+.\d+)").unwrap();
    }

    let mut parts = line.rsplit('/');
    let name = parts.next();
    let date = parts.next();

    if let Some(date) = date {
        if let Some(name) = name {
            // Create beta artifact in the form of beta-YYYY-MM-DD
            if name == "channel-rust-beta.toml" {
                return Some(format!("beta-{date}"));
            } else if name.contains("beta") {
                // No other beta releases are recognized as toolchains.
                //
                // We also have names like this:
                //
                // * channel-rust-1.75-beta.toml
                // * channel-rust-1.75.0-beta.toml
                // * channel-rust-1.75.0-beta.1.toml
                //
                // Which should get ignored for now, they're not consumable via rustup yet.
                return None;
            } else if let Some(capture) = VERSION_REGEX.captures(name) {
                if let Some(version) = capture.get(1).map(|c| c.as_str()) {
                    return Some(version.to_string());
                }
            }
        }
    }
    None
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
    let mut queue = master_commits
        .into_iter()
        .filter(|c| time.signed_duration_since(c.time) < Duration::days(29))
        .map(|c| {
            (
                Commit {
                    sha: c.sha,
                    date: Date(c.time),
                    r#type: CommitType::Master,
                },
                // All recent master commits should have an associated PR
                MissingReason::Master {
                    pr: c.pr.unwrap_or(0),
                    parent_sha: c.parent_sha,
                    is_try_parent: false,
                },
            )
        })
        .collect::<Vec<_>>();
    let master_commits = queue
        .iter()
        .map(|(mc, _)| mc.sha.clone())
        .collect::<HashSet<_>>();
    for database::QueuedCommit {
        sha,
        parent_sha,
        pr,
        include,
        exclude,
        runs,
        commit_date,
    } in queued_pr_commits
        .into_iter()
        // filter out any queued PR master commits (leaving only try commits)
        .filter(|c| !master_commits.contains(&c.sha))
    {
        // Mark the parent commit as a try_parent.
        if let Some((_, metadata)) = queue.iter_mut().find(|(m, _)| m.sha == parent_sha.as_str()) {
            if let MissingReason::Master { is_try_parent, .. } = metadata {
                *is_try_parent = true;
            } else {
                unreachable!("try commit has non-master parent {:?}", metadata);
            };
        }
        queue.push((
            Commit {
                sha: sha.to_string(),
                date: commit_date.unwrap_or(Date::empty()),
                r#type: CommitType::Try,
            },
            MissingReason::Try {
                pr,
                parent_sha,
                include,
                exclude,
                runs,
            },
        ));
    }
    for aid in in_progress_artifacts {
        match aid {
            ArtifactId::Commit(c) => {
                let previous = queue
                    .iter()
                    .find(|(i, _)| i.sha == c.sha)
                    .map(|v| Box::new(v.1.clone()));
                all_commits.remove(&c.sha);
                queue.insert(0, (c, MissingReason::InProgress(previous)));
            }
            ArtifactId::Tag(_) => {
                // do nothing, for now, though eventually we'll want an artifact queue
            }
        }
    }
    let mut already_tested = all_commits.clone();
    let mut i = 0;
    while i != queue.len() {
        if !already_tested.insert(queue[i].0.sha.clone()) {
            queue.remove(i);
        } else {
            i += 1;
        }
    }
    sort_queue(all_commits.clone(), queue)
}

fn sort_queue(
    mut done: HashSet<String>,
    mut unordered_queue: Vec<(Commit, MissingReason)>,
) -> Vec<(Commit, MissingReason)> {
    // A topological sort, where each "level" is additionally altered such that
    // try commits come first, and then sorted by PR # (as a rough heuristic for
    // earlier requests).

    let mut finished = 0;
    while finished < unordered_queue.len() {
        // The next level is those elements in the unordered queue which
        // are ready to be benchmarked (i.e., those with parent in done or no
        // parent).
        let level_len = partition_in_place(unordered_queue[finished..].iter_mut(), |(_, mr)| {
            mr.parent_sha().map_or(true, |parent| done.contains(parent))
        });

        // No commit is ready for benchmarking. This can happen e.g. when a try parent commit
        // was forcefully removed from the master branch of rust-lang/rust. In this case, just
        // let the commits be benchmarked in the current order that we have, these benchmark runs
        // just won't have a parent result available.
        if level_len == 0 {
            return unordered_queue;
        }

        let level = &mut unordered_queue[finished..][..level_len];
        level.sort_unstable_by_key(|(c, mr)| {
            (
                // InProgress MR go first (false < true)
                mr.parent_sha().is_some(),
                mr.pr().unwrap_or(0),
                c.sha.clone(),
            )
        });
        for (c, _) in level {
            done.insert(c.sha.clone());
        }
        finished += level_len;
    }
    unordered_queue
}

// Copy of Iterator::partition_in_place, which is currently unstable.
fn partition_in_place<'a, I, T: 'a, P>(mut iter: I, mut predicate: P) -> usize
where
    I: Sized + DoubleEndedIterator<Item = &'a mut T>,
    P: FnMut(&T) -> bool,
{
    // FIXME: should we worry about the count overflowing? The only way to have more than
    // `usize::MAX` mutable references is with ZSTs, which aren't useful to partition...

    // These closure "factory" functions exist to avoid genericity in `Self`.

    #[inline]
    fn is_false<'a, T>(
        predicate: &'a mut impl FnMut(&T) -> bool,
        true_count: &'a mut usize,
    ) -> impl FnMut(&&mut T) -> bool + 'a {
        move |x| {
            let p = predicate(&**x);
            *true_count += p as usize;
            !p
        }
    }

    #[inline]
    fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
        move |x| predicate(&**x)
    }

    // Repeatedly find the first `false` and swap it with the last `true`.
    let mut true_count = 0;
    while let Some(head) = iter.find(is_false(&mut predicate, &mut true_count)) {
        if let Some(tail) = iter.rfind(is_true(&mut predicate)) {
            std::mem::swap(head, tail);
            true_count += 1;
        } else {
            break;
        }
    }
    true_count
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

    // Checks that when we have a setup like the following, where a -> b means b
    // is the parent of a (i.e., must be tested before we can report comparison
    // results for a):
    //
    // a -> b
    //   -> try-on-a
    //
    // the resulting ordering is:
    //
    // b
    // a
    // try-on-a
    //
    // which ensures that as each commit finishes, we have the results for it.
    //
    // Note that try-on-a does *not* have a direct dependency on b's results
    // being available; we could order b after ([a, try-on-a, b]) but this means
    // that we have to be more careful about posting comparison results, and to
    // most observers they expect those posted as soon as the PR's build in the
    // queue finishes: not doing so will look odd to onlookers.
    #[test]
    fn try_commit_ancestors() {
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![
            MasterCommit {
                sha: "a".into(),
                parent_sha: "b".into(),
                pr: Some(2),
                time,
            },
            MasterCommit {
                sha: "b".into(),
                parent_sha: "c".into(),
                pr: Some(1),
                time,
            },
        ];
        let queued_pr_commits = vec![
            QueuedCommit {
                sha: "try-on-a".into(),
                parent_sha: "a".into(),
                pr: 3,
                include: None,
                exclude: None,
                runs: None,
                commit_date: None,
            },
            QueuedCommit {
                sha: "b".into(),
                parent_sha: "c".into(),
                pr: 1,
                include: None,
                exclude: None,
                runs: None,
                commit_date: None,
            },
            QueuedCommit {
                sha: "a".into(),
                parent_sha: "b".into(),
                pr: 2,
                include: None,
                exclude: None,
                runs: None,
                commit_date: None,
            },
        ];
        let in_progress_artifacts = vec![];
        let mut all_commits = HashSet::new();
        all_commits.insert("c".into());

        let expected = vec![
            (
                Commit {
                    sha: "b".into(),
                    date: database::Date(time),
                    r#type: CommitType::Master,
                },
                MissingReason::Master {
                    pr: 1,
                    parent_sha: "c".into(),
                    is_try_parent: false,
                },
            ),
            (
                Commit {
                    sha: "a".into(),
                    date: database::Date(time),
                    r#type: CommitType::Master,
                },
                MissingReason::Master {
                    pr: 2,
                    parent_sha: "b".into(),
                    is_try_parent: true,
                },
            ),
            (
                Commit {
                    sha: "try-on-a".into(),
                    date: database::Date(time),
                    r#type: CommitType::Try,
                },
                MissingReason::Try {
                    pr: 3,
                    parent_sha: "a".into(),
                    include: None,
                    exclude: None,
                    runs: None,
                },
            ),
        ];
        let found = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );
        assert_eq!(expected, found, "{:#?} != {:#?}", expected, found);
    }

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
                commit_date: None,
            },
            // A try run
            QueuedCommit {
                sha: "baz".into(),
                parent_sha: "foo".into(),
                pr: 101,
                include: None,
                exclude: None,
                runs: None,
                commit_date: None,
            },
        ];
        let in_progress_artifacts = vec![];
        let mut all_commits = HashSet::new();
        all_commits.insert(master_commits[1].sha.clone());
        // Parent trailers
        all_commits.insert(master_commits[0].parent_sha.clone());
        all_commits.insert(master_commits[1].parent_sha.clone());
        all_commits.insert(master_commits[2].parent_sha.clone());

        let expected = vec![
            (
                Commit {
                    sha: "123".into(),
                    date: database::Date(time),
                    r#type: CommitType::Master,
                },
                MissingReason::Master {
                    pr: 11,
                    parent_sha: "345".into(),
                    is_try_parent: false,
                },
            ),
            (
                Commit {
                    sha: "foo".into(),
                    date: database::Date(time),
                    r#type: CommitType::Master,
                },
                MissingReason::Master {
                    pr: 77,
                    parent_sha: "bar".into(),
                    is_try_parent: true,
                },
            ),
            (
                Commit {
                    sha: "baz".into(),
                    date: database::Date(time),
                    r#type: CommitType::Try,
                },
                MissingReason::Try {
                    pr: 101,
                    parent_sha: "foo".into(),
                    include: None,
                    exclude: None,
                    runs: None,
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

    #[test]
    fn parse_published_beta_artifact() {
        assert_eq!(
            parse_published_artifact_tag(
                "static.rust-lang.org/dist/2022-08-15/channel-rust-beta.toml"
            ),
            Some("beta-2022-08-15".to_string())
        );
    }

    #[test]
    fn parse_published_stable_artifact() {
        assert_eq!(
            parse_published_artifact_tag(
                "static.rust-lang.org/dist/2022-08-15/channel-rust-1.63.0.toml"
            ),
            Some("1.63.0".to_string())
        );
    }

    #[test]
    fn parse_published_beta_non_rustup_1() {
        assert_eq!(
            parse_published_artifact_tag(
                "static.rust-lang.org/dist/2023-11-13/channel-rust-1.75-beta.toml"
            ),
            None
        );
    }

    #[test]
    fn parse_published_beta_non_rustup_2() {
        assert_eq!(
            parse_published_artifact_tag(
                "static.rust-lang.org/dist/2023-11-13/channel-rust-1.75.0-beta.toml"
            ),
            None
        );
    }

    #[test]
    fn parse_published_beta_non_rustup_3() {
        assert_eq!(
            parse_published_artifact_tag(
                "static.rust-lang.org/dist/2023-11-13/channel-rust-1.75.0-beta.1.toml"
            ),
            None
        );
    }
}
