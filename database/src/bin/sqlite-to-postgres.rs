//! Program to export a SQLite database to Postgres.
//!
//! This is intended to be used on an empty Postgres database with no ongoing
//! transactions, and will likely fail if used on a populated database.

use bytes::{BufMut, Bytes, BytesMut};
use chrono::{DateTime, TimeZone, Utc};
use database::pool::{postgres, sqlite, ConnectionManager};
use futures_util::sink::SinkExt;
use hashbrown::HashMap;
use serde::{Serialize, Serializer};
use std::io::Write;
use std::time::Instant;

const NULL_STRING: &str = "\\N";

trait Table {
    /// Table name.
    fn name() -> &'static str;

    /// Comma-separated list of table's attribute names in SQLite.
    fn sqlite_attributes() -> &'static str;

    /// Comma-separated list of table's attribute names in Postgres.
    fn postgres_attributes() -> &'static str;

    /// Name of `generated always as identity` attribute in Postgres,
    /// if applicable.
    fn postgres_generated_id_attribute() -> Option<&'static str>;

    /// Extracts attribute values from SQLite row, converts them to match schema
    /// of Postgres table, and writes them to the CSV writer as a CSV record.
    ///
    /// Note to implementors: when extracting a value from the SQLite row, if
    /// the attribute has a narrower type in the Postgres schema than in the
    /// SQLite schema, it's advisable to extract it from the row as the narrower
    /// type, since this will give an error if the value will not fit. Note that
    /// SQLite integer attributes, regardless of declared type (e.g. tinyint),
    /// may be up to 64 bits in width.
    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row);
}

struct Artifact;

#[derive(Serialize)]
struct ArtifactRow<'a> {
    id: i32,
    name: &'a str,
    date: Nullable<DateTime<Utc>>,
    typ: &'a str,
}

impl Table for Artifact {
    fn name() -> &'static str {
        "artifact"
    }

    fn sqlite_attributes() -> &'static str {
        "id, name, date, type"
    }

    fn postgres_attributes() -> &'static str {
        "id, name, date, type"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        Some("id")
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        let date: Option<i64> = row.get(2).unwrap();

        writer
            .serialize(ArtifactRow {
                id: row.get(0).unwrap(),
                name: row.get_ref(1).unwrap().as_str().unwrap(),
                date: Nullable(date.map(|seconds| Utc.timestamp_opt(seconds, 0).unwrap())),
                typ: row.get_ref(3).unwrap().as_str().unwrap(),
            })
            .unwrap();
    }
}

struct ArtifactCollectionDuration;

#[derive(Serialize)]
struct ArtifactCollectionDurationRow {
    aid: i32,
    date_recorded: DateTime<Utc>,
    duration: i32,
}

impl Table for ArtifactCollectionDuration {
    fn name() -> &'static str {
        "artifact_collection_duration"
    }

    fn sqlite_attributes() -> &'static str {
        "aid, date_recorded, duration"
    }

    fn postgres_attributes() -> &'static str {
        "aid, date_recorded, duration"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        let date_recorded: i64 = row.get(1).unwrap();

        writer
            .serialize(ArtifactCollectionDurationRow {
                aid: row.get(0).unwrap(),
                date_recorded: Utc.timestamp_opt(date_recorded, 0).unwrap(),
                duration: row.get(2).unwrap(),
            })
            .unwrap();
    }
}

struct Benchmark;

#[derive(Serialize)]
struct BenchmarkRow<'a> {
    name: &'a str,
    // This has a non-null constraint in SQLite schema, but not in Postgres.
    stabilized: Nullable<bool>,
    category: &'a str,
}

impl Table for Benchmark {
    fn name() -> &'static str {
        "benchmark"
    }

    fn sqlite_attributes() -> &'static str {
        "name, stabilized, category"
    }

    fn postgres_attributes() -> &'static str {
        "name, stabilized, category"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(BenchmarkRow {
                name: row.get_ref(0).unwrap().as_str().unwrap(),
                stabilized: row.get(1).unwrap(),
                category: row.get_ref(2).unwrap().as_str().unwrap(),
            })
            .unwrap();
    }
}

struct Collection;

#[derive(Serialize)]
struct CollectionRow<'a> {
    id: i32,
    perf_commit: Nullable<&'a str>,
}

