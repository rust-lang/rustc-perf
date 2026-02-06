use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::Instant;

use arc_swap::{ArcSwap, Guard};
use log::error;
use parking_lot::Mutex;
use serde::Deserialize;

use crate::self_profile::SelfProfileCache;
use collector::compile::benchmark::category::Category;
use collector::{Bound, MasterCommit, SelfProfileStorage};
use database::Pool;
pub use database::{ArtifactId, Benchmark, Commit};

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
    /// Summary of some basic data present in the database.
    pub data_summary: BenchmarkDataSummary,
    /// Cached master-branch Rust commits
    pub master_commits: Arc<ArcSwap<MasterCommitCache>>, // outer Arc enables mutation in background task
    /// Cache for self profile data
    pub self_profile_cache: Mutex<SelfProfileCache>,
    /// Resolver for fetching self-profile data
    pub self_profile_storage: Box<dyn SelfProfileStorage + Send + Sync>,
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
    pub async fn from_db_url(
        db_url: &str,
        self_profile_storage: Box<dyn SelfProfileStorage + Send + Sync>,
    ) -> anyhow::Result<Self> {
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

        // We load this data only at website start, as they change very infrequently
        let compile_metrics = {
            let mut metrics = index.compile_metrics();
            metrics.sort();
            metrics
        };
        let runtime_metrics = {
            let mut metrics = index.runtime_metrics();
            metrics.sort();
            metrics
        };

        Ok(Self {
            config,
            index: ArcSwap::new(Arc::new(index)),
            data_summary: BenchmarkDataSummary {
                compile_metrics,
                runtime_metrics,
            },
            master_commits: Arc::new(ArcSwap::new(Arc::new(master_commits))),
            pool,
            landing_page: ArcSwap::new(Arc::new(None)),
            self_profile_cache: Mutex::new(SelfProfileCache::new(CACHED_SELF_PROFILE_COUNT)),
            self_profile_storage,
        })
    }

    pub async fn conn(&self) -> Box<dyn database::pool::Connection> {
        self.pool.connection().await
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

/// Summary of data contained in the database which changes *very* infrequently, so it can be
/// aggressively cached - we only load the summary when starting the website.
/// Currently, it contains compile and runtime metrics.
pub struct BenchmarkDataSummary {
    /// All known compile benchmark metrics (e.g. instruction count, cycles, etc.) contained in
    /// the DB.
    compile_metrics: Vec<String>,
    /// All known runtime benchmark metrics (e.g. instruction count, cycles, etc.) contained in
    /// the DB.
    runtime_metrics: Vec<String>,
}

impl BenchmarkDataSummary {
    pub fn compile_metrics(&self) -> &[String] {
        &self.compile_metrics
    }

    pub fn runtime_metrics(&self) -> &[String] {
        &self.runtime_metrics
    }
}
