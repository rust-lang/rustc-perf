use crate::selector::CompileTestCase;
use crate::{
    ArtifactCollection, ArtifactId, ArtifactIdNumber, BenchmarkJob, BenchmarkJobConclusion,
    BenchmarkJobKind, BenchmarkRequest, BenchmarkRequestIndex, BenchmarkRequestStatus,
    BenchmarkRequestWithErrors, BenchmarkSet, CodegenBackend, CollectorConfig, CompileBenchmark,
    PendingBenchmarkRequests, Target,
};
use crate::{CollectionId, Index, Profile, QueuedCommit, Scenario, Step};
use chrono::{DateTime, Utc};
use hashbrown::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub mod postgres;
pub mod sqlite;

#[async_trait::async_trait]
pub trait Connection: Send + Sync {
    async fn maybe_create_indices(&mut self);
    async fn transaction(&mut self) -> Box<dyn Transaction + '_>;

    async fn load_index(&mut self) -> Index;

    /// Returns true if the given database backend supports the job queue system.
    fn supports_job_queue(&self) -> bool;

    /// None means that the caller doesn't know; it should be left alone if
    /// known or set to false if unknown.
    async fn record_compile_benchmark(
        &self,
        krate: &str,
        supports_stable: Option<bool>,
        category: String,
    );
    async fn get_compile_benchmarks(&self) -> Vec<CompileBenchmark>;

    async fn artifact_by_name(&self, artifact: &str) -> Option<ArtifactId>;

    /// This records the duration of a collection run, i.e., collecting all of
    /// the statistics for a particular artifact.
    async fn record_duration(&self, artifact: ArtifactIdNumber, duration: Duration);

    /// One collection corresponds to all gathered metrics for a single iteration of a test case.
    async fn collection_id(&self, version: &str) -> CollectionId;
    async fn artifact_id(&self, artifact: &ArtifactId) -> ArtifactIdNumber;

    #[allow(clippy::too_many_arguments)]
    async fn record_statistic(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        benchmark: &str,
        profile: Profile,
        scenario: Scenario,
        backend: CodegenBackend,
        target: Target,
        metric: &str,
        value: f64,
    );
    async fn record_runtime_statistic(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        benchmark: &str,
        metric: &str,
        value: f64,
    );
    /// Records a self-profile artifact in S3.
    ///
    /// The upload is a separate step (which may fail or be canceled, but that's
    /// fine, the database just stores that the files *may* be there).
    async fn record_raw_self_profile(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        benchmark: &str,
        profile: Profile,
        scenario: Scenario,
    );
    async fn record_error(
        &self,
        artifact: ArtifactIdNumber,
        context: &str,
        message: &str,
        job_id: Option<u32>,
    );
    async fn record_rustc_crate(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        value: Duration,
    );

    /// Records the size of an artifact component (like `librustc_driver.so` or `libLLVM.so`) in
    /// bytes.
    async fn record_artifact_size(&self, artifact: ArtifactIdNumber, component: &str, size: u64);

    /// Returns the sizes of individual components of a single artifact.
    async fn get_artifact_size(&self, aid: ArtifactIdNumber) -> HashMap<String, u64>;

