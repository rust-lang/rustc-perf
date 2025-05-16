use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;
use std::sync::{Arc, LazyLock};
use std::time::Instant;

use arc_swap::{ArcSwap, Guard};
use chrono::{Duration, Utc};
use log::error;
use parking_lot::Mutex;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::self_profile::SelfProfileCache;
use collector::compile::benchmark::category::Category;
use collector::{Bound, MasterCommit};
pub use database::{ArtifactId, Benchmark, Commit};
use database::{CommitJob, CommitJobStatus, Pool, Target};
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
        backends: Option<String>,
    },
    InProgress(Option<Box<MissingReason>>),
}

impl MissingReason {
    pub fn pr(&self) -> Option<u32> {
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
    pub fn parent_sha(&self) -> Option<&str> {
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
    pub fn include(&self) -> Option<&str> {
        let mut this = self;
        loop {
            match this {
                // For Master variant there is no include field.
                MissingReason::Master { .. } => return None,
                MissingReason::Try { include, .. } => return include.as_deref(),
                MissingReason::InProgress(Some(s)) => this = s,
                MissingReason::InProgress(None) => return None,
            }
        }
    }
    pub fn exclude(&self) -> Option<&str> {
        let mut this = self;
        loop {
            match this {
                // For Master variant there is no exclude field.
                MissingReason::Master { .. } => return None,
                MissingReason::Try { exclude, .. } => return exclude.as_deref(),
                MissingReason::InProgress(Some(s)) => this = s,
                MissingReason::InProgress(None) => return None,
            }
        }
    }
    pub fn runs(&self) -> Option<i32> {
        let mut this = self;
        loop {
            match this {
                // For Master variant there is no runs field.
                MissingReason::Master { .. } => return None,
                MissingReason::Try { runs, .. } => return *runs,
                MissingReason::InProgress(Some(s)) => this = s,
                MissingReason::InProgress(None) => return None,
            }
        }
    }
    pub fn backends(&self) -> Option<&str> {
        let mut this = self;
        loop {
            match this {
                // For Master variant there is no backends field.
                MissingReason::Master { .. } => return None,
                MissingReason::Try { backends, .. } => return backends.as_deref(),
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

    /// Will enqueue jobs to the database ready for a collector to take
    pub async fn enqueue_commit_jobs(&self) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
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

            let jobs = enqueue_new_jobs(
                master_commits.clone(),
                queued_pr_commits,
                in_progress_artifacts,
                all_commits,
                Utc::now(),
            );

            conn.enqueue_commit_job(Target::X86_64UnknownLinuxGnu, &jobs)
                .await;
            interval.tick().await;

            println!("Cron job executed at: {:?}", std::time::SystemTime::now());
        }
    }

    /// Returns the not yet tested published artifacts, sorted from newest to oldest.
    pub async fn missing_published_artifacts(&self) -> anyhow::Result<Vec<String>> {
        let artifact_list: String = reqwest::get("https://static.rust-lang.org/manifests.txt")
            .await?
            .text()
            .await?;

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
    static VERSION_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(\d+\.\d+.\d+)").unwrap());

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
        .map(|(commit, _)| commit.sha.clone())
        .collect::<HashSet<_>>();
    for database::QueuedCommit {
        sha,
        parent_sha,
        pr,
        include,
        exclude,
        runs,
        commit_date,
        backends,
    } in queued_pr_commits
        .into_iter()
        // filter out any queued PR master commits (leaving only try commits)
        .filter(|queued_commit| !master_commits.contains(&queued_commit.sha))
    {
        // Mark the parent commit as a try_parent.
        if let Some((_, missing_reason)) = queue
            .iter_mut()
            .find(|(commit, _)| commit.sha == parent_sha.as_str())
        {
            /* Mutates the parent by scanning the list again... bad. */
            if let MissingReason::Master { is_try_parent, .. } = missing_reason {
                *is_try_parent = true;
            } else {
                unreachable!("try commit has non-master parent {:?}", missing_reason);
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
                backends,
            },
        ));
    }
    for aid in in_progress_artifacts {
        match aid {
            ArtifactId::Commit(aid_commit) => {
                let previous = queue
                    .iter()
                    .find(|(commit, _)| commit.sha == aid_commit.sha)
                    .map(|pair| Box::new(pair.1.clone()));
                all_commits.remove(&aid_commit.sha);
                queue.insert(0, (aid_commit, MissingReason::InProgress(previous)));
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

/// Conversion between our application layer and database object, also
/// simplifies the `(Commit, MissingReason)` tuple by essentially flatteing it
fn commit_job_from_queue_item(item: &(Commit, MissingReason)) -> CommitJob {
    CommitJob {
        sha: item.0.sha.clone(),
        parent_sha: item.1.parent_sha().map(|it| it.to_string()),
        commit_type: item.0.r#type.clone(),
        pr: item.1.pr().unwrap_or(0),
        commit_time: item.0.date,
        target: Target::X86_64UnknownLinuxGnu,
        machine_id: None,
        started_at: None,
        finished_at: None,
        status: CommitJobStatus::Queued,
        include: item.1.include().map(|it| it.to_string()),
        exclude: item.1.exclude().map(|it| it.to_string()),
        runs: item.1.runs(),
        backends: item.1.backends().map(|it| it.to_string()),
    }
}

fn enqueue_new_jobs(
    master_commits: Vec<collector::MasterCommit>,
    queued_pr_commits: Vec<database::QueuedCommit>,
    in_progress_artifacts: Vec<ArtifactId>,
    mut all_commits: HashSet<String>,
    time: chrono::DateTime<chrono::Utc>,
) -> Vec<CommitJob> {
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
        .map(|(commit, _)| commit.sha.clone())
        .collect::<HashSet<_>>();
    for database::QueuedCommit {
        sha,
        parent_sha,
        pr,
        include,
        exclude,
        runs,
        commit_date,
        backends,
    } in queued_pr_commits
        .into_iter()
        // filter out any queued PR master commits (leaving only try commits)
        .filter(|queued_commit| !master_commits.contains(&queued_commit.sha))
    {
        // Mark the parent commit as a try_parent.
        if let Some((_, missing_reason)) = queue
            .iter_mut()
            .find(|(commit, _)| commit.sha == parent_sha.as_str())
        {
            /* Mutates the parent by scanning the list again... bad. */
            if let MissingReason::Master { is_try_parent, .. } = missing_reason {
                *is_try_parent = true;
            } else {
                unreachable!("try commit has non-master parent {:?}", missing_reason);
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
                backends,
            },
        ));
    }
    for aid in in_progress_artifacts {
        match aid {
            ArtifactId::Commit(aid_commit) => {
                let previous = queue
                    .iter()
                    .find(|(commit, _)| commit.sha == aid_commit.sha)
                    .map(|pair| Box::new(pair.1.clone()));
                all_commits.remove(&aid_commit.sha);
                queue.insert(0, (aid_commit, MissingReason::InProgress(previous)));
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

    let (jobs, job_sha_set) = queue.iter().fold(
        (vec![], HashSet::<String>::new()),
        |(mut vec, mut set), item| {
            let job = commit_job_from_queue_item(item);
            set.insert(job.sha.clone());
            vec.push(job);
            (vec, set)
        },
    );

    /* Jobs that are viable to be inserted into the database, these are jobs
     * which either have a parent that is in the set or do not have a parent.
     * Some commits could reference a parent that is not in the set which means
     * it is not ready to be queued */
    jobs.into_iter()
        .filter(|job| {
            if let Some(parent_sha) = &job.parent_sha {
                job_sha_set.contains(parent_sha)
            } else {
                false
            }
        })
        .collect()
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

    /// Create a master missing reason
    fn create_missing_reason_master(
        parent_sha: &str,
        pr: u32,
        is_try_parent: bool,
    ) -> MissingReason {
        MissingReason::Master {
            pr,
            parent_sha: parent_sha.into(),
            is_try_parent,
        }
    }

    /// Create a try misssing reason
    fn create_missing_reason_try(parent_sha: &str, pr: u32) -> MissingReason {
        MissingReason::Try {
            pr,
            parent_sha: parent_sha.into(),
            include: None,
            exclude: None,
            runs: None,
            backends: None,
        }
    }

    /// Create a Commit
    fn create_commit(commit_sha: &str, time: chrono::DateTime<Utc>, r#type: CommitType) -> Commit {
        Commit {
            sha: commit_sha.into(),
            date: database::Date(time),
            r#type,
        }
    }

    /// Create a (Commit, MissingReason::Master) pair
    fn create_commit_master_pair(
        commit_sha: &str,
        parent_sha: &str,
        pr: u32,
        time: chrono::DateTime<Utc>,
        is_try_parent: bool,
    ) -> (Commit, MissingReason) {
        (
            create_commit(commit_sha, time, CommitType::Master),
            create_missing_reason_master(parent_sha, pr, is_try_parent),
        )
    }

    /// Create a (Commit, MissingReason::Try) pair
    fn create_commit_try_pair(
        commit_sha: &str,
        parent_sha: &str,
        pr: u32,
        time: chrono::DateTime<Utc>,
    ) -> (Commit, MissingReason) {
        (
            create_commit(commit_sha, time, CommitType::Try),
            create_missing_reason_try(parent_sha, pr),
        )
    }

    /// Create a master commit
    fn create_master_commit(
        sha: &str,
        parent_sha: &str,
        pr: Option<u32>,
        time: chrono::DateTime<Utc>,
    ) -> MasterCommit {
        MasterCommit {
            sha: sha.into(),
            parent_sha: parent_sha.into(),
            pr,
            time,
        }
    }

    /// Create a queued commit
    fn create_queued_commit(sha: &str, parent_sha: &str, pr: u32) -> QueuedCommit {
        QueuedCommit {
            sha: sha.into(),
            parent_sha: parent_sha.into(),
            pr,
            include: None,
            exclude: None,
            runs: None,
            commit_date: None,
            backends: None,
        }
    }

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
                backends: None,
            },
            QueuedCommit {
                sha: "b".into(),
                parent_sha: "c".into(),
                pr: 1,
                include: None,
                exclude: None,
                runs: None,
                commit_date: None,
                backends: None,
            },
            QueuedCommit {
                sha: "a".into(),
                parent_sha: "b".into(),
                pr: 2,
                include: None,
                exclude: None,
                runs: None,
                commit_date: None,
                backends: None,
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
                    backends: None,
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
            // An already tested commit
            create_master_commit("abc", "def", Some(90), time),
            // A queued PR commit
            create_master_commit("foo", "bar", Some(77), time),
            // A not yet tested commit
            create_master_commit("123", "345", Some(11), time),
        ];
        let queued_pr_commits = vec![
            // A master commit
            create_queued_commit("foo", "bar", 77),
            // A try run
            create_queued_commit("baz", "foo", 1),
            // Another try run that will become the head of the queue
            create_queued_commit("yee", "rrr", 4),
        ];
        let in_progress_artifacts = vec![];
        let mut all_commits = HashSet::new();
        all_commits.insert("abc".to_string());
        // Parent trailers
        all_commits.insert(master_commits[0].parent_sha.clone());
        all_commits.insert(master_commits[1].parent_sha.clone());
        all_commits.insert(master_commits[2].parent_sha.clone());
        all_commits.insert(queued_pr_commits[2].parent_sha.clone());

        let expected = vec![
            create_commit_try_pair("yee", "rrr", 4, time),
            create_commit_master_pair("123", "345", 11, time, false),
            create_commit_master_pair("foo", "bar", 77, time, true),
            create_commit_try_pair("baz", "foo", 1, time),
        ];

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn queue_empty() {
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        // Pathalogical case of an empty queue
        let master_commits = vec![];
        let queued_pr_commits = vec![];
        let in_progress_artifacts = vec![];
        let all_commits = HashSet::new();

        let expected: Vec<(Commit, MissingReason)> = vec![];

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn only_master_commits_already_tested() {
        // If all master commits are in the set then the queue is in effect empty
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![
            create_master_commit("m1", "p1", Some(10), time),
            create_master_commit("m2", "m1", Some(11), time),
            create_master_commit("m3", "m2", Some(12), time),
        ];
        let queued_pr_commits = vec![];
        let in_progress_artifacts = vec![];
        let mut all_commits = HashSet::new();

        all_commits.insert("m1".to_string());
        all_commits.insert("m2".to_string());
        all_commits.insert("m3".to_string());
        all_commits.insert("p1".to_string());

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );

        let expected: Vec<(Commit, MissingReason)> = vec![];
        assert_eq!(expected, result);
    }

    #[test]
    fn multiple_try_with_same_parent() {
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![create_master_commit("abc", "def", Some(1), time)];
        // scramble the order
        let queued_pr_commits = vec![
            create_queued_commit("try3", "abc", 203),
            create_queued_commit("try1", "abc", 201),
            create_queued_commit("try2", "abc", 202),
        ];
        let in_progress_artifacts = vec![];
        let mut all_commits = HashSet::new();
        all_commits.insert("abc".to_string());
        all_commits.insert("def".to_string());

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );

        let expected = vec![
            create_commit_try_pair("try1", "abc", 201, time),
            create_commit_try_pair("try2", "abc", 202, time),
            create_commit_try_pair("try3", "abc", 203, time),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn in_progress_commit() {
        // InProgress commits should get inserted into the queue
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![create_master_commit("abc", "def", Some(50), time)];
        let queued_pr_commits = vec![];
        let in_progress_artifacts = vec![ArtifactId::from(create_commit(
            "abc",
            time,
            CommitType::Master,
        ))];
        let mut all_commits = HashSet::new();
        all_commits.insert("def".to_string());

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );

        let expected = vec![(
            create_commit("abc", time, CommitType::Master),
            MissingReason::InProgress(Some(Box::new(create_missing_reason_master(
                "def", 50, false,
            )))),
        )];
        assert_eq!(expected, result);
    }

    #[test]
    fn orphan_commit() {
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![create_master_commit("x123", "ORPHAN", Some(69), time)];
        let queued_pr_commits = vec![];
        let in_progress_artifacts = vec![];
        let all_commits = HashSet::new();

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );

        let expected = vec![create_commit_master_pair("x123", "ORPHAN", 69, time, false)];
        assert_eq!(expected, result);
    }

    #[test]
    fn artifact_tags() {
        // ArtifactId with a type of `Tag` should be ignored
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![create_master_commit("abc", "def", Some(50), time)];
        let queued_pr_commits = vec![];
        let in_progress_artifacts = vec![ArtifactId::Tag("nightly-2025-04-10".to_string())];
        let mut all_commits = HashSet::new();
        all_commits.insert("def".to_string());

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );

        let expected = vec![create_commit_master_pair("abc", "def", 50, time, false)];
        assert_eq!(expected, result);
    }

    #[test]
    fn duplicate_commits() {
        let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
        let master_commits = vec![
            create_master_commit("dup1", "m1", Some(69), time),
            create_master_commit("dup1", "m1", Some(69), time),
        ];
        let queued_pr_commits = vec![
            create_queued_commit("trydup", "dup1", 100),
            create_queued_commit("trydup", "dup1", 100),
        ];
        let in_progress_artifacts = vec![];
        let all_commits = HashSet::new();

        let result = calculate_missing_from(
            master_commits,
            queued_pr_commits,
            in_progress_artifacts,
            all_commits,
            time,
        );

        let expected = vec![
            create_commit_master_pair("dup1", "m1", 69, time, true),
            create_commit_try_pair("trydup", "dup1", 100, time),
        ];
        assert_eq!(expected, result);
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
