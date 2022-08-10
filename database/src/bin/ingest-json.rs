//! This ingests JSON (old-style) content into a database.

use anyhow::Context as _;
use chrono::{DateTime, Utc};
use database::pool::ConnectionManager;
use database::{Benchmark, Profile, Scenario};
use database::{Commit, PatchName, Pool, QueryLabel};
use futures::stream::{FuturesUnordered, StreamExt};
use hashbrown::{HashMap, HashSet};
use rusqlite::params;
use rusqlite::types::{ToSqlOutput, ValueRef};
use serde::Deserialize;
use std::convert::TryFrom;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{future::Future, path::Path, pin::Pin, time::Duration};
use tokio_postgres::Statement;

#[derive(Deserialize)]
pub struct ArtifactData {
    pub id: String,
    // String in Result is the output of the command that failed
    pub benchmarks: HashMap<Benchmark, Result<BenchmarkRuns, String>>,
}

#[derive(Deserialize)]
pub struct CommitData {
    pub commit: Commit,
    // String in Result is the output of the command that failed
    pub benchmarks: HashMap<Benchmark, Result<BenchmarkRuns, String>>,
}

#[derive(Deserialize)]
pub struct BenchmarkRuns {
    pub runs: Vec<Run>,
    pub name: Benchmark,
}

#[derive(Deserialize)]
pub struct Run {
    pub stats: Stats,
    #[serde(default)]
    pub self_profile: Option<SelfProfile>,
    #[serde(default)]
    pub check: bool,
    pub release: bool,
    pub state: BenchmarkState,
}

#[derive(Deserialize)]
pub struct Stats {
    stats: Vec<Option<f64>>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: vec![None; 10],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, f64)> + '_ {
        self.stats.iter().enumerate().filter_map(|(idx, s)| {
            s.map(|s| {
                (
                    match idx {
                        0 => "cpu-clock:u",
                        1 => "cycles:u",
                        2 => "faults",
                        3 => "faults:u",
                        4 => "instructions:u",
                        5 => "max-rss",
                        6 => "task-clock",
                        7 => "task-clock:u",
                        8 => "wall-time",
                        9 => "cpu-clock",
                        _ => panic!("unknown id: {}", idx),
                    },
                    s,
                )
            })
        })
    }
}

#[derive(Deserialize)]
pub enum BenchmarkState {
    Clean,
    IncrementalStart,
    IncrementalClean,
    IncrementalPatched(Patch),
}

#[derive(Deserialize)]
pub struct Patch {
    pub name: PatchName,
}

#[derive(Deserialize)]
#[serde(from = "InternalSelfProfile")]
pub struct SelfProfile {
    pub query_data: Vec<QueryData>,
}

pub struct QueryData {
    pub label: QueryLabel,
    pub self_time: u64,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: u64,
    pub incremental_load_time: u64,
}

impl QueryData {
    pub fn self_time(&self) -> Duration {
        Duration::from_nanos(self.self_time)
    }

    pub fn blocked_time(&self) -> Duration {
        Duration::from_nanos(self.blocked_time)
    }

    pub fn incremental_load_time(&self) -> Duration {
        Duration::from_nanos(self.incremental_load_time)
    }
}

#[derive(Deserialize)]
struct InternalSelfProfile {
    label: Vec<QueryLabel>,
    // nanos
    self_time: Vec<u64>,
    number_of_cache_hits: Vec<u32>,
    invocation_count: Vec<u32>,
    // nanos
    blocked_time: Vec<u64>,
    // nanos
    incremental_load_time: Vec<u64>,
}

