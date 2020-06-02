use crate::{ArtifactIdNumber, Index, QueryDatum, QueuedCommit};
use hashbrown::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub mod both;
pub mod postgres;
pub mod sqlite;

#[async_trait::async_trait]
pub trait Connection: Send + Sync {
    async fn maybe_create_indices(&mut self);
    async fn transaction(&mut self) -> Box<dyn Transaction + '_>;

    async fn load_index(&mut self) -> Index;

    async fn get_pstats(
        &self,
        series: &[u32],
        cid: &[Option<ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>>;
    async fn insert_pstat(&self, series: u32, cid: ArtifactIdNumber, stat: f64);
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: ArtifactIdNumber,
    ) -> Option<QueryDatum>;
    async fn insert_self_profile_query(&self, series: u32, cid: ArtifactIdNumber, data: QueryDatum);
    async fn get_error(&self, cid: ArtifactIdNumber) -> HashMap<String, Option<String>>;
    async fn insert_error(&self, series: u32, cid: ArtifactIdNumber, text: String);

    async fn queue_pr(&self, pr: u32);
    /// Returns true if this PR was queued waiting for a commit
    async fn pr_attach_commit(&self, pr: u32, sha: &str, parent_sha: &str) -> bool;
    async fn queued_commits(&self) -> Vec<QueuedCommit>;
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit>;
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
    Both {
        sqlite: ConnectionPool<sqlite::Sqlite>,
        postgres: ConnectionPool<postgres::Postgres>,
    },
}

impl Pool {
    pub async fn connection(&self) -> Box<dyn Connection> {
        match self {
            Pool::Sqlite(p) => Box::new(sqlite::SqliteConnection::new(p.get().await)),
            Pool::Postgres(p) => Box::new(p.get().await),
            Pool::Both { sqlite, postgres } => Box::new(both::BothConnection::new(
                sqlite::SqliteConnection::new(sqlite.get().await),
                postgres.get().await,
            )),
        }
    }

    pub fn open(uri: &str) -> Pool {
        if uri.starts_with("postgres") {
            Pool::Postgres(ConnectionPool::new(postgres::Postgres::new(uri.into())))
        } else if uri.starts_with("both://") {
            let mut parts = uri["both://".len()..].rsplitn(2, ';');
            let p1 = parts.next().unwrap();
            let p2 = parts.next().unwrap();
            match (Pool::open(p1), Pool::open(p2)) {
                (Pool::Sqlite(s), Pool::Postgres(p)) | (Pool::Postgres(p), Pool::Sqlite(s)) => {
                    Pool::Both {
                        sqlite: s,
                        postgres: p,
                    }
                }
                _ => panic!(
                    "unsupported inputs, must be sqlite and postgres: {} and {}",
                    p1, p2
                ),
            }
        } else {
            Pool::Sqlite(ConnectionPool::new(sqlite::Sqlite::new(uri.into())))
        }
    }
}