    /// Returns vector of bootstrap build times for the given artifacts. The kth
    /// element is the minimum build time for the kth artifact in `aids`, across
    /// all collections for the artifact, or none if there is no bootstrap data
    /// for that artifact (for example, because the rustc benchmark wasn't
    /// executed for that artifact).
    async fn get_bootstrap(&self, aids: &[ArtifactIdNumber]) -> Vec<Option<Duration>>;
    /// Returns map from rustc crate name to vector of build times for that crate
    /// for the given artifacts. Within a crate's corresponding vector, the kth
    /// element is the minimum build time for the kth artifact in `aids`, across
    /// all collections for the artifact, or none if there is no data for that
    /// artifact / crate combination (for example, because that rustc crate
    /// wasn't present when building rustc with that artifact, or because the
    /// rustc benchmark wasn't executed for that artifact). A crate will not be
    /// included as a key in the map unless at least one artifact in `aids` has a
    /// build time for it.
    async fn get_bootstrap_by_crate(
        &self,
        aids: &[ArtifactIdNumber],
    ) -> HashMap<String, Vec<Option<Duration>>>;
    async fn get_pstats(
        &self,
        pstat_series_row_ids: &[u32],
        artifact_row_id: &[Option<ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>>;
    async fn get_runtime_pstats(
        &self,
        runtime_pstat_series_row_ids: &[u32],
        artifact_row_id: &[Option<ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>>;
    async fn get_error(&self, artifact_row_id: ArtifactIdNumber) -> HashMap<String, String>;

    async fn queue_pr(
        &self,
        pr: u32,
        include: Option<&str>,
        exclude: Option<&str>,
        runs: Option<i32>,
        backends: Option<&str>,
    );
    /// Returns true if this PR was queued waiting for a commit
    async fn pr_attach_commit(
        &self,
        pr: u32,
        sha: &str,
        parent_sha: &str,
        commit_date: Option<DateTime<Utc>>,
    ) -> bool;
    async fn queued_commits(&self) -> Vec<QueuedCommit>;
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit>;

    // Collector status API

    async fn collector_start(&self, aid: ArtifactIdNumber, steps: &[String]);

    // Returns `true` if the step was started, i.e., it did not previously have
    // an end. Otherwise returns false, indicating that we can skip it.
    async fn collector_start_step(&self, aid: ArtifactIdNumber, step: &str) -> bool;
    async fn collector_end_step(&self, aid: ArtifactIdNumber, step: &str);

    async fn collector_remove_step(&self, aid: ArtifactIdNumber, step: &str);

    async fn in_progress_artifacts(&self) -> Vec<ArtifactId>;

    async fn in_progress_steps(&self, aid: &ArtifactId) -> Vec<Step>;

    async fn last_n_artifact_collections(&self, n: u32) -> Vec<ArtifactCollection>;

    /// Returns the sha of the parent commit, if available.
    ///
    /// (Currently only works for try commits)
    async fn parent_of(&self, sha: &str) -> Option<String>;

    /// Returns the PR of the parent commit, if available.
    ///
    /// (Currently only works for try commits)
    async fn pr_of(&self, sha: &str) -> Option<u32>;

    /// Returns the collection ids corresponding to the query. Usually just one.
    ///
    /// Currently only supported by postgres (sqlite does not store self-profile
    /// results in the raw format).
    async fn list_self_profile(
        &self,
        aid: ArtifactId,
        crate_: &str,
        profile: &str,
        cache: &str,
    ) -> Vec<(ArtifactIdNumber, i32)>;

    /// Removes all data associated with the given artifact.
    async fn purge_artifact(&self, aid: &ArtifactId);

    /// Add an item to the `benchmark_requests`, if the `benchmark_request`
    /// exists an Error will be returned.
    /// We require the caller to pass an index, to ensure that it is always kept up-to-date.
    async fn insert_benchmark_request(
        &self,
        benchmark_request: &BenchmarkRequest,
    ) -> anyhow::Result<()>;

    /// Load all known benchmark request SHAs and all completed benchmark requests.
    async fn load_benchmark_request_index(&self) -> anyhow::Result<BenchmarkRequestIndex>;

    /// Load all pending benchmark requests, i.e. those that have artifacts ready, but haven't
    /// been completed yet. Pending statuses are `ArtifactsReady` and `InProgress`.
    /// Also returns their parents, so that we can quickly check which requests are ready for being
    /// enqueued.
    async fn load_pending_benchmark_requests(&self) -> anyhow::Result<PendingBenchmarkRequests>;

    /// Update the status of a `benchmark_request` with the given `tag`.
    /// If no such request exists in the DB, returns an error.
    async fn update_benchmark_request_status(
        &self,
        tag: &str,
        status: BenchmarkRequestStatus,
    ) -> anyhow::Result<()>;

    /// Update a Try commit to have a `sha` and `parent_sha`. Will update the
    /// status of the request too a ready state.
    async fn attach_shas_to_try_benchmark_request(
        &self,
        pr: u32,
        sha: &str,
        parent_sha: &str,
        commit_date: DateTime<Utc>,
    ) -> anyhow::Result<()>;

    /// Add a benchmark job to the job queue and returns its ID, if it was not
    /// already in the DB previously.
    async fn enqueue_benchmark_job(
        &self,
        request_tag: &str,
        target: Target,
        backend: CodegenBackend,
        profile: Profile,
        benchmark_set: u32,
        kind: BenchmarkJobKind,
    ) -> anyhow::Result<Option<u32>>;

    /// Add a benchmark job which is explicitly using a `parent_sha` we split
    /// this out to improve our error handling. A `parent_sha` may not have
    /// an associated request in the `benchmarek`
    async fn enqueue_parent_benchmark_job(
        &self,
        parent_sha: &str,
        target: Target,
        backend: CodegenBackend,
        profile: Profile,
        benchmark_set: u32,
        kind: BenchmarkJobKind,
    ) -> (bool, anyhow::Result<u32>);

    /// Returns a set of compile-time benchmark test cases that were already computed for the
    /// given artifact.
    /// Note that for efficiency reasons, the function only checks if we have at least a single
    /// result for a given test case. It does not check if *all* test results from all test
    /// iterations were finished.
    /// Therefore, the result is an over-approximation.
    async fn get_compile_test_cases_with_measurements(
        &self,
        artifact_row_id: &ArtifactIdNumber,
    ) -> anyhow::Result<HashSet<CompileTestCase>>;

    /// Add the confiuguration for a collector
    async fn add_collector_config(
        &self,
        collector_name: &str,
        target: Target,
        benchmark_set: u32,
        is_active: bool,
    ) -> anyhow::Result<CollectorConfig>;

    /// Call this function when a job queue collector starts.
    /// It ensures that a collector with the given name exists, updates its commit SHA and heartbeat
    /// and returns its collector config.
    async fn start_collector(
        &self,
        collector_name: &str,
        commit_sha: &str,
    ) -> anyhow::Result<Option<CollectorConfig>>;

    /// Dequeues a single job for the given collector, target and benchmark set.
    /// Also returns detailed information about the compiler artifact that should be benchmarked
    /// in the job.
    async fn dequeue_benchmark_job(
        &self,
        collector_name: &str,
        target: Target,
        benchmark_set: BenchmarkSet,
    ) -> anyhow::Result<Option<(BenchmarkJob, ArtifactId)>>;

    /// Try and mark the benchmark_request as completed. Will return `true` if
    /// it has been marked as completed else `false` meaning there was no change
    async fn maybe_mark_benchmark_request_as_completed(&self, tag: &str) -> anyhow::Result<bool>;

    /// Mark the job as completed. Sets the status to 'failed' or 'success'
    /// depending on the enum's completed state being a success
    async fn mark_benchmark_job_as_completed(
        &self,
        id: u32,
        conclusion: BenchmarkJobConclusion,
    ) -> anyhow::Result<()>;

    /// Return the last `count` completed benchmark requests, along with all errors associated with
    /// them.
    ///
    /// The requests will be ordered from most recently to least recently completed.
    async fn get_last_n_completed_benchmark_requests(
        &self,
        count: u64,
    ) -> anyhow::Result<Vec<BenchmarkRequestWithErrors>>;

    /// Return jobs of all requests that are currently in progress, and the jobs of their parents.
    /// The keys of the hashmap contain the request tags.
    async fn get_jobs_of_in_progress_benchmark_requests(
        &self,
    ) -> anyhow::Result<HashMap<String, Vec<BenchmarkJob>>>;

    /// Get all of the configuration for all of the collectors
    async fn get_collector_configs(&self) -> anyhow::Result<Vec<CollectorConfig>>;

    /// Updates the last known heartbeat of a collector to the current time.
    async fn update_collector_heartbeat(&self, collector_name: &str) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait Transaction: Send + Sync {
    fn conn(&mut self) -> &mut dyn Connection;
    fn conn_ref(&self) -> &dyn Connection;

    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error>;
    async fn finish(self: Box<Self>) -> Result<(), anyhow::Error>;
}

#[async_trait::async_trait]
pub trait ConnectionManager {
    type Connection;
    async fn open(&self) -> Self::Connection;
    async fn is_valid(&self, c: &mut Self::Connection) -> bool;
}

pub struct ConnectionPool<M: ConnectionManager> {
    connections: Arc<Mutex<Vec<M::Connection>>>,
    permits: Arc<Semaphore>,
    manager: M,
}

pub struct ManagedConnection<T> {
    conn: Option<T>,
    connections: Arc<Mutex<Vec<T>>>,
    #[allow(unused)]
    permit: OwnedSemaphorePermit,
}

impl<T> std::ops::Deref for ManagedConnection<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}
impl<T> std::ops::DerefMut for ManagedConnection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

impl<T> Drop for ManagedConnection<T> {
    fn drop(&mut self) {
        let conn = self.conn.take().unwrap();
        self.connections
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(conn);
    }
}

impl<T, M> ConnectionPool<M>
where
    T: Send,
    M: ConnectionManager<Connection = T>,
{
    fn new(manager: M) -> Self {
        ConnectionPool {
            connections: Arc::new(Mutex::new(Vec::with_capacity(16))),
            permits: Arc::new(Semaphore::new(16)),
            manager,
        }
    }

    pub fn raw(&mut self) -> &mut M {
        &mut self.manager
    }

    async fn get(&self) -> ManagedConnection<T> {
        let permit = self.permits.clone().acquire_owned().await.unwrap();
        let conn = {
            let mut slots = self.connections.lock().unwrap_or_else(|e| e.into_inner());
            slots.pop()
        };
        if let Some(mut c) = conn {
            if self.manager.is_valid(&mut c).await {
                return ManagedConnection {
                    conn: Some(c),
                    permit,
                    connections: self.connections.clone(),
                };
            }
        }

        let conn = self.manager.open().await;
        ManagedConnection {
            conn: Some(conn),
            connections: self.connections.clone(),
            permit,
        }
    }
}

pub enum Pool {
    Sqlite(ConnectionPool<sqlite::Sqlite>),
    Postgres(ConnectionPool<postgres::Postgres>),
}

impl Pool {
    pub async fn connection(&self) -> Box<dyn Connection> {
        match self {
            Pool::Sqlite(p) => Box::new(sqlite::SqliteConnection::new(p.get().await)),
            Pool::Postgres(p) => Box::new(p.get().await),
        }
    }

    pub fn open(uri: &str) -> Pool {
        if uri.starts_with("postgres") {
            Pool::Postgres(ConnectionPool::new(postgres::Postgres::new(uri.into())))
        } else {
            Pool::Sqlite(ConnectionPool::new(sqlite::Sqlite::new(uri.into())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metric::Metric;
    use crate::tests::builder::{job, CollectorBuilder, RequestBuilder};
    use crate::tests::run_postgres_test;
    use crate::{tests::run_db_test, BenchmarkRequestType, Commit, CommitType, Date};
    use chrono::Utc;
    use std::collections::BTreeSet;
    use std::str::FromStr;

    fn create_commit(commit_sha: &str, time: chrono::DateTime<Utc>, r#type: CommitType) -> Commit {
        Commit {
            sha: commit_sha.into(),
            date: Date(time),
            r#type,
        }
    }

    #[tokio::test]
    async fn pstat_returns_empty_vector_when_empty() {
        run_db_test(|ctx| async {
            // This is essentially testing the database testing framework is
            // wired up correctly. Though makes sense that there should be
            // an empty vector returned if there are no pstats.
            let result = ctx.db().get_pstats(&[], &[]).await;
            let expected: Vec<Vec<Option<f64>>> = vec![];

            assert_eq!(result, expected);
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn artifact_storage() {
        run_db_test(|ctx| async {
            let db = ctx.db();
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();

            let artifact_one = ArtifactId::from(create_commit("abc", time, CommitType::Master));
            let artifact_two = ArtifactId::Tag("nightly-2025-05-14".to_string());

            let artifact_one_id_number = db.artifact_id(&artifact_one).await;
            let artifact_two_id_number = db.artifact_id(&artifact_two).await;

            // We cannot arbitrarily add random sizes to the artifact size
            // table, as there is a constraint that the artifact must actually
            // exist before attaching something to it.

            // Artifact one inserts
            db.record_artifact_size(artifact_one_id_number, "llvm.so", 32)
                .await;
            db.record_artifact_size(artifact_one_id_number, "llvm.a", 64)
                .await;

            // Artifact two inserts
            db.record_artifact_size(artifact_two_id_number, "another-llvm.a", 128)
                .await;

            let result_one = db.get_artifact_size(artifact_one_id_number).await;
            let result_two = db.get_artifact_size(artifact_two_id_number).await;

            // artifact one
            assert_eq!(Some(32u64), result_one.get("llvm.so").copied());
            assert_eq!(Some(64u64), result_one.get("llvm.a").copied());
            assert_eq!(None, result_one.get("another-llvm.a").copied());

            // artifact two
            assert_eq!(Some(128), result_two.get("another-llvm.a").copied());
            Ok(ctx)
        })
        .await;
    }

    // Check that we can't have multiple requests with the same SHA
    #[tokio::test]
    async fn multiple_requests_same_sha() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            db.insert_benchmark_request(&BenchmarkRequest::create_master(
                "a-sha-1",
                "parent-sha-1",
                42,
                Utc::now(),
            ))
            .await
            .unwrap();

            db.insert_benchmark_request(&BenchmarkRequest::create_release("a-sha-1", Utc::now()))
                .await
                .expect_err("it was possible to insert a second commit with the same SHA");

            Ok(ctx)
        })
        .await;
    }

    // Check that we can't have multiple non-completed try requests on the same PR
    #[tokio::test]
    async fn multiple_non_completed_try_requests() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            // Insert a try build
            ctx.insert_try_request(42).await;
            db.attach_shas_to_try_benchmark_request(42, "sha-1", "sha-parent-1", Utc::now())
                .await
                .unwrap();

            // Then finish it
            ctx.complete_request("sha-1").await;

            // Insert a try build for the same PR again
            // This should be fine, because the previous request was already completed
            ctx.insert_try_request(42).await;
            // But this should fail, as we can't have two queued requests at once
            db.insert_benchmark_request(&BenchmarkRequest::create_try_without_artifacts(
                42, "", "",
            ))
            .await
            .expect_err("It was possible to record two try benchmark requests without artifacts");

            Ok(ctx)
        })
        .await;
    }

    // Check that we can't have multiple master requests on the same PR
    #[tokio::test]
    async fn multiple_master_requests_same_pr() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            db.insert_benchmark_request(&BenchmarkRequest::create_master(
                "a-sha-1",
                "parent-sha-1",
                42,
                Utc::now(),
            ))
            .await
            .unwrap();

            db.insert_benchmark_request(&BenchmarkRequest::create_master(
                "a-sha-2",
                "parent-sha-2",
                42,
                Utc::now(),
            ))
            .await
            .expect_err("it was possible to insert a second master commit on the same PR");

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn load_pending_benchmark_requests() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            // ArtifactsReady
            let req_a = ctx.insert_master_request("sha-1", "parent-sha-1", 42).await;
            // ArtifactsReady
            let req_b = ctx.insert_release_request("1.80.0").await;
            // WaitingForArtifacts
            ctx.insert_try_request(50).await;
            // InProgress
            let req_d = ctx.insert_master_request("sha-2", "parent-sha-2", 51).await;
            // Completed
            ctx.insert_release_request("1.79.0").await;

            ctx.complete_request("1.79.0").await;
            ctx.insert_master_request("parent-sha-1", "grandparent-sha-0", 100)
                .await;
            ctx.complete_request("parent-sha-1").await;
            ctx.insert_master_request("parent-sha-2", "grandparent-sha-1", 101)
                .await;
            ctx.complete_request("parent-sha-2").await;

            db.update_benchmark_request_status("sha-2", BenchmarkRequestStatus::InProgress)
                .await
                .unwrap();

            let pending = db.load_pending_benchmark_requests().await.unwrap();
            let requests = pending.requests;

            assert_eq!(requests.len(), 3);
            for req in &[req_a, req_b, req_d] {
                assert!(requests.iter().any(|r| r.tag() == req.tag()));
            }

            assert_eq!(
                pending
                    .completed_parent_tags
                    .into_iter()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>(),
                vec!["parent-sha-1".to_string(), "parent-sha-2".to_string()]
            );

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn attach_shas_to_try_benchmark_request() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let req = BenchmarkRequest::create_try_without_artifacts(42, "", "");

            db.insert_benchmark_request(&req).await.unwrap();
            db.attach_shas_to_try_benchmark_request(42, "sha1", "sha-parent-1", Utc::now())
                .await
                .unwrap();

            let req_db = db
                .load_pending_benchmark_requests()
                .await
                .unwrap()
                .requests
                .into_iter()
                .next()
                .unwrap();
            assert_eq!(req.backends, req_db.backends);
            assert_eq!(req.profiles, req_db.profiles);
            assert!(matches!(
                req_db.status,
                BenchmarkRequestStatus::ArtifactsReady
            ));
            assert!(matches!(
                req_db.commit_type,
                BenchmarkRequestType::Try { .. }
            ));

            assert_eq!(req_db.tag(), Some("sha1"));
            assert_eq!(req_db.parent_sha(), Some("sha-parent-1"));
            assert_eq!(req_db.pr(), Some(42));

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn enqueue_benchmark_job() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
            let benchmark_request =
                BenchmarkRequest::create_master("sha-1", "parent-sha-1", 42, time);

            // Insert the request so we don't violate the foreign key
            db.insert_benchmark_request(&benchmark_request)
                .await
                .unwrap();

            // Now we can insert the job
            let result = db
                .enqueue_benchmark_job(
                    benchmark_request.tag().unwrap(),
                    Target::X86_64UnknownLinuxGnu,
                    CodegenBackend::Llvm,
                    Profile::Opt,
                    0u32,
                    BenchmarkJobKind::Runtime,
                )
                .await;
            assert!(result.is_ok());

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_compile_test_cases_with_data() {
        run_db_test(|ctx| async {
            let db = ctx.db();

            let collection = db.collection_id("test").await;
            let artifact = db
                .artifact_id(&ArtifactId::Commit(create_commit(
                    "abcdef",
                    Utc::now(),
                    CommitType::Try,
                )))
                .await;
            db.record_compile_benchmark("benchmark", None, "primary".to_string())
                .await;

            db.record_statistic(
                collection,
                artifact,
                "benchmark",
                Profile::Check,
                Scenario::IncrementalFresh,
                CodegenBackend::Llvm,
                Target::X86_64UnknownLinuxGnu,
                Metric::CacheMisses.as_str(),
                1.0,
            )
            .await;

            assert_eq!(
                db.get_compile_test_cases_with_measurements(&artifact)
                    .await
                    .unwrap(),
                HashSet::from([CompileTestCase {
                    benchmark: "benchmark".into(),
                    profile: Profile::Check,
                    scenario: Scenario::IncrementalFresh,
                    backend: CodegenBackend::Llvm,
                    target: Target::X86_64UnknownLinuxGnu,
                }])
            );

            let artifact2 = db
                .artifact_id(&ArtifactId::Commit(create_commit(
                    "abcdef2",
                    Utc::now(),
                    CommitType::Try,
                )))
                .await;
            assert!(db
                .get_compile_test_cases_with_measurements(&artifact2)
                .await
                .unwrap()
                .is_empty());
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_collector_config_error_if_not_exist() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let collector_config_result = db.start_collector("collector-1", "foo").await.unwrap();

            assert!(collector_config_result.is_none());

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn add_collector_config() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let mut inserted_config = db
                .add_collector_config("collector-1", Target::X86_64UnknownLinuxGnu, 1, true)
                .await
                .unwrap();

            let config = db
                .start_collector("collector-1", "foo")
                .await
                .unwrap()
                .expect("collector config not found");

            inserted_config.commit_sha = Some("foo".to_string());
            inserted_config.last_heartbeat_at = config.last_heartbeat_at;

            assert_eq!(inserted_config, config);
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn dequeue_benchmark_job_empty_queue() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let benchmark_job_result = db
                .dequeue_benchmark_job(
                    "collector-1",
                    Target::X86_64UnknownLinuxGnu,
                    BenchmarkSet(420),
                )
                .await;

            assert!(benchmark_job_result.is_ok());
            assert!(benchmark_job_result.unwrap().is_none());

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn dequeue_benchmark_job() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();

            let collector_config = db
                .add_collector_config("collector-1", Target::X86_64UnknownLinuxGnu, 1, true)
                .await
                .unwrap();

            let benchmark_request =
                BenchmarkRequest::create_master("sha-1", "parent-sha-1", 42, time);

            // Insert the request so we don't violate the foreign key
            db.insert_benchmark_request(&benchmark_request)
                .await
                .unwrap();

            // Now we can insert the job
            db.enqueue_benchmark_job(
                benchmark_request.tag().unwrap(),
                Target::X86_64UnknownLinuxGnu,
                CodegenBackend::Llvm,
                Profile::Opt,
                1u32,
                BenchmarkJobKind::Runtime,
            )
            .await
            .unwrap();

            let (benchmark_job, artifact_id) = db
                .dequeue_benchmark_job(
                    collector_config.name(),
                    collector_config.target(),
                    collector_config.benchmark_set(),
                )
                .await
                .unwrap()
                .unwrap();

            // Ensure the properties of the job match both the request and the
            // collector configuration
            assert_eq!(
                benchmark_job.request_tag(),
                benchmark_request.tag().unwrap()
            );
            assert_eq!(
                benchmark_job.benchmark_set(),
                collector_config.benchmark_set()
            );
            assert_eq!(
                benchmark_job.collector_name().unwrap(),
                collector_config.name(),
            );

            assert_eq!(
                artifact_id,
                ArtifactId::Commit(Commit {
                    sha: "sha-1".to_string(),
                    date: Date(time),
                    r#type: CommitType::Master,
                })
            );

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn mark_request_as_complete_empty() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();

            let insert_result = db
                .add_collector_config("collector-1", Target::X86_64UnknownLinuxGnu, 1, true)
                .await;
            assert!(insert_result.is_ok());

            let benchmark_request =
                BenchmarkRequest::create_master("sha-1", "parent-sha-1", 42, time);
            db.insert_benchmark_request(&benchmark_request)
                .await
                .unwrap();
            assert!(db
                .maybe_mark_benchmark_request_as_completed("sha-1")
                .await
                .unwrap());
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn mark_request_as_complete() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
            let benchmark_set = BenchmarkSet(0u32);
            let tag = "sha-1";
            let collector_name = "collector-1";
            let target = Target::X86_64UnknownLinuxGnu;

            let insert_result = db
                .add_collector_config(collector_name, target, 1, true)
                .await;
            assert!(insert_result.is_ok());

            /* Create the request */
            let benchmark_request = BenchmarkRequest::create_release(tag, time);
            db.insert_benchmark_request(&benchmark_request)
                .await
                .unwrap();

            /* Create job for the request */
            db.enqueue_benchmark_job(
                benchmark_request.tag().unwrap(),
                target,
                CodegenBackend::Llvm,
                Profile::Opt,
                benchmark_set.0,
                BenchmarkJobKind::Runtime,
            )
            .await
            .unwrap();

            let (job, _) = db
                .dequeue_benchmark_job(collector_name, target, benchmark_set)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(job.request_tag(), benchmark_request.tag().unwrap());

            /* Make the job take some amount of time */
            std::thread::sleep(Duration::from_millis(1000));

            /* Mark the job as complete */
            db.mark_benchmark_job_as_completed(job.id(), BenchmarkJobConclusion::Success)
                .await
                .unwrap();

            db.maybe_mark_benchmark_request_as_completed(tag)
                .await
                .unwrap();

            /* From the status page view we can see that the duration has been
             * updated. Albeit that it will be a very short duration. */
            let completed = db.get_last_n_completed_benchmark_requests(1).await.unwrap();
            let req = &completed
                .iter()
                .find(|it| it.request.tag() == Some(tag))
                .unwrap()
                .request;

            assert!(matches!(
                req.status(),
                BenchmarkRequestStatus::Completed { .. }
            ));
            let BenchmarkRequestStatus::Completed { duration, .. } = req.status() else {
                unreachable!();
            };
            assert!(duration >= Duration::from_millis(1000));

            let completed_index = db.load_benchmark_request_index().await.unwrap();
            assert!(completed_index.contains_tag("sha-1"));

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_collector_configs() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();
            let target = Target::X86_64UnknownLinuxGnu;

            let benchmark_set_one = BenchmarkSet(0u32);
            let collector_name_one = "collector-1";
            db.add_collector_config(collector_name_one, target, benchmark_set_one.0, true)
                .await
                .unwrap();

            let benchmark_set_two = BenchmarkSet(1u32);
            let collector_name_two = "collector-2";
            db.add_collector_config(collector_name_two, target, benchmark_set_two.0, true)
                .await
                .unwrap();

            let collector_configs = db.get_collector_configs().await;
            assert!(collector_configs.is_ok());
            let collector_configs = collector_configs.unwrap();

            assert_eq!(collector_configs[0].name(), collector_name_one);
            assert_eq!(collector_configs[0].benchmark_set(), benchmark_set_one);
            assert!(collector_configs[0].is_active());

            assert_eq!(collector_configs[1].name(), collector_name_two);
            assert_eq!(collector_configs[1].benchmark_set(), benchmark_set_two);
            assert!(collector_configs[1].is_active());

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_last_completed_requests() {
        run_postgres_test(|ctx| async {
            let mut requests = vec![];
            let db = ctx.db();

            let collector = ctx.add_collector(Default::default()).await;

            // Create several completed requests
            for id in 1..=3 {
                // Make some space between completions
                tokio::time::sleep(Duration::from_millis(100)).await;

                requests.push(
                    RequestBuilder::master(db, &format!("sha{id}"), &format!("sha{}", id - 1), id)
                        .await
                        .add_job(db, job())
                        .await
                        .complete(db, &collector)
                        .await,
                );
            }

            // Create an additional non-completed request
            ctx.insert_master_request("foo", "bar", 1000).await;

            // Request 1 will have artifact with errors
            let aid1 = ctx.upsert_master_artifact("sha1").await;
            db.record_error(aid1, "crate1", "error1", None).await;
            db.record_error(aid1, "crate2", "error2", None).await;

            // Request 2 will have artifact without errors
            let _aid2 = ctx.upsert_master_artifact("sha2").await;

            // Request 3 will have no artifact (shouldn't happen in practice, but...)

            let reqs = db.get_last_n_completed_benchmark_requests(5).await.unwrap();
            assert_eq!(reqs.len(), 3);

            let expected = [
                ("sha3", HashMap::new()),
                ("sha2", HashMap::new()),
                (
                    "sha1",
                    HashMap::from([
                        ("crate1".to_string(), "error1".to_string()),
                        ("crate2".to_string(), "error2".to_string()),
                    ]),
                ),
            ];
            for ((sha, errors), req) in expected.into_iter().zip(reqs) {
                assert_eq!(
                    req.request.tag().unwrap(),
                    sha,
                    "Request {req:?} does not have expected sha {sha}"
                );
                assert_eq!(
                    req.errors, errors,
                    "Request {req:?} does not have expected errors {errors:?}"
                );
            }

            let reqs = db.get_last_n_completed_benchmark_requests(1).await.unwrap();
            assert_eq!(reqs.len(), 1);
            assert_eq!(reqs[0].request.tag().unwrap(), "sha3");

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_in_progress_jobs() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let collector = ctx.add_collector(Default::default()).await;

            // Artifacts ready request, should be ignored
            RequestBuilder::master(db, "foo", "bar", 1000).await;

            // Create a completed parent with jobs
            let completed = RequestBuilder::master(db, "sha4-parent", "sha0", 1001)
                .await
                .add_jobs(
                    db,
                    &[job().profile(Profile::Doc), job().profile(Profile::Opt)],
                )
                .await
                .complete(db, &collector)
                .await;

            // In progress request without a parent
            let req1 = RequestBuilder::master(db, "sha1", "sha0", 1)
                .await
                .set_in_progress(db)
                .await;

            // In progress request with a parent that has no jobs
            let req2 = RequestBuilder::master(db, "sha2", "sha1", 2)
                .await
                .add_jobs(
                    db,
                    &[job().profile(Profile::Check), job().profile(Profile::Debug)],
                )
                .await
                .set_in_progress(db)
                .await;

            // In progress request with a parent that has jobs
            let req3 = RequestBuilder::master(db, "sha3", "sha2", 3)
                .await
                .add_jobs(
                    db,
                    &[job().profile(Profile::Doc), job().profile(Profile::Opt)],
                )
                .await
                .set_in_progress(db)
                .await;

            // In progress request with a parent that has jobs, but is completed
            let req4 = RequestBuilder::master(db, "sha4", completed.tag(), 4)
                .await
                .add_jobs(
                    db,
                    &[job().profile(Profile::Doc), job().profile(Profile::Check)],
                )
                .await
                .set_in_progress(db)
                .await;

            let mut reqs = db
                .get_jobs_of_in_progress_benchmark_requests()
                .await
                .unwrap();

            // Check that all jobs are unique
            let mut job_ids = HashSet::new();
            for job in reqs.values().flatten() {
                assert!(job_ids.insert(job.id));
            }

            // Check that all jobs were returned
            assert!(!reqs.contains_key(req1.tag()));
            req2.assert_has_exact_jobs(&reqs.remove(req2.tag()).unwrap());
            req3.assert_has_exact_jobs(&reqs.remove(req3.tag()).unwrap());
            req4.assert_has_exact_jobs(&reqs.remove(req4.tag()).unwrap());
            completed.assert_has_exact_jobs(&reqs.remove(completed.tag()).unwrap());
            assert!(reqs.is_empty());

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn enqueue_parent_benchmark_job() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            let (violates_foreign_key, _) = db
                .enqueue_parent_benchmark_job(
                    "sha-0",
                    Target::X86_64UnknownLinuxGnu,
                    CodegenBackend::Llvm,
                    Profile::Debug,
                    0,
                    BenchmarkJobKind::Runtime,
                )
                .await;

            assert!(violates_foreign_key);

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn purge_artifact() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();

            ctx.upsert_master_artifact("foo").await;
            ctx.insert_master_request("foo", "bar", 1).await;
            db.enqueue_benchmark_job(
                "foo",
                Target::X86_64UnknownLinuxGnu,
                CodegenBackend::Llvm,
                Profile::Check,
                0,
                BenchmarkJobKind::Compiletime,
            )
            .await
            .unwrap()
            .unwrap();
            db.purge_artifact(&ArtifactId::Tag("foo".to_string())).await;

            assert!(!db
                .load_benchmark_request_index()
                .await
                .unwrap()
                .contains_tag("foo"));

            let collector = ctx.add_collector(CollectorBuilder::default()).await;
            assert!(db
                .dequeue_benchmark_job(
                    collector.name(),
                    collector.target(),
                    collector.benchmark_set(),
                )
                .await
                .unwrap()
                .is_none());

            Ok(ctx)
        })
        .await;
    }
}
