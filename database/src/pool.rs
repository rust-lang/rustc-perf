use crate::{
    ArtifactCollection, ArtifactId, ArtifactIdNumber, CodegenBackend, CommitJob, CompileBenchmark,
    Target,
};
use crate::{CollectionId, Index, Profile, QueuedCommit, Scenario, Step};
use chrono::{DateTime, Utc};
use hashbrown::HashMap;
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

    /// Add a job to the queue
    async fn enqueue_commit_job(&self, jobs: &CommitJob);

    /// Dequeue jobs, we pass `machine_id` and `target` in case there are jobs
    /// the machine was previously doing and can pick up again
    async fn take_commit_job(&self, machine_id: &str, target: Target) -> Option<CommitJob>;

    /// Mark the job as finished
    async fn finish_commit_job(&self, machine_id: &str, target: Target, sha: String);
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
    use chrono::Utc;
    use std::str::FromStr;

    use super::*;
    use crate::{tests::run_db_test, Commit, CommitJobState, CommitJobType, CommitType, Date};

    /// Create a Commit
    fn create_commit(commit_sha: &str, time: chrono::DateTime<Utc>, r#type: CommitType) -> Commit {
        Commit {
            sha: commit_sha.into(),
            date: Date(time),
            r#type,
        }
    }

    /// Create a CommitJob
    fn create_commit_job(
        sha: &str,
        parent_sha: &str,
        commit_time: chrono::DateTime<Utc>,
        target: Target,
        job_type: CommitJobType,
        state: CommitJobState,
    ) -> CommitJob {
        CommitJob {
            sha: sha.to_string(),
            parent_sha: parent_sha.to_string(),
            commit_time: Date(commit_time),
            target,
            include: None,
            exclude: None,
            runs: None,
            backends: None,
            job_type,
            state,
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
    async fn take_commit_job() {
        run_db_test(|ctx| async {
            // ORDER:
            // Releases first
            // Master commits second, order by oldest PR ascending
            // Try commits last, order by oldest PR ascending

            let db = ctx.db_client().connection().await;
            let time = chrono::DateTime::from_str("2021-09-01T00:00:00.000Z").unwrap();

            // Try commits
            let try_job_1 = create_commit_job(
                "sha1",
                "p1",
                time,
                Target::X86_64UnknownLinuxGnu,
                CommitJobType::Try { pr: 1 },
                CommitJobState::Queued,
            );
            let try_job_2 = create_commit_job(
                "sha2",
                "p2",
                time,
                Target::X86_64UnknownLinuxGnu,
                CommitJobType::Try { pr: 2 },
                CommitJobState::Queued,
            );

            // Master commits
            let master_job_1 = create_commit_job(
                "sha3",
                "p3",
                time,
                Target::X86_64UnknownLinuxGnu,
                CommitJobType::Master { pr: 3 },
                CommitJobState::Queued,
            );
            let master_job_2 = create_commit_job(
                "sha4",
                "p4",
                time,
                Target::X86_64UnknownLinuxGnu,
                CommitJobType::Master { pr: 4 },
                CommitJobState::Queued,
            );

            // Release commits
            let release_job_1 = create_commit_job(
                "sha5",
                "p5",
                time,
                Target::X86_64UnknownLinuxGnu,
                CommitJobType::Release { tag: "tag1".into() },
                CommitJobState::Queued,
            );
            let release_job_2 = create_commit_job(
                "sha6",
                "p6",
                time,
                Target::X86_64UnknownLinuxGnu,
                CommitJobType::Release { tag: "tag2".into() },
                CommitJobState::Queued,
            );

            // Shuffle the insert order a bit
            let all_commits = vec![
                release_job_1,
                master_job_2,
                try_job_1,
                release_job_2,
                master_job_1,
                try_job_2,
            ];

            // queue all the jobs
            for commit in all_commits {
                db.enqueue_commit_job(&commit).await;
            }

            // Now we test the ordering: after each dequeue we immediately mark
            // the job as finished for the sake of testing so it can't be
            // returned again in the test.
            //
            // The priority should be;
            //
            //   1. Release commits           (oldest tag first)
            //   2. Master  commits           (oldest PR first)
            //   3. Try     commits           (oldest PR first)
            //
            // Given the data we inserted above the expected SHA order is:
            // sha5, sha6, sha3, sha4, sha1, sha2.

            let machine = "machine-1";
            let target = Target::X86_64UnknownLinuxGnu;
            let expected = ["sha5", "sha6", "sha3", "sha4", "sha1", "sha2"];

            for &sha in &expected {
                let job = db.take_commit_job(machine, target).await;
                assert!(job.is_some(), "expected a job for sha {sha}");
                let job = job.unwrap();
                assert_eq!(job.sha, sha, "jobs dequeued out of priority order");

                // Mark the job finished so it is not returned again.
                db.finish_commit_job(machine, target, sha.to_string()).await;
            }

            // After all six jobs have been taken, the queue should be empty.
            assert!(
                db.take_commit_job(machine, target).await.is_none(),
                "queue should be empty after draining all jobs"
            );

            Ok(ctx)
        })
        .await;
    }
}
