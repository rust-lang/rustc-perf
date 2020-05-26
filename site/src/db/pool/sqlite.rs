use crate::db::pool::{Connection, Transaction};
use crate::db::{CollectionIdNumber, QueryDatum};
use r2d2::{ManageConnection, PooledConnection};
use rusqlite::params;
use rusqlite::OptionalExtension;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::time::Duration;

pub struct SqliteTransaction<'a> {
    conn: &'a mut SqliteConnection,
    finished: bool,
}

#[async_trait::async_trait]
impl<'a> Transaction for SqliteTransaction<'a> {
    async fn commit(&mut self) -> Result<(), anyhow::Error> {
        self.finished = true;
        Ok(self.conn.conn.execute_batch("COMMIT")?)
    }

    async fn finish(&mut self) -> Result<(), anyhow::Error> {
        self.finished = true;
        Ok(self.conn.conn.execute_batch("ROLLBACK")?)
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
            self.conn.conn.execute_batch("ROLLBACK").unwrap();
        }
    }
}

pub struct Sqlite(PathBuf);

impl Sqlite {
    pub fn new(path: PathBuf) -> Self {
        Sqlite(path)
    }
}

impl ManageConnection for Sqlite {
    type Connection = rusqlite::Connection;
    type Error = rusqlite::Error;
    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = rusqlite::Connection::open(&self.0)?;
        conn.pragma_update(None, "cache_size", &-128000)?;
        Ok(conn)
    }
    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.execute_batch("")
    }
    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

pub struct SqliteConnection {
    conn: PooledConnection<Sqlite>,
}

impl SqliteConnection {
    pub fn new(conn: PooledConnection<Sqlite>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl Connection for SqliteConnection {
    async fn maybe_create_tables(&mut self) {
        self.conn
            .execute(
                "create table if not exists interned(name text primary key, value blob);",
                params![],
            )
            .unwrap();
        self.conn
            .execute(
                "create table if not exists errors(series integer, cid integer, value text);",
                params![],
            )
            .unwrap();
        self.conn
            .execute(
                "create table if not exists pstat(series integer, cid integer, value real);",
                params![],
            )
            .unwrap();
        self.conn
            .execute(
                "create table if not exists self_profile_query(
                    series integer,
                    cid integer,
                    self_time integer,
                    blocked_time integer,
                    incremental_load_time integer,
                    number_of_cache_hits integer,
                    invocation_count integer
                );",
                params![],
            )
            .unwrap();
    }

    async fn transaction(&mut self) -> Box<dyn Transaction + '_> {
        self.conn.execute_batch("BEGIN DEFERRED").unwrap();
        Box::new(SqliteTransaction {
            conn: self,
            finished: false,
        })
    }

    async fn load_index(&mut self) -> Option<Vec<u8>> {
        let indices: rusqlite::Result<Option<Vec<u8>>> = self
            .conn
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
        self.conn
            .execute(
                "insert or replace into interned (name, value) VALUES ('index', ?)",
                params![&index],
            )
            .unwrap();
    }
    async fn insert_pstat(&mut self, series: u32, cid: CollectionIdNumber, stat: f64) {
        self.conn
            .prepare_cached("insert into pstat(series, cid, value) VALUES (?, ?, ?)")
            .unwrap()
            .execute(params![&series, &u32::from_be_bytes(cid.as_bytes()), stat])
            .unwrap();
    }
    async fn insert_self_profile_query(
        &mut self,
        series: u32,
        cid: CollectionIdNumber,
        data: &QueryDatum,
    ) {
        self.conn
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
                &u32::from_be_bytes(cid.as_bytes()),
                &i64::try_from(data.self_time.as_nanos()).unwrap(),
                &i64::try_from(data.blocked_time.as_nanos()).unwrap(),
                &i64::try_from(data.incremental_load_time.as_nanos()).unwrap(),
                data.number_of_cache_hits,
                data.invocation_count,
            ])
            .unwrap();
    }
    async fn insert_error(&mut self, series: u32, cid: CollectionIdNumber, text: &str) {
        let cid = u32::from_be_bytes(cid.as_bytes());
        self.conn
            .prepare_cached(
                "insert into errors(
                        series, cid, value
                    ) VALUES (?, ?, ?)",
            )
            .unwrap()
            .execute(params![&series, &cid, text,])
            .unwrap();
    }
    async fn get_pstat(&mut self, series: u32, cid: CollectionIdNumber) -> Option<f64> {
        let cid = u32::from_be_bytes(cid.as_bytes());

        self.conn
            .prepare_cached("select value from pstat where series = ? and cid = ?;")
            .unwrap()
            .query_row(params![&series, &cid], |row| row.get(0))
            .optional()
            .unwrap()
    }
    async fn get_self_profile_query(
        &mut self,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<QueryDatum> {
        let cid = u32::from_be_bytes(cid.as_bytes());
        self.conn.prepare_cached("
                select self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count
                    from self_profile_query
                    where series = ? and cid = ?;").unwrap()
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
    async fn get_error(&mut self, series: u32, cid: CollectionIdNumber) -> Option<String> {
        let cid = u32::from_be_bytes(cid.as_bytes());
        self.conn
            .prepare_cached("select value from errors where series = ? and cid = ?;")
            .unwrap()
            .query_row(params![&series, &cid], |row| row.get(0))
            .optional()
            .unwrap()
    }
}