impl Table for Collection {
    fn name() -> &'static str {
        "collection"
    }

    fn sqlite_attributes() -> &'static str {
        "id, perf_commit"
    }

    fn postgres_attributes() -> &'static str {
        "id, perf_commit"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        Some("id")
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(CollectionRow {
                id: row.get(0).unwrap(),
                perf_commit: row.get_ref(1).unwrap().try_into().unwrap(),
            })
            .unwrap();
    }
}

struct CollectorProgress;

#[derive(Serialize)]
struct CollectorProgressRow<'a> {
    aid: i32,
    step: &'a str,
    start_time: Nullable<DateTime<Utc>>,
    end_time: Nullable<DateTime<Utc>>,
}

impl Table for CollectorProgress {
    fn name() -> &'static str {
        "collector_progress"
    }

    fn sqlite_attributes() -> &'static str {
        "aid, step, start, end"
    }

    fn postgres_attributes() -> &'static str {
        "aid, step, start_time, end_time"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        let start: Option<i64> = row.get(2).unwrap();
        let end: Option<i64> = row.get(3).unwrap();
        let start_time = Nullable(start.map(|seconds| Utc.timestamp_opt(seconds, 0).unwrap()));
        let end_time = Nullable(end.map(|seconds| Utc.timestamp_opt(seconds, 0).unwrap()));

        writer
            .serialize(CollectorProgressRow {
                aid: row.get(0).unwrap(),
                step: row.get_ref(1).unwrap().as_str().unwrap(),
                start_time,
                end_time,
            })
            .unwrap();
    }
}

struct Error;

#[derive(Serialize)]
struct ErrorRow<'a> {
    aid: i32,
    benchmark: &'a str,
    error: Nullable<&'a str>,
}

impl Table for Error {
    fn name() -> &'static str {
        "error"
    }

    fn sqlite_attributes() -> &'static str {
        "aid, benchmark, error"
    }

    fn postgres_attributes() -> &'static str {
        "aid, benchmark, error"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(ErrorRow {
                aid: row.get(0).unwrap(),
                benchmark: row.get_ref(1).unwrap().as_str().unwrap(),
                error: row.get_ref(2).unwrap().try_into().unwrap(),
            })
            .unwrap();
    }
}

struct Pstat;

#[derive(Serialize)]
struct PstatRow {
    series: i32,
    aid: i32,
    cid: i32,
    value: f64,
}

impl Table for Pstat {
    fn name() -> &'static str {
        "pstat"
    }

    fn sqlite_attributes() -> &'static str {
        "series, aid, cid, value"
    }

    fn postgres_attributes() -> &'static str {
        "series, aid, cid, value"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(PstatRow {
                series: row.get(0).unwrap(),
                aid: row.get(1).unwrap(),
                cid: row.get(2).unwrap(),
                value: row.get(3).unwrap(),
            })
            .unwrap();
    }
}

struct PstatSeries;

#[derive(Serialize)]
struct PstatSeriesRow<'a> {
    id: i32,
    krate: &'a str,
    profile: &'a str,
    scenario: &'a str,
    backend: &'a str,
    metric: &'a str,
}

impl Table for PstatSeries {
    fn name() -> &'static str {
        "pstat_series"
    }

    fn sqlite_attributes() -> &'static str {
        "id, crate, profile, scenario, backend, metric"
    }

    fn postgres_attributes() -> &'static str {
        "id, crate, profile, scenario, backend, metric"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        Some("id")
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(PstatSeriesRow {
                id: row.get(0).unwrap(),
                krate: row.get_ref(1).unwrap().as_str().unwrap(),
                profile: row.get_ref(2).unwrap().as_str().unwrap(),
                scenario: row.get_ref(3).unwrap().as_str().unwrap(),
                backend: row.get_ref(4).unwrap().as_str().unwrap(),
                metric: row.get_ref(5).unwrap().as_str().unwrap(),
            })
            .unwrap();
    }
}

struct PullRequestBuild;

#[derive(Serialize)]
struct PullRequestBuildRow<'a> {
    bors_sha: Nullable<&'a str>,
    pr: i32,
    parent_sha: Nullable<&'a str>,
    complete: Nullable<bool>,
    requested: Nullable<DateTime<Utc>>,
    include: Nullable<&'a str>,
    exclude: Nullable<&'a str>,
    runs: Nullable<i32>,
    commit_date: Nullable<DateTime<Utc>>,
}

