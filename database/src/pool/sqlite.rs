use crate::pool::{Connection, ConnectionManager, ManagedConnection, Transaction};
use crate::{CollectionIdNumber, QueryDatum, QueuedCommit};
use rusqlite::params;
use rusqlite::OptionalExtension;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::Once;
use std::time::Duration;

pub struct SqliteTransaction<'a> {
    conn: &'a mut SqliteConnection,
    finished: bool,
}

#[async_trait::async_trait]
impl<'a> Transaction for SqliteTransaction<'a> {
    async fn commit(mut self: Box<Self>) -> Result<(), anyhow::Error> {
        self.finished = true;
        Ok(self.conn.raw().execute_batch("COMMIT")?)
    }

    async fn finish(mut self: Box<Self>) -> Result<(), anyhow::Error> {
        self.finished = true;
        Ok(self.conn.raw().execute_batch("ROLLBACK")?)
    }
    fn conn(&mut self) -> &mut dyn Connection {
        &mut *self.conn
    }
    fn conn_ref(&self) -> &dyn Connection {
        &*self.conn
    }
}

impl std::ops::Deref for SqliteTransaction<'_> {
    type Target = dyn Connection;
    fn deref(&self) -> &Self::Target {
        &*self.conn
    }
}

impl std::ops::DerefMut for SqliteTransaction<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn
    }
}

impl Drop for SqliteTransaction<'_> {
    fn drop(&mut self) {
        if !self.finished {
            self.conn.raw().execute_batch("ROLLBACK").unwrap();
        }
    }
}

pub struct Sqlite(PathBuf, Once);

impl Sqlite {
    pub fn new(path: PathBuf) -> Self {
        Sqlite(path, Once::new())
    }
}

static MIGRATIONS: &[&str] = &["
    create table interned(name text primary key, value blob);
    create table errors(series integer, cid integer, value text);
    create table pstat(series integer, cid integer, value real);
    create table self_profile_query(
        series integer,
        cid integer,
        self_time integer,
        blocked_time integer,
        incremental_load_time integer,
        number_of_cache_hits integer,
        invocation_count integer
    );
    create table pull_request_builds(
        pr integer not null,
        bors_sha text unique,
        parent_sha text,
        complete boolean,
        requested integer -- timestamp
    );
    "];

#[async_trait::async_trait]
impl ConnectionManager for Sqlite {
    type Connection = Mutex<rusqlite::Connection>;
    async fn open(&self) -> Self::Connection {
        let mut conn = rusqlite::Connection::open(&self.0).unwrap();
        conn.pragma_update(None, "cache_size", &-128000).unwrap();
        conn.pragma_update(None, "journal_mode", &"WAL").unwrap();

        self.1.call_once(|| {
            let version: i32 = conn
                .query_row(
                    "select user_version from pragma_user_version;",
                    params![],
                    |row| row.get(0),
                )
                .unwrap();
            for mid in (version as usize)..MIGRATIONS.len() {
                let sql = MIGRATIONS[mid];
                let tx = conn.transaction().unwrap();
                tx.execute_batch(&sql).unwrap();
                tx.pragma_update(None, "user_version", &(mid as i32))
                    .unwrap();
                tx.commit().unwrap();
            }
        });

        Mutex::new(conn)
    }
    async fn is_valid(&self, conn: &mut Self::Connection) -> bool {
        conn.get_mut()
            .unwrap_or_else(|e| e.into_inner())
            .execute_batch("")
            .is_ok()
    }
}

pub struct SqliteConnection {
    conn: ManagedConnection<Mutex<rusqlite::Connection>>,
}

fn assert_sync<T: Sync>() {}

impl SqliteConnection {
    pub fn new(conn: ManagedConnection<Mutex<rusqlite::Connection>>) -> Self {
        assert_sync::<Self>();
        Self { conn }
    }

    pub fn raw(&mut self) -> &mut rusqlite::Connection {
        self.conn.get_mut().unwrap_or_else(|e| e.into_inner())
    }
    pub fn raw_ref(&self) -> std::sync::MutexGuard<rusqlite::Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }
}

#[async_trait::async_trait]
impl Connection for SqliteConnection {
    async fn maybe_create_indices(&mut self) {
        self.raw().execute_batch("
            create index if not exists pstat_on_series_cid on pstat(series, cid);
            create index if not exists self_profile_query_on_series_cid on self_profile_query(series, cid);
        ").unwrap();
    }

    async fn transaction(&mut self) -> Box<dyn Transaction + '_> {
        self.raw().execute_batch("BEGIN DEFERRED").unwrap();
        Box::new(SqliteTransaction {
            conn: self,
            finished: false,
        })
    }

    async fn load_index(&mut self) -> Option<Vec<u8>> {
        let indices: rusqlite::Result<Option<Vec<u8>>> = self
            .raw()
            .query_row(
                "select value from interned where name = 'index'",
                params![],
                |row| row.get(0),
            )
            .optional();
        match indices {
            Ok(Some(s)) => Some(s),
            Ok(None) => None,
            Err(e) => {
                if e.to_string().contains("no such table: interned") {
                    None
                } else {
                    panic!("could not load index: {}", e);
                }
            }
        }
    }