impl From<InternalSelfProfile> for SelfProfile {
    fn from(profile: InternalSelfProfile) -> SelfProfile {
        let InternalSelfProfile {
            label,
            self_time,
            number_of_cache_hits,
            invocation_count,
            blocked_time,
            incremental_load_time,
        } = profile;
        let mut query_data = Vec::with_capacity(label.len());
        let label = label.into_iter();
        let mut self_time = self_time.into_iter();
        let mut number_of_cache_hits = number_of_cache_hits.into_iter();
        let mut invocation_count = invocation_count.into_iter();
        let mut blocked_time = blocked_time.into_iter();
        let mut incremental_load_time = incremental_load_time.into_iter();
        for label in label {
            query_data.push(QueryData {
                label,
                self_time: self_time.next().unwrap(),
                number_of_cache_hits: number_of_cache_hits.next().unwrap(),
                invocation_count: invocation_count.next().unwrap(),
                blocked_time: blocked_time.next().unwrap(),
                incremental_load_time: incremental_load_time.next().unwrap(),
            });
        }
        assert_eq!(query_data.capacity(), query_data.len());
        SelfProfile { query_data }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct StatDescription {
    benchmark: Arc<String>,
    profile: &'static str,
    scenario: String,
    metric: &'static str,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct SpqSeries {
    benchmark: Arc<String>,
    profile: &'static str,
    scenario: String,
    query: &'static str,
}

#[async_trait::async_trait]
trait Ingesting {
    async fn artifact(&self, name: &str, date: Option<DateTime<Utc>>, ty: &str) -> i16;
    async fn benchmark(&self, name: &str);
    async fn collection(&self) -> i32;
    async fn error_series(&self, krate: &str) -> i32;
    async fn error(&self, series: i32, aid: i16, error: &str);
    async fn pstat_series(&self, series: &[StatDescription]) -> Vec<i32>;
    async fn pstat(&self, series: i32, aid: i16, cid: i32, value: f64);
    async fn self_profile_query_series(&self, series: &[SpqSeries]) -> Vec<i32>;
    async fn self_profile_query(&self, series: i32, aid: i16, cid: i32, qd: &QueryData);
}

struct Sqlite<'a> {
    conn: Mutex<&'a mut rusqlite::Connection>,
}

impl<'a> Sqlite<'a> {
    pub fn conn(&self) -> std::sync::MutexGuard<&'a mut rusqlite::Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }
}

#[async_trait::async_trait]
impl Ingesting for Sqlite<'_> {
    async fn artifact(&self, name: &str, date: Option<DateTime<Utc>>, ty: &str) -> i16 {
        self.conn()
            .execute(
                "insert or ignore into artifact (name, date, type) VALUES (?, ?, ?)",
                params![&name, &date.map(|d| d.timestamp()), ty],
            )
            .unwrap();
        self.conn()
            .query_row(
                "select id from artifact where name = ?",
                params![name],
                |r| r.get(0),
            )
            .unwrap()
    }
    async fn benchmark(&self, name: &str) {
        self.conn()
            .execute(
                "insert or ignore into benchmark (name, stabilized) VALUES (?, 0)",
                params![&name],
            )
            .unwrap();
    }
    async fn collection(&self) -> i32 {
        self.conn()
            .execute("insert into collection default values", params![])
            .unwrap();
        self.conn()
            .query_row(
                "select id from collection where rowid = last_insert_rowid()",
                params![],
                |r| r.get(0),
            )
            .unwrap()
    }
    async fn error_series(&self, krate: &str) -> i32 {
        self.conn()
            .execute(
                "insert or ignore into error_series (crate) VALUES (?)",
                params![&krate],
            )
            .unwrap();
        self.conn()
            .query_row(
                "select id from error_series where error_series.crate = ?",
                params![&krate],
                |r| r.get(0),
            )
            .unwrap()
    }
    async fn error(&self, series: i32, aid: i16, error: &str) {
        self.conn()
            .execute(
                "insert into error (series, aid, value) VALUES (?, ?, ?)",
                params![&series, &aid, &error],
            )
            .unwrap();
    }
    async fn pstat_series(&self, series: &[StatDescription]) -> Vec<i32> {
        let c = self.conn();
        let mut cached = c
            .prepare_cached(
                "insert into pstat_series (crate, profile, cache, statistic)
                VALUES (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?)
            ",
            )
            .unwrap();
        let mut exact = series.chunks_exact(9);
        for series in exact.by_ref() {
            cached
                .execute(rusqlite::params_from_iter(series.iter().flat_map(|v| {
                    vec![
                        ToSqlOutput::Borrowed(ValueRef::Text(v.benchmark.as_bytes())),
                        ToSqlOutput::Borrowed(ValueRef::Text(v.profile.as_bytes())),
                        ToSqlOutput::Borrowed(ValueRef::Text(v.scenario.as_bytes())),
                        ToSqlOutput::Borrowed(ValueRef::Text(v.metric.as_bytes())),
                    ]
                })))
                .unwrap();
        }
        for v in exact.remainder() {
            c.prepare_cached(
                "insert into pstat_series (crate, profile, cache, statistic)
                VALUES (?,?,?,?)
            ",
            )
            .unwrap()
            .execute(params![&v.benchmark, &v.profile, &v.scenario, &v.metric,])
            .unwrap();
        }

        series.iter().map(|s| {
            c
                .prepare_cached("select id from pstat_series where pstat_series.crate = ? and pstat_series.profile = ? and pstat_series.cache = ? and pstat_series.statistic = ?")
                .unwrap()
                .query_row(params![&s.benchmark, &s.profile, &s.scenario, &s.metric], |r| r.get(0))
                .unwrap()
        }).collect()
    }
    async fn pstat(&self, series: i32, aid: i16, cid: i32, value: f64) {
        self.conn()
            .prepare_cached("insert into pstat (series, aid, cid, value) VALUES (?,?,?,?)")
            .unwrap()
            .execute(params![&series, &aid, &cid, &value])
            .unwrap();
    }
    async fn self_profile_query_series(&self, series: &[SpqSeries]) -> Vec<i32> {
        let c = self.conn();
        let mut cached = c
            .prepare_cached(
                "insert into self_profile_query_series (crate, profile, cache, query)
                VALUES (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?),
                       (?,?,?,?)
            ",
            )
            .unwrap();
        let mut exact = series.chunks_exact(21);
        for series in exact.by_ref() {
            cached
                .execute(rusqlite::params_from_iter(series.iter().flat_map(|v| {
                    vec![
                        ToSqlOutput::Borrowed(ValueRef::Text(v.benchmark.as_bytes())),
                        ToSqlOutput::Borrowed(ValueRef::Text(v.profile.as_bytes())),
                        ToSqlOutput::Borrowed(ValueRef::Text(v.scenario.as_bytes())),
                        ToSqlOutput::Borrowed(ValueRef::Text(v.query.as_bytes())),
                    ]
                })))
                .unwrap();
        }
        for v in exact.remainder() {
            c.prepare_cached(
                "insert into self_profile_query_series (crate, profile, cache, query)
                VALUES (?,?,?,?)
            ",
            )
            .unwrap()
            .execute(params![&v.benchmark, &v.profile, &v.scenario, &v.query])
            .unwrap();
        }

        let mut cached =
            c.prepare_cached("select id from self_profile_query_series where crate = ? and profile = ? and cache = ? and query = ?")
             .unwrap();
        series
            .iter()
            .map(|s| {
                cached
                    .query_row(
                        params![&s.benchmark, &s.profile, &s.scenario, &s.query],
                        |r| r.get(0),
                    )
                    .unwrap()
            })
            .collect()
    }
    async fn self_profile_query(&self, series: i32, aid: i16, cid: i32, qd: &QueryData) {
        self.conn()
            .prepare_cached("insert into self_profile_query
                (series, aid, cid, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count)
                VALUES (?,?,?,?, ?,?,?,?)")
            .unwrap()
            .execute(params![
                &series,
                &aid,
                &cid,
                &i64::try_from(qd.self_time).unwrap(),
                &i64::try_from(qd.blocked_time).unwrap(),
                &i64::try_from(qd.incremental_load_time).unwrap(),
                qd.number_of_cache_hits,
                qd.invocation_count,
            ])
            .unwrap();
    }
}

struct Postgres<'a> {
    conn: tokio_postgres::Transaction<'a>,
    cached: Arc<CachedStatements>,
}