impl Table for PullRequestBuild {
    fn name() -> &'static str {
        "pull_request_build"
    }

    fn sqlite_attributes() -> &'static str {
        "bors_sha, pr, parent_sha, complete, requested, include, exclude, runs, commit_date"
    }

    fn postgres_attributes() -> &'static str {
        "bors_sha, pr, parent_sha, complete, requested, include, exclude, runs, commit_date"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        let requested: Option<i64> = row.get(4).unwrap();
        let commit_date: Option<i64> = row.get(8).unwrap();

        writer
            .serialize(PullRequestBuildRow {
                bors_sha: row.get_ref(0).unwrap().try_into().unwrap(),
                pr: row.get(1).unwrap(),
                parent_sha: row.get_ref(2).unwrap().try_into().unwrap(),
                complete: row.get(3).unwrap(),
                requested: Nullable(
                    requested.map(|seconds| Utc.timestamp_opt(seconds, 0).unwrap()),
                ),
                include: row.get_ref(5).unwrap().try_into().unwrap(),
                exclude: row.get_ref(6).unwrap().try_into().unwrap(),
                runs: row.get(7).unwrap(),
                commit_date: Nullable(
                    commit_date.map(|seconds| Utc.timestamp_opt(seconds, 0).unwrap()),
                ),
            })
            .unwrap();
    }
}

struct RawSelfProfile;

#[derive(Serialize)]
struct RawSelfProfileRow<'a> {
    aid: i32,
    cid: i32,
    krate: &'a str,
    profile: &'a str,
    cache: &'a str,
}

impl Table for RawSelfProfile {
    fn name() -> &'static str {
        "raw_self_profile"
    }

    fn sqlite_attributes() -> &'static str {
        "aid, cid, crate, profile, cache"
    }

    fn postgres_attributes() -> &'static str {
        "aid, cid, crate, profile, cache"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(RawSelfProfileRow {
                aid: row.get(0).unwrap(),
                cid: row.get(1).unwrap(),
                krate: row.get_ref(2).unwrap().as_str().unwrap(),
                profile: row.get_ref(3).unwrap().as_str().unwrap(),
                cache: row.get_ref(4).unwrap().as_str().unwrap(),
            })
            .unwrap();
    }
}

struct RustcCompilation;

#[derive(Serialize)]
struct RustcCompilationRow<'a> {
    aid: i32,
    cid: i32,
    krate: &'a str,
    duration: i64,
}

impl Table for RustcCompilation {
    fn name() -> &'static str {
        "rustc_compilation"
    }

    fn sqlite_attributes() -> &'static str {
        "aid, cid, crate, duration"
    }

    fn postgres_attributes() -> &'static str {
        "aid, cid, crate, duration"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(RustcCompilationRow {
                aid: row.get(0).unwrap(),
                cid: row.get(1).unwrap(),
                krate: row.get_ref(2).unwrap().as_str().unwrap(),
                duration: row.get(3).unwrap(),
            })
            .unwrap();
    }
}

struct RuntimePstat;

#[derive(Serialize)]
struct RuntimePstatRow {
    series: i32,
    aid: i32,
    cid: i32,
    value: f64,
}

impl Table for RuntimePstat {
    fn name() -> &'static str {
        "runtime_pstat"
    }

    fn sqlite_attributes() -> &'static str {
        "series, aid, cid, value"
    }

    fn postgres_attributes() -> &'static str {
        "series, aid, cid, value"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(RuntimePstatRow {
                series: row.get(0).unwrap(),
                aid: row.get(1).unwrap(),
                cid: row.get(2).unwrap(),
                value: row.get(3).unwrap(),
            })
            .unwrap();
    }
}

struct RuntimePstatSeries;

#[derive(Serialize)]
struct RuntimePstatSeriesRow<'a> {
    id: i32,
    benchmark: &'a str,
    metric: &'a str,
}

impl Table for RuntimePstatSeries {
    fn name() -> &'static str {
        "runtime_pstat_series"
    }

    fn sqlite_attributes() -> &'static str {
        "id, benchmark, metric"
    }

    fn postgres_attributes() -> &'static str {
        "id, benchmark, metric"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        Some("id")
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(RuntimePstatSeriesRow {
                id: row.get(0).unwrap(),
                benchmark: row.get_ref(1).unwrap().as_str().unwrap(),
                metric: row.get_ref(2).unwrap().as_str().unwrap(),
            })
            .unwrap();
    }
}

struct ArtifactSize;

#[derive(Serialize)]
struct ArtifactSizeRow<'a> {
    aid: i32,
    component: &'a str,
    size: i32,
}

