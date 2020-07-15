use crate::pool::{Connection, ConnectionManager, ManagedConnection, Transaction};
use crate::{ArtifactIdNumber, Index, QueryDatum, QueuedCommit};
use crate::{CollectionId, Commit, Crate, Date, Profile};
use chrono::{TimeZone, Utc};
use hashbrown::HashMap;
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

static MIGRATIONS: &[&str] = &[
    "",
    r#"
    create table benchmark(
        name text primary key,
        -- Whether this benchmark supports stable
        stabilized bool not null
    );
    create table artifact(
        id integer primary key not null,
        name text not null unique,
        date integer,
        type text not null
    );
    create table collection(
        id integer primary key not null
    );
    create table error_series(
        id integer primary key not null,
        crate text not null unique references benchmark(name) on delete cascade on update cascade
    );
    create table error(
        series integer not null references error_series(id) on delete cascade on update cascade,
        aid integer not null references artifact(id) on delete cascade on update cascade,
        error text,
        PRIMARY KEY(series, aid)
    );
    create table pstat_series(
        id integer primary key not null,
        crate text not null references benchmark(name) on delete cascade on update cascade,
        profile text not null,
        cache text not null,
        statistic text not null,
        UNIQUE(crate, profile, cache, statistic)
    );
    create table pstat(
        series integer references pstat_series(id) on delete cascade on update cascade,
        aid integer references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        value double not null,
        PRIMARY KEY(series, aid, cid)
    );
    create table self_profile_query_series(
        id integer primary key not null,
        crate text not null references benchmark(name) on delete cascade on update cascade,
        profile text not null,
        cache text not null,
        query text not null,
        UNIQUE(crate, profile, cache, query)
    );
    create table self_profile_query(
        series integer references self_profile_query_series(id) on delete cascade on update cascade,
        aid integer references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        self_time integer,
        blocked_time integer,
        incremental_load_time integer,
        number_of_cache_hits integer,
        invocation_count integer,
        PRIMARY KEY(series, aid, cid)
    );
    create table pull_request_builds(
        bors_sha text unique,
        pr integer not null,
        parent_sha text,
        complete boolean,
        requested timestamp without time zone
    );
    "#,
];

