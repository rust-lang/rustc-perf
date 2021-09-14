//! Program to export a Postgres database to SQLite.
//!
//! This is intended to be used on an empty SQLite database with no ongoing
//! transactions, and will likely fail if used on a populated database.

use chrono::{DateTime, Utc};
use database::pool::{postgres, sqlite, ConnectionManager};
use futures::StreamExt;
use rusqlite::params;
use std::time::Instant;

trait Table {
    /// Table name.
    fn name() -> &'static str;

    /// Comma-separated list of table's attribute names in Postgres.
    fn postgres_attributes() -> &'static str;

    /// Parameterized statement to insert row into SQLite table.
    fn sqlite_insert_statement() -> &'static str;

    /// Extracts attribute values from Postgres row and inserts them into SQLite
    /// using the given statement and transaction. The statement should be one
    /// prepared from the output of `sqlite_insert_statement`.
    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row);
}

struct Artifact;

impl Table for Artifact {
    fn name() -> &'static str {
        "artifact"
    }

    fn postgres_attributes() -> &'static str {
        "id, name, date, type"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into artifact (id, name, date, type) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, Option<DateTime<Utc>>>(2)
                    .map(|d| d.timestamp()),
                row.get::<_, &str>(3)
            ])
            .unwrap();
    }
}

struct ArtifactCollectionDuration;

impl Table for ArtifactCollectionDuration {
    fn name() -> &'static str {
        "artifact_collection_duration"
    }

    fn postgres_attributes() -> &'static str {
        "aid, date_recorded, duration"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into artifact_collection_duration (aid, date_recorded, duration) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, DateTime<Utc>>(1).timestamp(),
                row.get::<_, i32>(2)
            ])
            .unwrap();
    }
}

struct Benchmark;

impl Table for Benchmark {
    fn name() -> &'static str {
        "benchmark"
    }

    fn postgres_attributes() -> &'static str {
        "name, stabilized"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into benchmark (name, stabilized) VALUES (?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, &str>(0),
                // This has a non-null constraint in SQLite schema, but not in Postgres.
                row.get::<_, bool>(1) as u8,
            ])
            .unwrap();
    }
}

struct Collection;

impl Table for Collection {
    fn name() -> &'static str {
        "collection"
    }

    fn postgres_attributes() -> &'static str {
        "id, perf_commit"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into collection (id, perf_commit) VALUES (?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![row.get::<_, i32>(0), row.get::<_, Option<&str>>(1)])
            .unwrap();
    }
}

struct CollectorProgress;

impl Table for CollectorProgress {
    fn name() -> &'static str {
        "collector_progress"
    }

    fn postgres_attributes() -> &'static str {
        "aid, step, start_time, end_time"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into collector_progress (aid, step, start, end) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, Option<DateTime<Utc>>>(2)
                    .map(|d| d.timestamp()),
                row.get::<_, Option<DateTime<Utc>>>(3)
                    .map(|d| d.timestamp()),
            ])
            .unwrap();
    }
}

struct Error;

impl Table for Error {
    fn name() -> &'static str {
        "error"
    }

    fn postgres_attributes() -> &'static str {
        "series, aid, error"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into error (series, aid, error) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i32>(1),
                row.get::<_, Option<&str>>(2),
            ])
            .unwrap();
    }
}

struct ErrorSeries;

impl Table for ErrorSeries {
    fn name() -> &'static str {
        "error_series"
    }

    fn postgres_attributes() -> &'static str {
        "id, crate"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into error_series (id, crate) VALUES (?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![row.get::<_, i32>(0), row.get::<_, &str>(1)])
            .unwrap();
    }
}

struct Pstat;

impl Table for Pstat {
    fn name() -> &'static str {
        "pstat"
    }

    fn postgres_attributes() -> &'static str {
        "series, aid, cid, value"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into pstat (series, aid, cid, value) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i32>(1),
                row.get::<_, i32>(2),
                row.get::<_, f64>(3),
            ])
            .unwrap();
    }
}

struct PstatSeries;

impl Table for PstatSeries {
    fn name() -> &'static str {
        "pstat_series"
    }

    fn postgres_attributes() -> &'static str {
        "id, crate, profile, cache, statistic"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into pstat_series (id, crate, profile, cache, statistic) VALUES (?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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

struct PullRequestBuild;

impl Table for PullRequestBuild {
    fn name() -> &'static str {
        "pull_request_build"
    }

    fn postgres_attributes() -> &'static str {
        "bors_sha, pr, parent_sha, complete, requested, include, exclude, runs"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into pull_request_build (bors_sha, pr, parent_sha, complete, requested, include, exclude, runs) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, Option<&str>>(0),
                row.get::<_, i32>(1),
                row.get::<_, Option<&str>>(2),
                row.get::<_, Option<bool>>(3),
                row.get::<_, Option<DateTime<Utc>>>(4)
                    .map(|d| d.timestamp()),
                row.get::<_, Option<&str>>(5),
                row.get::<_, Option<&str>>(6),
                row.get::<_, Option<i32>>(7),
            ])
            .unwrap();
    }
}

struct RawSelfProfile;

impl Table for RawSelfProfile {
    fn name() -> &'static str {
        "raw_self_profile"
    }

    fn postgres_attributes() -> &'static str {
        "aid, cid, crate, profile, cache"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into raw_self_profile (aid, cid, crate, profile, cache) VALUES (?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i32>(1),
                row.get::<_, &str>(2),
                row.get::<_, &str>(3),
                row.get::<_, &str>(4),
            ])
            .unwrap();
    }
}