impl Table for ArtifactSize {
    fn name() -> &'static str {
        "artifact_size"
    }

    fn sqlite_attributes() -> &'static str {
        "aid, component, size"
    }

    fn postgres_attributes() -> &'static str {
        "aid, component, size"
    }

    fn postgres_generated_id_attribute() -> Option<&'static str> {
        None
    }

    fn write_postgres_csv_row<W: Write>(writer: &mut csv::Writer<W>, row: &rusqlite::Row) {
        writer
            .serialize(ArtifactSizeRow {
                aid: row.get(0).unwrap(),
                component: row.get_ref(1).unwrap().as_str().unwrap(),
                size: row.get(2).unwrap(),
            })
            .unwrap();
    }
}

// `Nullable<T>` helps to work around the fact that the `csv` crate (and the CSV
// format in general) doesn't distinguish between nulls and empty strings, while
// the Postgres CSV format does.
//
// By default, the Postgres CSV format uses a pair of double quotes to represent
// an empty string, and an actual empty string to represent null. However, the
// `csv` crate serializes both `Option<&str>::None` and `Option<&str>::Some("")`
// as an empty string. There are workarounds to enable writing them in the
// format Postgres expects, but they are much more involved than the alternative
// used here, which follows.
//
// If we tell Postgres to use a different string to represent nulls in the
// `COPY` command, then it will treat an actual empty string as an empty string.
// Then we can distinguish between nulls and empty strings in the CSV by writing
// our null string for `Option<&str>::None` and the empty string for
// `Option<&str>::Some("")`. The `Serialize` implementation for `Nullable`
// serializes its contained `Option` this way.
//
// The first downside of this approach is that the null string we choose might
// be present in our data, in which case it will be treated as null by Postgres.
//
// The second downside is that it may increase the size of the CSV, as every
// null needs to be represented by a non-empty string. However, empty strings no
// longer need to be represented by a pair of double quotes, so it's a tradeoff.
//
// We use the string "\N" to represent nulls, which is unlikely to be present in
// our data, while still being short enough to not bloat the CSV too much.
struct Nullable<T>(Option<T>);

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(ref t) => t.serialize(serializer),
            None => NULL_STRING.serialize(serializer),
        }
    }
}

// Enables getting a `Nullable<T>` from `rusqlite::Row::get`.
impl<T: rusqlite::types::FromSql> rusqlite::types::FromSql for Nullable<T> {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(Nullable(rusqlite::types::FromSql::column_result(value)?))
    }
}