struct CachedStatements {
    pstat_series_insert: Statement,
    pstat_series_insert_many: Statement,
    spq_series_insert: Statement,
    spq_series_insert_many: Statement,
    pstat_insert: Statement,
    spq_insert: Statement,
}

#[async_trait::async_trait]
impl Ingesting for Postgres<'_> {
    async fn artifact(&self, name: &str, date: Option<DateTime<Utc>>, ty: &str) -> i16 {
        self.conn
            .execute(
                "insert into artifact (name, date, type) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                &[&name, &date, &ty],
            )
            .await
            .unwrap();
        self.conn
            .query_one("select id from artifact where name = $1", &[&name])
            .await
            .unwrap()
            .get(0)
    }
    async fn benchmark(&self, name: &str) {
        self.conn
            .execute(
                "insert into benchmark (name) VALUES ($1) ON CONFLICT DO NOTHING",
                &[&name],
            )
            .await
            .unwrap();
    }
    async fn collection(&self) -> i32 {
        self.conn
            .query_one("insert into collection DEFAULT VALUES returning id", &[])
            .await
            .unwrap()
            .get(0)
    }
    async fn error_series(&self, krate: &str) -> i32 {
        self.conn
            .execute(
                "insert into error_series (crate) VALUES ($1) ON CONFLICT DO NOTHING",
                &[&krate],
            )
            .await
            .unwrap();
        self.conn
            .query_one(
                "select id from error_series where error_series.crate = $1",
                &[&krate],
            )
            .await
            .unwrap()
            .get(0)
    }
    async fn error(&self, series: i32, aid: i16, error: &str) {
        self.conn
            .execute(
                "insert into error (series, aid, error) VALUES ($1, $2, $3)",
                &[&series, &aid, &error],
            )
            .await
            .unwrap();
    }
    async fn pstat_series(&self, series: &[StatDescription]) -> Vec<i32> {
        let mut res = Vec::with_capacity(series.len());
        let mut exact = series.chunks_exact(11);
        for series in exact.by_ref() {
            let params = series
                .iter()
                .flat_map(|s| {
                    vec![
                        &*s.benchmark as &(dyn tokio_postgres::types::ToSql + Sync),
                        &s.profile as &(dyn tokio_postgres::types::ToSql + Sync),
                        &s.scenario as &(dyn tokio_postgres::types::ToSql + Sync),
                        &s.metric as &(dyn tokio_postgres::types::ToSql + Sync),
                    ]
                })
                .collect::<Vec<_>>();
            let rows = self
                .conn
                .query(&self.cached.pstat_series_insert_many, &params)
                .await
                .unwrap();
            for row in rows {
                res.push(row.get(0));
            }
        }
        for s in exact.remainder() {
            let row = self
                .conn
                .query_one(
                    &self.cached.pstat_series_insert,
                    &[&s.benchmark.as_str(), &s.profile, &s.scenario, &s.metric],
                )
                .await
                .unwrap();
            res.push(row.get(0));
        }
        res
    }
    async fn pstat(&self, series: i32, aid: i16, cid: i32, value: f64) {
        self.conn
            .execute(&self.cached.pstat_insert, &[&series, &aid, &cid, &value])
            .await
            .unwrap();
    }
    async fn self_profile_query_series(&self, series: &[SpqSeries]) -> Vec<i32> {
        let mut res = Vec::with_capacity(series.len());
        let mut exact = series.chunks_exact(30);
        for series in exact.by_ref() {
            let params = series
                .iter()
                .flat_map(|s| {
                    vec![
                        &*s.benchmark as &(dyn tokio_postgres::types::ToSql + Sync),
                        &s.profile as &(dyn tokio_postgres::types::ToSql + Sync),
                        &s.scenario as &(dyn tokio_postgres::types::ToSql + Sync),
                        &s.query as &(dyn tokio_postgres::types::ToSql + Sync),
                    ]
                })
                .collect::<Vec<_>>();
            let rows = self
                .conn
                .query(&self.cached.spq_series_insert_many, &params)
                .await
                .unwrap();
            for row in rows {
                res.push(row.get(0));
            }
        }
        for s in exact.remainder() {
            let row = self
                .conn
                .query_one(
                    &self.cached.spq_series_insert,
                    &[&s.benchmark.as_str(), &s.profile, &s.scenario, &s.query],
                )
                .await
                .unwrap();
            res.push(row.get(0));
        }
        res
    }
    async fn self_profile_query(&self, series: i32, aid: i16, cid: i32, qd: &QueryData) {
        self.conn
            .execute(
                &self.cached.spq_insert,
                &[
                    &series,
                    &aid,
                    &cid,
                    &i64::try_from(qd.self_time).unwrap(),
                    &i64::try_from(qd.blocked_time).unwrap(),
                    &i64::try_from(qd.incremental_load_time).unwrap(),
                    &(qd.number_of_cache_hits as i32),
                    &(qd.invocation_count as i32),
                ],
            )
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let db = std::env::args().nth(1).expect("database as first arg");
    let uploaded = std::env::args().nth(2).expect("uploaded");
    let uploaded = Path::new(&uploaded);
    let pool = Pool::open(&db);
    let mut sqlite = None::<rusqlite::Connection>;
    let mut postgres = None::<tokio_postgres::Client>;
    match pool {
        Pool::Sqlite(mut p) => {
            sqlite = Some(p.raw().open().await.into_inner().unwrap());
        }
        Pool::Postgres(mut p) => {
            postgres = Some(p.raw().open().await.into());
        }
    }

    if let Some(s) = &mut sqlite {
        s.pragma_update(None, "synchronous", &"OFF").unwrap();
        s.pragma_update(None, "cache_size", &-(2 << 23)).unwrap();
    }

    let cached = if let Some(p) = &mut postgres {
        Some(Arc::new(CachedStatements {
            pstat_series_insert: p.prepare(
                "insert into pstat_series(crate, profile, cache, statistic)
                VALUES ($1, $2, $3, $4)
                RETURNING id").await.unwrap(),
            pstat_series_insert_many: p.prepare(
                "insert into pstat_series(crate, profile, cache, statistic)
                VALUES ($1, $2, $3, $4),
                       ($5, $6, $7, $8),
                       ($9, $10, $11, $12),
                       ($13, $14, $15, $16),
                       ($17, $18, $19, $20),
                       ($21, $22, $23, $24),
                       ($25, $26, $27, $28),
                       ($29, $30, $31, $32),
                       ($33, $34, $35, $36),
                       ($37, $38, $39, $40),
                       ($41, $42, $43, $44)
                RETURNING id").await.unwrap(),
            pstat_insert: p.prepare(
                "insert into pstat (series, aid, cid, value) VALUES ($1, $2, $3, $4)",
            ).await.unwrap(),
            spq_series_insert: p.prepare(
                "insert into self_profile_query_series(crate, profile, cache, query)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT DO NOTHING
                RETURNING id").await.unwrap(),
            spq_series_insert_many: p.prepare(
                "insert into self_profile_query_series(crate, profile, cache, query)
                VALUES ($1, $2, $3, $4),
                        ($5, $6, $7, $8),
                        ($9, $10, $11, $12),
                        ($13, $14, $15, $16),
                        ($17, $18, $19, $20),
                        ($21, $22, $23, $24),
                        ($25, $26, $27, $28),
                        ($29, $30, $31, $32),
                        ($33, $34, $35, $36),
                        ($37, $38, $39, $40),
                        ($41, $42, $43, $44),
                        ($45, $46, $47, $48),
                        ($49, $50, $51, $52),
                        ($53, $54, $55, $56),
                        ($57, $58, $59, $60),
                        ($61, $62, $63, $64),
                        ($65, $66, $67, $68),
                        ($69, $70, $71, $72),
                        ($73, $74, $75, $76),
                        ($77, $78, $79, $80),
                        ($81, $82, $83, $84),
                        ($85, $86, $87, $88),
                        ($89, $90, $91, $92),
                        ($93, $94, $95, $96),
                        ($97, $98, $99, $100),
                        ($101, $102, $103, $104),
                        ($105, $106, $107, $108),
                        ($109, $110, $111, $112),
                        ($113, $114, $115, $116),
                        ($117, $118, $119, $120)
                RETURNING id").await.unwrap(),
            spq_insert: p.prepare(
                "insert into self_profile_query
                (series, aid, cid, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            ).await.unwrap(),
        }))
    } else {
        None
    };

    let paths = std::env::args().skip(3).collect::<Vec<_>>();
    let paths_count = paths.len();
    let mut s_cache = IdCache::default();
    if let Some(sqlite) = &mut sqlite {
        let mut rows = sqlite
            .prepare("select id, crate, profile, cache, statistic from pstat_series")
            .unwrap();
        let mut rows = rows.query(params![]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let profile: String = row.get(2).unwrap();
            let metric: String = row.get(4).unwrap();
            s_cache.pstat_series.insert(
                StatDescription {
                    benchmark: Arc::new(row.get(1).unwrap()),
                    profile: match profile.as_str() {
                        "check" => "check",
                        "opt" => "opt",
                        "debug" => "debug",
                        o => unimplemented!("{}", o),
                    },
                    scenario: row.get(3).unwrap(),
                    metric: match metric.as_str() {
                        "instructions:u" => "instructions:u",
                        "cycles:u" => "cycles:u",
                        "faults" => "faults",
                        "max-rss" => "max-rss",
                        "task-clock" => "task-clock",
                        "wall-time" => "wall-time",
                        "cpu-clock" => "cpu-clock",
                        o => unimplemented!("{}", o),
                    },
                },
                row.get(0).unwrap(),
            );
        }
        let mut rows = sqlite
            .prepare("select id, crate, profile, cache, query from self_profile_query_series")
            .unwrap();
        let mut rows = rows.query(params![]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let profile: String = row.get(2).unwrap();
            let query: String = row.get(4).unwrap();
            s_cache.spq_series.insert(
                SpqSeries {
                    benchmark: Arc::new(row.get(1).unwrap()),
                    profile: match profile.as_str() {
                        "check" => "check",
                        "opt" => "opt",
                        "debug" => "debug",
                        o => unimplemented!("{}", o),
                    },
                    scenario: row.get(3).unwrap(),
                    query: QueryLabel::from(query.as_str()).as_str(),
                },
                row.get(0).unwrap(),
            );
        }
    }

    let mut p_cache = IdCache::default();
    if let Some(postgres) = &mut postgres {
        let rows = postgres
            .copy_out(
                "copy pstat_series (id, crate, profile, cache, statistic) to stdout (FORMAT 'binary')",
            )
            .await
            .unwrap();
        let rows = tokio_postgres::binary_copy::BinaryCopyOutStream::new(
            rows,
            &[
                tokio_postgres::types::Type::INT4,
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
            ],
        );
        futures::pin_mut!(rows);
        while let Some(row) = rows.next().await {
            let row = row.unwrap();
            let profile: String = row.get(2);
            let metric: String = row.get(4);
            p_cache.pstat_series.insert(
                StatDescription {
                    benchmark: Arc::new(row.get(1)),
                    profile: match profile.as_str() {
                        "check" => "check",
                        "opt" => "opt",
                        "debug" => "debug",
                        o => unimplemented!("{}", o),
                    },
                    scenario: row.get(3),
                    metric: match metric.as_str() {
                        "instructions:u" => "instructions:u",
                        "cycles:u" => "cycles:u",
                        "faults" => "faults",
                        "max-rss" => "max-rss",
                        "task-clock" => "task-clock",
                        "wall-time" => "wall-time",
                        "cpu-clock" => "cpu-clock",
                        o => unimplemented!("{}", o),
                    },
                },
                row.get(0),
            );
        }
        let rows = postgres
            .copy_out(
                "copy self_profile_query_series (id, crate, profile, cache, query) to stdout (FORMAT 'binary')",
            )
            .await
            .unwrap();
        let rows = tokio_postgres::binary_copy::BinaryCopyOutStream::new(
            rows,
            &[
                tokio_postgres::types::Type::INT4,
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
                tokio_postgres::types::Type::TEXT,
            ],
        );
        futures::pin_mut!(rows);
        while let Some(row) = rows.next().await {
            let row = row.unwrap();
            let profile: String = row.get(2);
            let query: String = row.get(4);
            p_cache.spq_series.insert(
                SpqSeries {
                    benchmark: Arc::new(row.get(1)),
                    profile: match profile.as_str() {
                        "check" => "check",
                        "opt" => "opt",
                        "debug" => "debug",
                        o => unimplemented!("{}", o),
                    },
                    scenario: row.get(3),
                    query: QueryLabel::from(query.as_str()).as_str(),
                },
                row.get(0),
            );
        }
    }

    let mut last = std::time::Instant::now();
    for (idx, path) in paths.into_iter().enumerate() {
        if idx % 10 == 0 {
            eprintln!(
                "{}/{}, per {:?}; estimated time left {:?}",
                idx,
                paths_count,
                last.elapsed() / 10,
                last.elapsed() / 10 * (paths_count as u32 - idx as u32)
            );
            last = std::time::Instant::now();
        }
        let path = Path::new(&path);
        let sfut: std::pin::Pin<Box<dyn Future<Output = ()>>> = if let Some(s) = &mut sqlite {
            let mut s_cache = &mut s_cache;
            Box::pin(async move {
                let mut s = Sqlite {
                    conn: Mutex::new(s),
                };
                s.conn().execute_batch("BEGIN DEFERRED").unwrap();
                ingest(&mut s, &mut s_cache, path).await;
                s.conn().execute_batch("COMMIT").unwrap();
            })
        } else {
            Box::pin(async move {})
        };
        let cached = cached.clone();
        let pfut: std::pin::Pin<Box<dyn Future<Output = ()>>> = if let Some(p) = &mut postgres {
            let mut p_cache = &mut p_cache;
            Box::pin(async move {
                let mut p = Postgres {
                    conn: p.transaction().await.unwrap(),
                    cached: cached.unwrap(),
                };
                ingest(&mut p, &mut p_cache, path).await;
                p.conn.commit().await.unwrap();
            })
        } else {
            Box::pin(async move {})
        };
        futures::join!(sfut, pfut);
        std::fs::rename(path, uploaded.join(path.file_name().unwrap())).unwrap();
    }
}

#[derive(Default)]
struct IdCache {
    benchmarks: HashSet<Arc<String>>,
    pstat_series: HashMap<StatDescription, i32>,
    spq_series: HashMap<SpqSeries, i32>,
}

async fn ingest<T: Ingesting>(conn: &T, caches: &mut IdCache, path: &Path) {
    let res = deserialize_path(path);
    let (name, date, ty, benchmarks) = match res {
        Res::Commit(cd) => (
            cd.commit.sha.to_string(),
            Some(cd.commit.date.0),
            if cd.commit.is_try() { "try" } else { "master" },
            cd.benchmarks,
        ),
        Res::Artifact(ad) => (ad.id, None, "release", ad.benchmarks),
    };

    // All JSON files are implicitly assumed to be just one collection
    let (aid, cid) = futures::join!(conn.artifact(&name, date, ty), conn.collection());

    let mut buf = FuturesUnordered::<Pin<Box<dyn Future<Output = ()>>>>::new();
    let mut series = Vec::new();
    let mut values = Vec::new();
    let mut spq_series = Vec::new();
    let mut spq_values = Vec::new();
    for (name, bres) in benchmarks.iter() {
        let name = Arc::new(name.to_string());
        if caches.benchmarks.insert(name.clone()) {
            conn.benchmark(&name).await;
        }

        let benchmark = match bres {
            Ok(b) => b,
            Err(e) => {
                let name = name.clone();
                buf.push(Box::pin(async move {
                    let eid = conn.error_series(&name).await;
                    conn.error(eid, aid, e.as_str()).await;
                }));
                continue;
            }
        };

        for run in &benchmark.runs {
            let profile = if run.check {
                Profile::Check
            } else if run.release {
                Profile::Opt
            } else {
                Profile::Debug
            };
            let profile_str = match profile {
                Profile::Check => "check",
                Profile::Debug => "debug",
                Profile::Doc => "doc",
                Profile::Opt => "opt",
            };
            let state = match &run.state {
                BenchmarkState::Clean => Scenario::Empty,
                BenchmarkState::IncrementalStart => Scenario::IncrementalEmpty,
                BenchmarkState::IncrementalClean => Scenario::IncrementalFresh,
                BenchmarkState::IncrementalPatched(p) => Scenario::IncrementalPatch(p.name),
            };

            for (sid, stat) in run.stats.iter() {
                let name = name.clone();
                let key = StatDescription {
                    benchmark: name.clone(),
                    profile: profile_str,
                    scenario: state.to_string(),
                    metric: sid,
                };
                if let Some(&sid) = caches.pstat_series.get(&key) {
                    buf.push(conn.pstat(sid, aid, cid, stat));
                } else {
                    series.push(key);
                    values.push(stat);
                }
            }

            if let Some(self_profile) = &run.self_profile {
                for qd in self_profile.query_data.iter() {
                    let name = name.clone();
                    let key = SpqSeries {
                        benchmark: name.clone(),
                        profile: profile_str,
                        scenario: state.to_string(),
                        query: qd.label.as_str(),
                    };
                    if let Some(&sid) = caches.spq_series.get(&key) {
                        buf.push(conn.self_profile_query(sid, aid, cid, &qd));
                    } else {
                        spq_series.push(key);
                        spq_values.push(qd.clone());
                    }
                }
            }
        }
    }
    eprintln!(
        "finishing insert of {} pstat series and {} queries",
        series.len(),
        spq_series.len()
    );
    let sids = conn.pstat_series(&series).await;
    let mut series = series.into_iter();
    for (idx, sid) in sids.into_iter().enumerate() {
        caches.pstat_series.insert(series.next().unwrap(), sid);
        buf.push(conn.pstat(sid, aid, cid, values[idx]));
    }
    let sids = conn.self_profile_query_series(&spq_series).await;
    let mut series = spq_series.into_iter();
    for (idx, sid) in sids.into_iter().enumerate() {
        caches.spq_series.insert(series.next().unwrap(), sid);
        buf.push(conn.self_profile_query(sid, aid, cid, &spq_values[idx]));
    }
    while let Some(()) = buf.next().await {}
}

enum Res {
    Artifact(ArtifactData),
    Commit(CommitData),
}

fn deserialize_path(path: &Path) -> Res {
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("Failed to open {}", path.display()))
        .unwrap();
    let mut file_contents = Vec::new();
    if path.extension().map_or(false, |e| e == "sz") {
        let mut szip_reader = snap::read::FrameDecoder::new(std::io::BufReader::new(file));
        szip_reader
            .read_to_end(&mut file_contents)
            .with_context(|| format!("Failed to read {}", path.display()))
            .unwrap();
    } else {
        file.read_to_end(&mut file_contents)
            .with_context(|| format!("Failed to read {}", path.display()))
            .unwrap();
    };

    if path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with("artifact-")
    {
        Res::Artifact(serde_json::from_slice(&file_contents).unwrap())
    } else {
        Res::Commit(serde_json::from_slice(&file_contents).unwrap())
    }
}
