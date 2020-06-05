use database::{pool::ConnectionManager, Pool};
use futures::StreamExt;
use rusqlite::params;
use std::time::Instant;
use tokio_postgres::types::Type;
use tokio_postgres::Row;

trait Table {
    fn name() -> &'static str;
    fn copy_out() -> &'static str;
    fn insert() -> &'static str;
    fn types() -> &'static [Type];
    fn execute(statement: &mut rusqlite::Statement, row: Row);
}

struct PstatSeries;
impl Table for PstatSeries {
    fn name() -> &'static str {
        "pstat_series"
    }
    fn copy_out() -> &'static str {
        "id, crate, profile, cache, statistic"
    }
    fn insert() -> &'static str {
        "insert into pstat_series (id, crate, profile, cache, statistic) VALUES (?, ?, ?, ?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT4, Type::TEXT, Type::TEXT, Type::TEXT, Type::TEXT]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, &str>(2),
                row.get::<_, &str>(3),
                row.get::<_, &str>(4),
            ])
            .unwrap();
    }
}

struct Pstat;
impl Table for Pstat {
    fn name() -> &'static str {
        "pstat"
    }
    fn copy_out() -> &'static str {
        "series, aid, cid, value"
    }
    fn insert() -> &'static str {
        "insert into pstat (series, aid, cid, value) VALUES (?, ?, ?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT4, Type::INT2, Type::INT4, Type::FLOAT8]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i16>(1),
                row.get::<_, i32>(2),
                row.get::<_, f64>(3),
            ])
            .unwrap();
    }
}

struct ErrorSeries;
impl Table for ErrorSeries {
    fn name() -> &'static str {
        "error_series"
    }
    fn copy_out() -> &'static str {
        "id, crate"
    }
    fn insert() -> &'static str {
        "insert into error_series (id, crate) VALUES (?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT4, Type::TEXT]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![row.get::<_, i32>(0), row.get::<_, &str>(1),])
            .unwrap();
    }
}

struct Error;
impl Table for Error {
    fn name() -> &'static str {
        "error"
    }
    fn copy_out() -> &'static str {
        "series, aid, error"
    }
    fn insert() -> &'static str {
        "insert into error (series, aid, error) VALUES (?, ?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT4, Type::INT2, Type::TEXT]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i16>(1),
                row.get::<_, &str>(2),
            ])
            .unwrap();
    }
}

struct Benchmark;
impl Table for Benchmark {
    fn name() -> &'static str {
        "benchmark"
    }
    fn copy_out() -> &'static str {
        "name, stabilized"
    }
    fn insert() -> &'static str {
        "insert into benchmark (name, stabilized) VALUES (?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::TEXT, Type::BOOL]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![row.get::<_, &str>(0), row.get::<_, bool>(1) as u8,])
            .unwrap();
    }
}

struct Artifact;
impl Table for Artifact {
    fn name() -> &'static str {
        "artifact"
    }
    fn copy_out() -> &'static str {
        "id, name, date, type"
    }
    fn insert() -> &'static str {
        "insert into artifact (id, name, date, type) VALUES (?, ?, ?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT2, Type::TEXT, Type::TIMESTAMPTZ, Type::TEXT]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![
                row.get::<_, i16>(0),
                row.get::<_, &str>(1),
                row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(2)
                    .map(|d| d.timestamp()),
                row.get::<_, &str>(3)
            ])
            .unwrap();
    }
}

struct Collection;
impl Table for Collection {
    fn name() -> &'static str {
        "collection"
    }
    fn copy_out() -> &'static str {
        "id"
    }
    fn insert() -> &'static str {
        "insert into collection (id) VALUES (?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT4]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement.execute(params![row.get::<_, i32>(0),]).unwrap();
    }
}