// Enables getting a `Nullable<&str>` from `rusqlite::types::ValueRef::try_into`.
impl<'a> TryFrom<rusqlite::types::ValueRef<'a>> for Nullable<&'a str> {
    type Error = rusqlite::types::FromSqlError;

    fn try_from(value: rusqlite::types::ValueRef<'a>) -> Result<Self, Self::Error> {
        use rusqlite::types::ValueRef;

        match value {
            ValueRef::Null => Ok(Nullable(None)),
            ValueRef::Text(_) => Ok(Nullable(Some(value.as_str()?))),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = clap::Command::new("sqlite-to-postgres")
        .about("Exports a rustc-perf SQLite database to a Postgres database")
        .version(clap::crate_version!())
        .arg(
            clap::Arg::new("sqlite-db")
                .required(true)
                .value_name("SQLITE_DB")
                .help("SQLite database file"),
        )
        .arg(
            clap::Arg::new("postgres-db")
                .required(true)
                .value_name("POSTGRES_DB")
                .help(
                    "Postgres database connection string, \
                        e.g. postgres://user:password@localhost:5432",
                ),
        )
        .get_matches();

    let postgres = matches.get_one::<String>("postgres-db").unwrap();
    let sqlite = matches.get_one::<String>("sqlite-db").unwrap();

    let mut sqlite = sqlite::Sqlite::new(sqlite.into())
        .open()
        .await
        .into_inner()
        .unwrap();

    let mut postgres: tokio_postgres::Client =
        postgres::Postgres::new(postgres.into()).open().await.into();

    // SQLite read transactions use a snapshot of the database, and we do the
    // entire export under a single transaction. This ensures we avoid running
    // into inconsistencies resulting from concurrent SQLite transactions.
    let sqlite_tx = sqlite.transaction().unwrap();

    // While this is intended to be used on a Postgres database with no ongoing
    // transactions, we still want to use a transaction in order to only commit
    // if everything succeeds. The transaction is not being used for isolation.
    let postgres_tx = postgres.transaction().await?;

    // Disabling table triggers before copying data and then re-enabling them afterwards can
    // yield a significant speedup.
    // Note: Disabling or enabling internally generated constraint triggers requires superuser
    // privileges. [See PostgreSQL documentation](
    // https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-DESC-DISABLE-ENABLE-TRIGGER)
    let tables = get_tables(&postgres_tx).await;
    disable_table_triggers(&postgres_tx, &tables).await;
    // Order matters to the extent necessary to satisfy foreign key constraints.
    copy::<Artifact>(&sqlite_tx, &postgres_tx).await;
    copy::<ArtifactCollectionDuration>(&sqlite_tx, &postgres_tx).await;
    copy::<Benchmark>(&sqlite_tx, &postgres_tx).await;
    copy::<Collection>(&sqlite_tx, &postgres_tx).await;
    copy::<CollectorProgress>(&sqlite_tx, &postgres_tx).await;
    copy::<Error>(&sqlite_tx, &postgres_tx).await;
    copy::<PstatSeries>(&sqlite_tx, &postgres_tx).await;
    copy::<Pstat>(&sqlite_tx, &postgres_tx).await;
    copy::<PullRequestBuild>(&sqlite_tx, &postgres_tx).await;
    copy::<RawSelfProfile>(&sqlite_tx, &postgres_tx).await;
    copy::<RustcCompilation>(&sqlite_tx, &postgres_tx).await;
    copy::<RuntimePstatSeries>(&sqlite_tx, &postgres_tx).await;
    copy::<RuntimePstat>(&sqlite_tx, &postgres_tx).await;
    copy::<ArtifactSize>(&sqlite_tx, &postgres_tx).await;
    enable_table_triggers(&postgres_tx, &tables).await;

    // This is overly paranoid, but don't commit the Postgres transaction until
    // the rollback of the SQLite transaction succeeds.
    sqlite_tx.rollback().unwrap();
    postgres_tx.commit().await?;

    Ok(())
}

async fn copy<T: Table>(
    sqlite: &rusqlite::Transaction<'_>,
    postgres: &tokio_postgres::Transaction<'_>,
) {
    // We export from SQLite by writing SQLite tables to CSV, and import to
    // Postgres by using the `COPY` command. This is much faster than using
    // `INSERT` commands, even if sending multiple rows per insert, and using
    // multiple tasks to insert concurrently, despite there being some
    // inefficiencies in serializing to CSV.

    // There are SQL injection vulnerabilities below, but it seems extremely
    // unlikely that we will ever execute SQL built from external strings.
    let table = T::name();

    let postgres_columns = postgres
        .query(
            r#"SELECT column_name FROM information_schema.columns
               WHERE table_schema = 'public' AND table_name = $1
               ORDER BY ordinal_position"#,
            &[&table],
        )
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.get(0))
        .collect::<Vec<String>>();
    let attributes = mapping_pg_columns_to_attributes::<T>(&postgres_columns);

    let copy = postgres
        .prepare(&format!(
            r#"copy {} ({}) from stdin (encoding utf8, format csv, null '{}')"#,
            table, attributes, NULL_STRING,
        ))
        .await
        .unwrap();

    let copy_in_sink = postgres.copy_in::<_, Bytes>(&copy).await.unwrap();
    tokio::pin!(copy_in_sink);

    // Writing to a `futures::sink::Sink` from a `csv::Writer` is a bit awkward
    // and inefficient. We'd like it to write directly to the `BytesMut` buffer
    // that the sink will consume (as a split off `Bytes`). Unfortunately, the
    // writer does its own buffering, which is unnecessary in our case since it
    // could hypothetically just write to the `BytesMut` we provide.
    //
    // We can at least have it flush to our `BytesMut`, but there's still the
    // problem of needing to extract it periodically (to avoid having the entire
    // CSV table in memory at once) in order to split off a `Bytes` and send it
    // to the sink. We have to consume the writer to do this, which means we
    // have to create a new one for every chunk of data we want to send.
    let mut csv_writer = postgres_csv_writer(BytesMut::new().writer());

    let mut select = sqlite
        .prepare(&format!("select {} from {}", T::sqlite_attributes(), table))
        .unwrap();

    let start = Instant::now();
    let mut rows = select.query([]).unwrap();
    let mut count = 0;

    const ROWS_PER_SEND: usize = 1024;

    while let Some(result) = rows.next().transpose() {
        let row = result.unwrap();
        T::write_postgres_csv_row(&mut csv_writer, row);
        count += 1;

        if count % ROWS_PER_SEND == 0 {
            // Send batch of rows.
            let mut bytes_writer = csv_writer.into_inner().unwrap();
            let bytes = bytes_writer.get_mut().split().freeze();
            copy_in_sink.send(bytes).await.unwrap();
            csv_writer = postgres_csv_writer(bytes_writer);
        }
    }

    if count % ROWS_PER_SEND != 0 {
        // Send remaining rows.
        let bytes = csv_writer.into_inner().unwrap().into_inner().freeze();
        copy_in_sink.send(bytes).await.unwrap();
    }

    copy_in_sink.close().await.unwrap();

    // We have Postgres tables that use generated attributes declared like `id
    // integer primary key generated always as identity`. Postgres normally
    // generates the values for such attributes using sequences. But when we
    // import tables from the SQLite database, in order to maintain referential
    // integrity, we need to override the generation of the IDs and use the
    // SQLite IDs directly. The sequences used by Postgres need to be updated to
    // account for our imported IDs, otherwise Postgres may later try to use IDs
    // which are already in use.
    //
    // This query updates the sequence used to generate the ID attribute's
    // value. The next value will be one more than the max ID value currently in
    // the table, or one if the table is empty.
    //
    // This is vulnerable to race conditions with concurrent transactions, even
    // if we were to use a serializable isolation mode transaction and lock the
    // table. Serializable isolation mode doesn't prevent serialization
    // anomalies for sequences, locking the table doesn't help because a
    // concurrent transaction can still read or write the corresponding sequence
    // while the table is locked, and sequences themselves can't be locked. This
    // program is designed to run on a fresh Postgres database anyway, so it's
    // not too big of a concern.
    if count > 0 {
        if let Some(generated_id_attr) = T::postgres_generated_id_attribute() {
            postgres
                .execute(
                    &format!(
                        "select setval(
                            pg_get_serial_sequence($1, $2),
                            coalesce(max({}) + 1, 1), false)
                        from {}",
                        generated_id_attr, table
                    ) as &str,
                    &[&table, &generated_id_attr],
                )
                .await
                .unwrap();
        }
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

fn postgres_csv_writer<W: Write>(w: W) -> csv::Writer<W> {
    csv::WriterBuilder::new().has_headers(false).from_writer(w)
}

/// # Panics
/// Panics if the number of `sqlite_attributes` and `postgres_attributes` is mismatched or
/// a corresponding attribute for the column is not found.
fn mapping_pg_columns_to_attributes<T: Table>(postgres_columns: &[impl AsRef<str>]) -> String {
    // We assume that the attributes between SQLite and Postgres have equal lengths.
    // Additionally, the attribute names of Postgres should exactly match the column names in the
    // database. We then attempt to reorder the attributes to mitigate the ordering difference
    // between versions.
    let sl = T::sqlite_attributes()
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();
    let pg = T::postgres_attributes()
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();
    assert_eq!(
        sl.len(),
        pg.len(),
        "The number of attributes in SQLite and Postgres is mismatched."
    );
    let map = pg
        .iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();
    let mut out_attrs = vec![""; pg.len()];
    for col in postgres_columns.iter().map(AsRef::as_ref) {
        let idx = map.get(&col).unwrap_or_else(|| {
            panic!(
                "Failed to find a corresponding attribute for column {} in table {}.",
                col,
                T::name()
            )
        });
        out_attrs[*idx] = col;
    }
    out_attrs.join(", ")
}

async fn get_tables(postgres: &tokio_postgres::Transaction<'_>) -> Vec<String> {
    postgres
        .query(
            "SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname = 'public'",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.get(0))
        .collect()
}

async fn disable_table_triggers(postgres: &tokio_postgres::Transaction<'_>, tables: &[String]) {
    for table in tables {
        postgres
            .execute(&format!("ALTER TABLE {} DISABLE TRIGGER ALL", table), &[])
            .await
            .unwrap();
    }
    eprintln!("Disabled table triggers");
}

async fn enable_table_triggers(postgres: &tokio_postgres::Transaction<'_>, tables: &[String]) {
    for table in tables {
        postgres
            .execute(&format!("ALTER TABLE {} ENABLE TRIGGER ALL", table), &[])
            .await
            .unwrap();
    }
    eprintln!("Enabled table triggers");
}
