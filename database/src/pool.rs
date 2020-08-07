use crate::{ArtifactId, ArtifactIdNumber};
use crate::{Cache, CollectionId, Index, Profile, QueryDatum, QueuedCommit, Step};
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

    /// This records the duration of a collection run, i.e., collecting all of
    /// the statistics for a particular artifact.
    async fn record_duration(&self, artifact: ArtifactIdNumber, duration: Duration);

    async fn collection_id(&self, version: &str) -> CollectionId;
    async fn artifact_id(&self, artifact: &ArtifactId) -> ArtifactIdNumber;
    async fn record_benchmark(&self, krate: &str, supports_stable: bool);
    async fn record_statistic(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        profile: Profile,
        cache: Cache,
        statistic: &str,
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
        krate: &str,
        profile: Profile,
        cache: Cache,
    );
    async fn record_self_profile_query(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        profile: Profile,
        cache: Cache,
        query: &str,
        qd: QueryDatum,
    );
    async fn record_error(&self, artifact: ArtifactIdNumber, krate: &str, error: &str);

    async fn get_pstats(
        &self,
        series: &[u32],
        cid: &[Option<ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>>;
    async fn get_self_profile(
        &self,
        cid: ArtifactIdNumber,
        crate_: &str,
        profile: &str,
        cache: &str,
    ) -> HashMap<crate::QueryLabel, QueryDatum>;
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: ArtifactIdNumber,
    ) -> Option<QueryDatum>;
    async fn get_error(&self, cid: ArtifactIdNumber) -> HashMap<String, Option<String>>;

    async fn queue_pr(&self, pr: u32, include: Option<&str>, exclude: Option<&str>);
    /// Returns true if this PR was queued waiting for a commit
    async fn pr_attach_commit(&self, pr: u32, sha: &str, parent_sha: &str) -> bool;
    async fn queued_commits(&self) -> Vec<QueuedCommit>;
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit>;

    // Collector status API

    async fn collector_start(&self, aid: ArtifactIdNumber, steps: &[String]);

    // Returns `true` if the step was started, i.e., it did not previously have
    // an end. Otherwise returns false, indicating that we can skip it.
    async fn collector_start_step(&self, aid: ArtifactIdNumber, step: &str) -> bool;
    async fn collector_end_step(&self, aid: ArtifactIdNumber, step: &str);

    async fn in_progress_artifacts(&self) -> Vec<ArtifactId>;

    async fn in_progress_steps(&self, aid: &ArtifactId) -> Vec<Step>;

    async fn last_end_time(&self) -> Option<DateTime<Utc>>;

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
        let permit = self.permits.clone().acquire_owned().await;
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