struct SelfProfileQuerySeries;
impl Table for SelfProfileQuerySeries {
    fn name() -> &'static str {
        "self_profile_query_series"
    }
    fn copy_out() -> &'static str {
        "id, crate, profile, cache, query"
    }
    fn insert() -> &'static str {
        "insert into self_profile_query_series (id, crate, profile, cache, query) VALUES (?, ?, ?, ?, ?)"
    }
    fn types() -> &'static [Type] {
        &[Type::INT4, Type::TEXT, Type::TEXT, Type::TEXT, Type::TEXT]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, &str>(2),
                row.get::<_, &str>(3),
                row.get::<_, &str>(4),
            ])
            .unwrap();
    }
}

struct SelfProfileQuery;
impl Table for SelfProfileQuery {
    fn name() -> &'static str {
        "self_profile_query"
    }
    fn copy_out() -> &'static str {
        "series, aid, cid, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count"
    }
    fn insert() -> &'static str {
        "insert into self_profile_query (series, aid, cid, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    }
    fn types() -> &'static [Type] {
        &[
            Type::INT4,
            Type::INT2,
            Type::INT4,
            Type::INT8,
            Type::INT8,
            Type::INT8,
            Type::INT4,
            Type::INT4,
        ]
    }
    fn execute(statement: &mut rusqlite::Statement, row: Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i16>(1),
                row.get::<_, i32>(2),
                row.get::<_, i64>(3),
                row.get::<_, i64>(4),
                row.get::<_, i64>(5),
                row.get::<_, i32>(6),
                row.get::<_, i32>(7),
            ])
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let postgres = std::env::args().nth(1).expect("postgres database");
    let sqlite = std::env::args().nth(2).expect("sqlite database");
    let mut postgres: tokio_postgres::Client = match Pool::open(&postgres) {
        Pool::Postgres(mut p) => p.raw().open().await.into(),
        _ => panic!("first argument must be postgres db"),
    };
    let mut sqlite = match Pool::open(&sqlite) {
        Pool::Sqlite(mut p) => p.raw().open().await.into_inner().unwrap(),
        _ => panic!("second argument must be sqlite db"),
    };
    sqlite
        .pragma_update(None, "cache_size", &-(1 << 17))
        .unwrap();
    sqlite.pragma_update(None, "journal_mode", &"OFF").unwrap();
    sqlite.pragma_update(None, "synchronous", &"OFF").unwrap();

    sqlite.execute_batch("BEGIN DEFERRED").unwrap();
    let mut tx = postgres.transaction().await.unwrap();
    copy::<Benchmark>(&mut tx, &mut sqlite).await;
    copy::<Artifact>(&mut tx, &mut sqlite).await;
    copy::<Collection>(&mut tx, &mut sqlite).await;
    copy::<SelfProfileQuerySeries>(&mut tx, &mut sqlite).await;
    copy::<ErrorSeries>(&mut tx, &mut sqlite).await;
    copy::<PstatSeries>(&mut tx, &mut sqlite).await;

    copy::<Error>(&mut tx, &mut sqlite).await;
    copy::<Pstat>(&mut tx, &mut sqlite).await;
    copy::<SelfProfileQuery>(&mut tx, &mut sqlite).await;
    sqlite.execute_batch("COMMIT").unwrap();
    tx.rollback().await.unwrap();
}

async fn copy<R: Table>(
    postgres: &mut tokio_postgres::Transaction<'_>,
    sqlite: &mut rusqlite::Connection,
) {
    let mut prepared = sqlite.prepare(R::insert()).unwrap();
    let rows = postgres
        .query_raw(
            format!("select {} from {}", R::copy_out(), R::name()).as_str(),
            vec![],
        )
        .await
        .unwrap();
    futures::pin_mut!(rows);
    let mut count = 0;
    let start = Instant::now();
    while let Some(row) = rows.next().await {
        let row = row.unwrap();
        R::execute(&mut prepared, row);
        count += 1;
    }
    let elapsed = start.elapsed();
    eprintln!(
        "{} exported {} rows in {:?} ({:.0} rows/second)",
        R::name(),
        count,
        elapsed,
        count as f64 / elapsed.as_secs_f64()
    );
}
