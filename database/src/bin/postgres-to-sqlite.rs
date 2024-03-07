//! Program to export a Postgres database to SQLite.
//!
//! This is intended to be used on an empty SQLite database with no ongoing
//! transactions, and will likely fail if used on a populated database.

use chrono::{DateTime, Utc};
use clap::{builder::PossibleValuesParser, ArgAction};
use database::pool::{postgres, sqlite, ConnectionManager};
use futures_util::StreamExt;
use rusqlite::params;
use std::time::Instant;

const ARTIFACT_WHERE: &str = "where artifact.date > (CURRENT_TIMESTAMP - interval '{} week')";

const ARTIFACT_JOIN_AND_WHERE: &str = "join artifact on artifact.id = aid \
    where artifact.date > (CURRENT_TIMESTAMP - interval '{} week')";

fn with_filter_clause_maybe(
    mut statement: String,
    filter_clause: &str,
    since_weeks_ago: Option<u32>,
) -> String {
    if let Some(weeks) = since_weeks_ago {
        statement.push(' ');
        statement.push_str(&filter_clause.replace("{}", &weeks.to_string()));
    }

    statement
}

trait Table {
    /// Table name.
    fn name(&self) -> &'static str;

    // Statement to select rows from Postgres table.
    //
    // If `since_weeks_ago` is `Some(n)`, the select statement excludes any rows
    // associated with artifacts whose date value precedes `n` weeks ago.
    // Otherwise, it includes all rows.
    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String;

    /// Parameterized statement to insert row into SQLite table.
    fn sqlite_insert_statement(&self) -> &'static str;

    /// Extracts attribute values from Postgres row and inserts them into SQLite
    /// using the given statement and transaction. The statement should be one
    /// prepared from the output of `sqlite_insert_statement`.
    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row);
}

struct Artifact;

impl Table for Artifact {
    fn name(&self) -> &'static str {
        "artifact"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select id, name, date, type from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into artifact (id, name, date, type) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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
    fn name(&self) -> &'static str {
        "artifact_collection_duration"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select aid, date_recorded, duration from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into artifact_collection_duration (aid, date_recorded, duration) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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
    fn name(&self) -> &'static str {
        "benchmark"
    }

    fn postgres_select_statement(&self, _since_weeks_ago: Option<u32>) -> String {
        "select name, stabilized, category from ".to_string() + self.name()
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into benchmark (name, stabilized, category) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, &str>(0),
                // This has a non-null constraint in SQLite schema, but not in Postgres.
                row.get::<_, bool>(1) as u8,
                row.get::<_, &str>(2),
            ])
            .unwrap();
    }
}

struct Collection;

impl Table for Collection {
    fn name(&self) -> &'static str {
        "collection"
    }

    fn postgres_select_statement(&self, _since_weeks_ago: Option<u32>) -> String {
        "select id, perf_commit from ".to_string() + self.name()
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into collection (id, perf_commit) VALUES (?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![row.get::<_, i32>(0), row.get::<_, Option<&str>>(1)])
            .unwrap();
    }
}

struct CollectorProgress;

impl Table for CollectorProgress {
    fn name(&self) -> &'static str {
        "collector_progress"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select aid, step, start_time, end_time from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into collector_progress (aid, step, start, end) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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
    fn name(&self) -> &'static str {
        "error"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select benchmark, aid, error from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into error (benchmark, aid, error) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, &str>(0),
                row.get::<_, i32>(1),
                row.get::<_, Option<&str>>(2),
            ])
            .unwrap();
    }
}

struct Pstat;

impl Table for Pstat {
    fn name(&self) -> &'static str {
        "pstat"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select series, aid, cid, value from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into pstat (series, aid, cid, value) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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
    fn name(&self) -> &'static str {
        "pstat_series"
    }

    fn postgres_select_statement(&self, _since_weeks_ago: Option<u32>) -> String {
        "select id, crate, profile, scenario, backend, metric from ".to_string() + self.name()
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into pstat_series (id, crate, profile, scenario, backend, metric) VALUES (?, ?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, &str>(2),
                row.get::<_, &str>(3),
                row.get::<_, &str>(4),
                row.get::<_, &str>(5),
            ])
            .unwrap();
    }
}

struct PullRequestBuild;