struct RustcCompilation;

impl Table for RustcCompilation {
    fn name() -> &'static str {
        "rustc_compilation"
    }

    fn postgres_attributes() -> &'static str {
        "aid, cid, crate, duration"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into rustc_compilation (aid, cid, crate, duration) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i32>(1),
                row.get::<_, &str>(2),
                row.get::<_, i64>(3),
            ])
            .unwrap();
    }
}

struct SelfProfileQuery;

impl Table for SelfProfileQuery {
    fn name() -> &'static str {
        "self_profile_query"
    }

    fn postgres_attributes() -> &'static str {
        "series, aid, cid, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into self_profile_query (series, aid, cid, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, i32>(1),
                row.get::<_, i32>(2),
                row.get::<_, Option<i64>>(3),
                row.get::<_, Option<i64>>(4),
                row.get::<_, Option<i64>>(5),
                row.get::<_, Option<i32>>(6),
                row.get::<_, Option<i32>>(7),
            ])
            .unwrap();
    }
}

struct SelfProfileQuerySeries;

impl Table for SelfProfileQuerySeries {
    fn name() -> &'static str {
        "self_profile_query_series"
    }

    fn postgres_attributes() -> &'static str {
        "id, crate, profile, cache, query"
    }

    fn sqlite_insert_statement() -> &'static str {
        "insert into self_profile_query_series (id, crate, profile, cache, query) VALUES (?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut args = std::env::args();
    let executable = args.next().unwrap();

    if args.len() != 2 {
        eprintln!(
            "Usage: {0} <postgres-db> <sqlite-db>\n\
            E.g.: {0} postgres://user:password@localhost:5432 results.db",
            executable,
        );
        std::process::exit(1);
    }

    let postgres = args.next().unwrap();
    let sqlite = args.next().unwrap();

    let mut postgres: tokio_postgres::Client =
        postgres::Postgres::new(postgres.into()).open().await.into();

    let mut sqlite = sqlite::Sqlite::new(sqlite.into())
        .open()
        .await
        .into_inner()
        .unwrap();

    // Postgres repeatable-read transactions use a snapshot of the database, and
    // we do the entire export under a single transaction. This ensures we avoid
    // running into inconsistencies resulting from concurrent Postgres transactions.
    let postgres_tx = postgres
        .build_transaction()
        .isolation_level(tokio_postgres::IsolationLevel::RepeatableRead)
        .read_only(true)
        .start()
        .await?;

    // While this is intended to be used on a SQLite database with no ongoing
    // transactions, we still want to use a transaction in order to only commit
    // if everything succeeds. The transaction is not being used for isolation.
    let sqlite_tx = sqlite.transaction().unwrap();

    // Order matters to the extent necessary to satisfy foreign key constraints.
    copy::<Artifact>(&postgres_tx, &sqlite_tx).await;
    copy::<ArtifactCollectionDuration>(&postgres_tx, &sqlite_tx).await;
    copy::<Benchmark>(&postgres_tx, &sqlite_tx).await;
    copy::<Collection>(&postgres_tx, &sqlite_tx).await;
    copy::<CollectorProgress>(&postgres_tx, &sqlite_tx).await;
    copy::<ErrorSeries>(&postgres_tx, &sqlite_tx).await;
    copy::<Error>(&postgres_tx, &sqlite_tx).await;
    copy::<PstatSeries>(&postgres_tx, &sqlite_tx).await;
    copy::<Pstat>(&postgres_tx, &sqlite_tx).await;
    copy::<PullRequestBuild>(&postgres_tx, &sqlite_tx).await;
    copy::<RawSelfProfile>(&postgres_tx, &sqlite_tx).await;
    copy::<RustcCompilation>(&postgres_tx, &sqlite_tx).await;
    copy::<SelfProfileQuerySeries>(&postgres_tx, &sqlite_tx).await;
    copy::<SelfProfileQuery>(&postgres_tx, &sqlite_tx).await;

    // This is overly paranoid, but don't commit the SQLite transaction until
    // the rollback of the Postgres transaction succeeds.
    postgres_tx.rollback().await?;
    sqlite_tx.commit().unwrap();

    Ok(())
}

async fn copy<T: Table>(
    postgres: &tokio_postgres::Transaction<'_>,
    sqlite: &rusqlite::Transaction<'_>,
) {
    // There are SQL injection vulnerabilities below, but it seems extremely
    // unlikely that we will ever execute SQL built from external strings.
    let table = T::name();

    let select = postgres
        .prepare(&format!(
            "select {} from {}",
            T::postgres_attributes(),
            table
        ))
        .await
        .unwrap();

    let mut insert = sqlite.prepare(T::sqlite_insert_statement()).unwrap();

    let start = Instant::now();
    let rows = postgres
        .query_raw(&select, Vec::<u32>::new())
        .await
        .unwrap();
    let mut count = 0;

    tokio::pin!(rows);

    while let Some(result) = rows.next().await {
        let row = result.unwrap();
        T::sqlite_execute_insert(&mut insert, row);
        count += 1;
    }

    let elapsed = start.elapsed();

    eprintln!(
        "Copied {} rows from {} table in {:?} ({:.0} rows/second)",
        count,
        table,
        elapsed,
        count as f64 / elapsed.as_secs_f64()
    );
}