#[async_trait::async_trait]
impl ConnectionManager for Sqlite {
    type Connection = Mutex<rusqlite::Connection>;
    async fn open(&self) -> Self::Connection {
        let mut conn = rusqlite::Connection::open(&self.0).unwrap();
        conn.pragma_update(None, "cache_size", &-128000).unwrap();
        conn.pragma_update(None, "journal_mode", &"WAL").unwrap();
        conn.pragma_update(None, "foreign_keys", &"ON").unwrap();

        self.1.call_once(|| {
            let version: i32 = conn
                .query_row(
                    "select user_version from pragma_user_version;",
                    params![],
                    |row| row.get(0),
                )
                .unwrap();
            for mid in (version as usize + 1)..MIGRATIONS.len() {
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
            create index if not exists pstat_on_series_aid on pstat(series, aid);
            create index if not exists self_profile_query_on_series_aid on self_profile_query(series, aid);
        ").unwrap();
    }

    async fn transaction(&mut self) -> Box<dyn Transaction + '_> {
        self.raw().execute_batch("BEGIN DEFERRED").unwrap();
        Box::new(SqliteTransaction {
            conn: self,
            finished: false,
        })
    }

    async fn load_index(&mut self) -> Index {
        let commits = self
            .raw()
            .prepare("select id, name, date from artifact where type = 'master' or type = 'try'")
            .unwrap()
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, i16>(0)? as u32,
                    Commit {
                        sha: row.get::<_, String>(1)?.as_str().into(),
                        date: {
                            let timestamp: Option<i64> = row.get(2)?;
                            match timestamp {
                                Some(t) => Date(Utc.timestamp(t, 0)),
                                None => Date(Utc.ymd(2001, 01, 01).and_hms(0, 0, 0)),
                            }
                        },
                    },
                ))
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let artifacts = self
            .raw()
            .prepare("select id, name from artifact where type = 'release'")
            .unwrap()
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, i16>(0)? as u32,
                    row.get::<_, String>(1)?.into_boxed_str(),
                ))
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let queries = self
            .raw()
            .prepare("select id, crate, profile, cache, query from self_profile_query_series;")
            .unwrap()
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, i32>(0)? as u32,
                    (
                        Crate::from(row.get::<_, String>(1)?.as_str()),
                        match row.get::<_, String>(2)?.as_str() {
                            "check" => Profile::Check,
                            "opt" => Profile::Opt,
                            "debug" => Profile::Debug,
                            "doc" => Profile::Doc,
                            o => unreachable!("{}: not a profile", o),
                        },
                        row.get::<_, String>(3)?.as_str().parse().unwrap(),
                        row.get::<_, String>(4)?.as_str().into(),
                    ),
                ))
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let errors = self
            .raw()
            .prepare("select id, crate from error_series")
            .unwrap()
            .query_map(params![], |row| {
                Ok((
                    row.get::<_, i32>(0)? as u32,
                    row.get::<_, String>(1)?.as_str().into(),
                ))
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        Index {
            commits,
            artifacts,
            errors,
            pstats: self
                .raw()
                .prepare("select id, crate, profile, cache, statistic from pstat_series;")
                .unwrap()
                .query_map(params![], |row| {
                    Ok((
                        row.get::<_, i32>(0)? as u32,
                        (
                            Crate::from(row.get::<_, String>(1)?.as_str()),
                            match row.get::<_, String>(2)?.as_str() {
                                "check" => Profile::Check,
                                "opt" => Profile::Opt,
                                "debug" => Profile::Debug,
                                "doc" => Profile::Doc,
                                o => unreachable!("{}: not a profile", o),
                            },
                            row.get::<_, String>(3)?.as_str().parse().unwrap(),
                            row.get::<_, String>(4)?.as_str().into(),
                        ),
                    ))
                })
                .unwrap()
                .map(|r| r.unwrap())
                .collect(),
            queries,
        }
    }

    async fn get_pstats(
        &self,
        series: &[u32],
        cids: &[Option<ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let conn = self.raw_ref();
        let mut query = conn
            .prepare_cached("select min(value) from pstat where series = ? and aid = ?;")
            .unwrap();
        series
            .iter()
            .map(|sid| {
                let elements = cids
                    .iter()
                    .map(|cid| {
                        cid.and_then(|cid| {
                            query
                                .query_row(params![&sid, &cid.0], |row| row.get(0))
                                .unwrap_or_else(|e| {
                                    panic!("{:?}: series={:?}, aid={:?}", e, sid, cid);
                                })
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
        cid: ArtifactIdNumber,
    ) -> Option<QueryDatum> {
        self.raw_ref().prepare_cached("
                select self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count
                    from self_profile_query
                    where series = ? and aid = ? order by self_time asc;").unwrap()
            .query_row(params![&series, &cid.0], |row| {
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
    async fn get_error(&self, cid: crate::ArtifactIdNumber) -> HashMap<String, Option<String>> {
        self.raw_ref()
            .prepare_cached(
                "select crate, error from error_series
                    left join error on error.series = error_series.id and aid = ?",
            )
            .unwrap()
            .query_map(params![&cid.0], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    }
    async fn queue_pr(&self, pr: u32) {
        self.raw_ref()
            .prepare_cached(
                "insert into pull_request_builds (pr, complete, requested) VALUES (?, 0, now)",
            )
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
    async fn collection_id(&self) -> CollectionId {
        let raw = self.raw_ref();
        raw.execute("insert into collection default values", params![])
            .unwrap();
        CollectionId(
            raw.query_row(
                "select id from collection where rowid = last_insert_rowid()",
                params![],
                |r| r.get(0),
            )
            .unwrap(),
        )
    }

    async fn record_statistic(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        profile: Profile,
        cache: crate::Cache,
        statistic: &str,
        value: f64,
    ) {
        let profile = profile.to_string();
        let cache = cache.to_string();
        self.raw_ref().execute("insert or ignore into pstat_series (crate, profile, cache, statistic) VALUES (?, ?, ?, ?)", params![
            &krate,
            &profile,
            &cache,
            &statistic,
        ]).unwrap();
        let sid: i32 = self.raw_ref().query_row("select id from pstat_series where crate = ? and profile = ? and cache = ? and statistic = ?", params![
            &krate,
            &profile,
            &cache,
            &statistic,
        ], |r| r.get(0)).unwrap();
        self.raw_ref()
            .execute(
                "insert into pstat (series, aid, cid, value) VALUES (?, ?, ?, ?)",
                params![&sid, &artifact.0, &collection.0, &value],
            )
            .unwrap();
    }

    async fn artifact_id(&self, artifact: &crate::ArtifactId) -> ArtifactIdNumber {
        let (name, date, ty) = match artifact {
            crate::ArtifactId::Commit(commit) => (
                commit.sha.to_string(),
                if commit.is_try() {
                    None
                } else {
                    Some(commit.date.0)
                },
                if commit.is_try() { "try" } else { "master" },
            ),
            crate::ArtifactId::Artifact(a) => (a.clone(), None, "release"),
        };

        self.raw_ref()
            .execute(
                "insert or ignore into artifact (name, date, type) VALUES (?, ?, ?)",
                params![&name, &date.map(|d| d.timestamp()), &ty,],
            )
            .unwrap();
        ArtifactIdNumber(
            self.raw_ref()
                .query_row(
                    "select id from artifact where name = $1",
                    params![&name],
                    |r| r.get::<_, i16>(0),
                )
                .unwrap() as u32,
        )
    }

    async fn record_self_profile_query(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        profile: Profile,
        cache: crate::Cache,
        query: &str,
        qd: QueryDatum,
    ) {
        let profile = profile.to_string();
        let cache = cache.to_string();
        self.raw_ref().execute("insert or ignore into self_profile_query_series (crate, profile, cache, query) VALUES (?, ?, ?, ?)", params![
            &krate,
            &profile,
            &cache,
            &query,
        ]).unwrap();
        let sid: i32 = self.raw_ref().query_row("select id from self_profile_query_series where crate = ? and profile = ? and cache = ? and query = ?", params![
            &krate,
            &profile,
            &cache,
            &query,
        ], |r| r.get(0)).unwrap();
        self.raw_ref()
            .prepare_cached(
                "insert into self_profile_query(
                    series,
                    aid,
                    cid,
                    self_time,
                    blocked_time,
                    incremental_load_time,
                    number_of_cache_hits,
                    invocation_count
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .unwrap()
            .execute(params![
                &sid,
                &artifact.0,
                &collection.0,
                &i64::try_from(qd.self_time.as_nanos()).unwrap(),
                &i64::try_from(qd.blocked_time.as_nanos()).unwrap(),
                &i64::try_from(qd.incremental_load_time.as_nanos()).unwrap(),
                qd.number_of_cache_hits,
                qd.invocation_count,
            ])
            .unwrap();
    }
    async fn record_error(&self, artifact: ArtifactIdNumber, krate: &str, error: &str) {
        self.raw_ref()
            .execute(
                "insert or ignore into error_series (crate) VALUES (?)",
                params![&krate,],
            )
            .unwrap();
        let sid: i32 = self
            .raw_ref()
            .query_row(
                "select id from error_series where crate = ?",
                params![&krate,],
                |r| r.get(0),
            )
            .unwrap();
        self.raw_ref()
            .execute(
                "insert into error (series, aid, error) VALUES (?, ?, ?)",
                params![&sid, &artifact.0, &error],
            )
            .unwrap();
    }
    async fn record_benchmark(&self, krate: &str, supports_stable: bool) {
        self.raw_ref()
            .execute(
                "insert into benchmark (name, stabilized) VALUES (?, ?)
                ON CONFLICT (name) do update set stabilized = excluded.stabilized",
                params![krate, supports_stable],
            )
            .unwrap();
    }
}
