use crate::selector::CompileTestCase;
use crate::{
    ArtifactCollection, ArtifactId, ArtifactIdNumber, BenchmarkRequest, CodegenBackend,
    CompileBenchmark, Target,
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
    async fn record_error(&self, artifact: ArtifactIdNumber, krate: &str, error: &str);
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
    /// exists it will be ignored
    async fn insert_benchmark_request(&self, benchmark_request: &BenchmarkRequest);

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
    use crate::tests::run_postgres_test;
    use crate::{tests::run_db_test, BenchmarkRequestStatus, Commit, CommitType, Date};
    use chrono::Utc;
    use std::str::FromStr;

    /// Create a Commit
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
            let db = ctx.db_client();
            let result = db.connection().await.get_pstats(&vec![], &vec![]).await;
            let expected: Vec<Vec<Option<f64>>> = vec![];

            assert_eq!(result, expected);
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn artifact_storage() {
        run_db_test(|ctx| async {
            let db = ctx.db_client();
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();

            let artifact_one = ArtifactId::from(create_commit("abc", time, CommitType::Master));
            let artifact_two = ArtifactId::Tag("nightly-2025-05-14".to_string());

            let artifact_one_id_number = db.connection().await.artifact_id(&artifact_one).await;
            let artifact_two_id_number = db.connection().await.artifact_id(&artifact_two).await;

            // We cannot arbitrarily add random sizes to the artifact size
            // table, as there is a constraint that the artifact must actually
            // exist before attaching something to it.

            let db = db.connection().await;

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

    #[tokio::test]
    async fn insert_benchmark_requests() {
        run_postgres_test(|ctx| async {
            let db = ctx.db_client();
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();
            let master_benchmark_request = BenchmarkRequest::create_master(
                "a-sha-1",
                "parent-sha-1",
                42,
                time,
                BenchmarkRequestStatus::ArtifactsReady,
                "llvm",
                "",
            );

            let try_benchmark_request = BenchmarkRequest::create_try(
                "b-sha-2",
                "parent-sha-2",
                32,
                time,
                BenchmarkRequestStatus::ArtifactsReady,
                "cranelift",
                "",
            );

            let release_benchmark_request = BenchmarkRequest::create_release(
                "1.8.0",
                time,
                BenchmarkRequestStatus::ArtifactsReady,
                "cranelift,llvm",
                "",
            );

            let db = db.connection().await;
            db.insert_benchmark_request(&master_benchmark_request).await;
            db.insert_benchmark_request(&try_benchmark_request).await;
            db.insert_benchmark_request(&release_benchmark_request)
                .await;
            // duplicate insert
            db.insert_benchmark_request(&master_benchmark_request).await;

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn get_compile_test_cases_with_data() {
        run_db_test(|ctx| async {
            let db = ctx.db_client().connection().await;

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
}