    async fn store_index(&mut self, index: &[u8]) {
        self.raw()
            .execute(
                "insert or replace into interned (name, value) VALUES ('index', ?)",
                params![&index],
            )
            .unwrap();
    }
    async fn insert_pstat(&self, series: u32, cid: CollectionIdNumber, stat: f64) {
        self.raw_ref()
            .prepare_cached("insert into pstat(series, cid, value) VALUES (?, ?, ?)")
            .unwrap()
            .execute(params![&series, &cid.pack(), stat])
            .unwrap();
    }
    async fn insert_self_profile_query(
        &self,
        series: u32,
        cid: CollectionIdNumber,
        data: QueryDatum,
    ) {
        self.raw_ref()
            .prepare_cached(
                "insert into self_profile_query(
                        series, cid,
                        self_time,
                        blocked_time,
                        incremental_load_time,
                        number_of_cache_hits,
                        invocation_count
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .unwrap()
            .execute(params![
                &series,
                &cid.pack(),
                &i64::try_from(data.self_time.as_nanos()).unwrap(),
                &i64::try_from(data.blocked_time.as_nanos()).unwrap(),
                &i64::try_from(data.incremental_load_time.as_nanos()).unwrap(),
                data.number_of_cache_hits,
                data.invocation_count,
            ])
            .unwrap();
    }
    async fn insert_error(&self, series: u32, cid: CollectionIdNumber, text: String) {
        let cid = cid.pack();
        self.raw_ref()
            .prepare_cached(
                "insert into errors(
                        series, cid, value
                    ) VALUES (?, ?, ?)",
            )
            .unwrap()
            .execute(params![&series, &cid, text,])
            .unwrap();
    }
    async fn get_pstats(
        &self,
        series: &[u32],
        cids: &[Option<CollectionIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        series
            .iter()
            .map(|sid| {
                let elements = cids
                    .iter()
                    .map(|cid| {
                        cid.and_then(|cid| {
                            let cid = cid.pack();

                            self.raw_ref()
                                .prepare_cached(
                                    "select min(value) from pstat where series = ? and cid = ?;",
                                )
                                .unwrap()
                                .query_row(params![&sid, &cid], |row| row.get(0))
                                .optional()
                                .unwrap()
                        })
                    })
                    .collect::<Vec<_>>();
                if elements.is_empty() {
                    vec![None; cids.len()]
                } else {
                    elements
                }
            })
            .collect()
    }
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<QueryDatum> {
        let cid = cid.pack();
        self.raw_ref().prepare_cached("
                select self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count
                    from self_profile_query
                    where series = ? and cid = ? order by self_time asc;").unwrap()
            .query_row(params![&series, &cid], |row| {
        let self_time: i64 = row.get(0)?;
        let blocked_time: i64 = row.get(1)?;
        let incremental_load_time: i64 = row.get(2)?;
        Ok(QueryDatum {
            self_time: Duration::from_nanos(self_time as u64),
            blocked_time: Duration::from_nanos(blocked_time as u64),
            incremental_load_time: Duration::from_nanos(incremental_load_time as u64),
            number_of_cache_hits: row.get(3)?,
            invocation_count: row.get(4)?,
        })

            })
            .optional()
            .unwrap()
    }
    async fn get_error(&self, series: u32, cid: CollectionIdNumber) -> Option<String> {
        let cid = cid.pack();
        self.raw_ref()
            .prepare_cached("select value from errors where series = ? and cid = ?;")
            .unwrap()
            .query_row(params![&series, &cid], |row| row.get(0))
            .optional()
            .unwrap()
    }
    async fn queue_pr(&self, pr: u32) {
        self.raw_ref()
            .prepare_cached("insert into pull_request_builds (pr, requested) VALUES (?, now)")
            .unwrap()
            .execute(params![pr])
            .unwrap();
    }
    async fn pr_attach_commit(&self, pr: u32, sha: &str, parent_sha: &str) -> bool {
        self.raw_ref()
            .prepare_cached(
                "update pull_request_builds SET bors_sha = ?, parent_sha = ?
                where pr = ? and bors_sha is null",
            )
            .unwrap()
            .execute(params![sha, parent_sha, pr])
            .unwrap()
            > 0
    }
    async fn queued_commits(&self) -> Vec<QueuedCommit> {
        self.raw_ref()
            .prepare_cached(
                "select pr, bors_sha, parent_sha from pull_request_builds
                where complete is false and bors_sha is not null
                order by requested asc",
            )
            .unwrap()
            .query(params![])
            .unwrap()
            .mapped(|row| {
                Ok(QueuedCommit {
                    pr: row.get(0).unwrap(),
                    sha: row.get(1).unwrap(),
                    parent_sha: row.get(2).unwrap(),
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit> {
        let count = self
            .raw_ref()
            .execute(
                "update pull_request_builds SET complete = 1 where sha = ? and complete = 0",
                params![sha],
            )
            .unwrap();
        if count == 0 {
            return None;
        }
        assert_eq!(count, 1, "sha is unique column");
        self.raw_ref()
            .query_row(
                "select pr, sha, parent_sha from pull_request_builds
            where sha = ?",
                params![sha],
                |row| {
                    Ok(QueuedCommit {
                        pr: row.get(0).unwrap(),
                        sha: row.get(1).unwrap(),
                        parent_sha: row.get(2).unwrap(),
                    })
                },
            )
            .optional()
            .unwrap()
    }
}