impl Table for PullRequestBuild {
    fn name(&self) -> &'static str {
        "pull_request_build"
    }

    fn postgres_select_statement(&self, _since_weeks_ago: Option<u32>) -> String {
        "select bors_sha, pr, parent_sha, complete, requested, include, exclude, runs from "
            .to_string()
            + self.name()
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into pull_request_build (bors_sha, pr, parent_sha, complete, requested, include, exclude, runs) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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
    fn name(&self) -> &'static str {
        "raw_self_profile"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select aid, cid, crate, profile, cache from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into raw_self_profile (aid, cid, crate, profile, cache) VALUES (?, ?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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
    fn name(&self) -> &'static str {
        "rustc_compilation"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select aid, cid, crate, duration from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into rustc_compilation (aid, cid, crate, duration) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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

struct RuntimePstat;

impl Table for RuntimePstat {
    fn name(&self) -> &'static str {
        "runtime_pstat"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select series, aid, cid, value from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into runtime_pstat (series, aid, cid, value) VALUES (?, ?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
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

struct RuntimePstatSeries;

impl Table for RuntimePstatSeries {
    fn name(&self) -> &'static str {
        "runtime_pstat_series"
    }

    fn postgres_select_statement(&self, _since_weeks_ago: Option<u32>) -> String {
        "select id, benchmark, metric from ".to_string() + self.name()
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into runtime_pstat_series (id, benchmark, metric) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, &str>(2),
            ])
            .unwrap();
    }
}

struct ArtifactSize;

impl Table for ArtifactSize {
    fn name(&self) -> &'static str {
        "artifact_size"
    }

    fn postgres_select_statement(&self, since_weeks_ago: Option<u32>) -> String {
        let s = "select aid, component, size from ".to_string() + self.name();
        with_filter_clause_maybe(s, ARTIFACT_JOIN_AND_WHERE, since_weeks_ago)
    }

    fn sqlite_insert_statement(&self) -> &'static str {
        "insert into artifact_size (aid, component, size) VALUES (?, ?, ?)"
    }

    fn sqlite_execute_insert(&self, statement: &mut rusqlite::Statement, row: tokio_postgres::Row) {
        statement
            .execute(params![
                row.get::<_, i32>(0),
                row.get::<_, &str>(1),
                row.get::<_, i32>(2),
            ])
            .unwrap();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Order matters to the extent necessary to satisfy foreign key constraints.
    let tables: &[&dyn Table] = &[
        &Artifact,
        &ArtifactCollectionDuration,
        &Benchmark,
        &Collection,
        &CollectorProgress,
        &Error,
        &PstatSeries,
        &Pstat,
        &PullRequestBuild,
        &RawSelfProfile,
        &RustcCompilation,
        &RuntimePstatSeries,
        &RuntimePstat,
        &ArtifactSize,
    ];

    let table_names: Vec<_> = tables.iter().map(|table| table.name()).collect();

    let matches = clap::Command::new("postgres-to-sqlite")
        .about("Exports a rustc-perf Postgres database to a SQLite database")
        .version(clap::crate_version!())
        .arg(
            clap::Arg::new("exclude-tables")
                .action(ArgAction::Set)
                .long("exclude-tables")
                .value_name("TABLES")
                .value_parser(PossibleValuesParser::new(table_names))
                .value_delimiter(',')
                .help("Exclude given tables (as foreign key constraints allow)"),
        )
        .arg(
            clap::Arg::new("no-self-profile")
                .action(ArgAction::SetTrue)
                .long("no-self-profile")
                .help("Exclude some potentially large self-profile tables (additive with --exclude-tables)"),
        )
        .arg(
            clap::Arg::new("since-weeks-ago")
                .action(ArgAction::Set)
                .long("since-weeks-ago")
                .value_name("WEEKS")
                .value_parser(clap::value_parser!(u32))
                .help("Exclude data associated with artifacts whose date value precedes <WEEKS> weeks ago"),
        )
        .arg(
            clap::Arg::new("fast-unsafe")
                .action(ArgAction::SetTrue)
                .long("fast-unsafe")
                .help("Enable faster execution at the risk of corrupting SQLite database in the event of a crash"),
        )
        .arg(
            clap::Arg::new("postgres-db")
                .action(ArgAction::Set)
                .required(true)
                .value_name("POSTGRES_DB")
                .help(
                    "Postgres database connection string, \
                        e.g. postgres://user:password@localhost:5432",
                ),
        )
        .arg(
            clap::Arg::new("sqlite-db")
                .action(ArgAction::Set)
                .required(true)
                .value_name("SQLITE_DB")
                .help("SQLite database file"),
        )
        .get_matches();

    let postgres = matches.get_one::<String>("postgres-db").unwrap();
    let sqlite = matches.get_one::<String>("sqlite-db").unwrap();

    let exclude_tables: std::collections::HashSet<_> = matches
        .get_many::<String>("exclude-tables")
        .unwrap_or_default()
        .cloned()
        .collect();

    let since_weeks_ago = matches.get_one::<u32>("since-weeks-ago").copied();

    let mut postgres: tokio_postgres::Client =
        postgres::Postgres::new(postgres.into()).open().await.into();

    let mut sqlite = sqlite::Sqlite::new(sqlite.into())
        .open()
        .await
        .into_inner()
        .unwrap();

    if matches.get_flag("fast-unsafe") {
        sqlite.pragma_update(None, "journal_mode", "OFF").unwrap();
        sqlite.pragma_update(None, "synchronous", "OFF").unwrap();
    }

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

    for &table in tables {
        if !exclude_tables.contains(table.name()) {
            copy(table, &postgres_tx, &sqlite_tx, since_weeks_ago).await;
        }
    }

    // This is overly paranoid, but don't commit the SQLite transaction until
    // the rollback of the Postgres transaction succeeds.
    postgres_tx.rollback().await?;
    sqlite_tx.commit().unwrap();

    Ok(())
}

async fn copy<T: Table + ?Sized>(
    table: &T,
    postgres: &tokio_postgres::Transaction<'_>,
    sqlite: &rusqlite::Transaction<'_>,
    since_weeks_ago: Option<u32>,
) {
    // There are SQL injection vulnerabilities below, but it seems extremely
    // unlikely that we will ever execute SQL built from external strings.
    let select = postgres
        .prepare(&table.postgres_select_statement(since_weeks_ago))
        .await
        .unwrap();

    let mut insert = sqlite.prepare(table.sqlite_insert_statement()).unwrap();

    let start = Instant::now();
    let rows = postgres
        .query_raw(&select, Vec::<u32>::new())
        .await
        .unwrap();
    let mut count = 0;

    tokio::pin!(rows);

    while let Some(result) = rows.next().await {
        let row = result.unwrap();
        table.sqlite_execute_insert(&mut insert, row);
        count += 1;
    }

    let elapsed = start.elapsed();

    eprintln!(
        "Copied {} rows from {} table in {:?} ({:.0} rows/second)",
        count,
        table.name(),
        elapsed,
        count as f64 / elapsed.as_secs_f64()
    );
}
