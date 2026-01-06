use crate::pool::{
    Connection, ConnectionManager, JobEnqueueResult, ManagedConnection, Transaction,
};
use crate::selector::CompileTestCase;
use crate::{
    ArtifactId, ArtifactIdNumber, Benchmark, BenchmarkJob, BenchmarkJobConclusion,
    BenchmarkJobKind, BenchmarkJobStatus, BenchmarkRequest, BenchmarkRequestIndex,
    BenchmarkRequestInsertResult, BenchmarkRequestStatus, BenchmarkRequestType,
    BenchmarkRequestWithErrors, BenchmarkSet, CodegenBackend, CollectionId, CollectorConfig,
    Commit, CommitType, CompileBenchmark, Date, Index, PendingBenchmarkRequests, Profile, Scenario,
    Target, BENCHMARK_JOB_STATUS_FAILURE_STR, BENCHMARK_JOB_STATUS_IN_PROGRESS_STR,
    BENCHMARK_JOB_STATUS_QUEUED_STR, BENCHMARK_JOB_STATUS_SUCCESS_STR,
    BENCHMARK_REQUEST_MASTER_STR, BENCHMARK_REQUEST_RELEASE_STR,
    BENCHMARK_REQUEST_STATUS_ARTIFACTS_READY_STR, BENCHMARK_REQUEST_STATUS_COMPLETED_STR,
    BENCHMARK_REQUEST_STATUS_IN_PROGRESS_STR, BENCHMARK_REQUEST_STATUS_WAITING_FOR_ARTIFACTS_STR,
    BENCHMARK_REQUEST_TRY_STR,
};
use anyhow::Context as _;
use chrono::{DateTime, TimeZone, Utc};
use hashbrown::{HashMap, HashSet};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_postgres::error::SqlState;
use tokio_postgres::Statement;
use tokio_postgres::{GenericClient, Row};

pub struct Postgres(String, std::sync::Once);

impl Postgres {
    pub fn new(url: String) -> Self {
        Postgres(url, std::sync::Once::new())
    }
}

const CERT_URL: &str = "https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem";

pub async fn make_client(db_url: &str) -> anyhow::Result<tokio_postgres::Client> {
    if db_url.contains("rds.amazonaws.com") {
        let mut builder = TlsConnector::builder();
        for cert in make_certificates().await {
            builder.add_root_certificate(cert);
        }
        let connector = builder.build().context("built TlsConnector")?;
        let connector = MakeTlsConnector::new(connector);

        let (db_client, connection) = match tokio_postgres::connect(db_url, connector).await {
            Ok(v) => v,
            Err(e) => {
                anyhow::bail!("failed to connect to DB: {}", e);
            }
        };
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("database connection error: {e}");
            }
        });

        Ok(db_client)
    } else {
        eprintln!("Warning: Non-TLS connection to non-RDS DB");
        let (db_client, connection) =
            match tokio_postgres::connect(db_url, tokio_postgres::NoTls).await {
                Ok(v) => v,
                Err(e) => {
                    anyhow::bail!("failed to connect to DB: {}", e);
                }
            };
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("database connection error: {e}");
            }
        });

        Ok(db_client)
    }
}
async fn make_certificates() -> Vec<Certificate> {
    use x509_cert::der::pem::LineEnding;
    use x509_cert::der::EncodePem;

    static CERTIFICATE_PEMS: Mutex<Option<Vec<u8>>> = Mutex::const_new(None);

    let mut guard = CERTIFICATE_PEMS.lock().await;
    if guard.is_none() {
        let client = reqwest::Client::new();
        let resp = client
            .get(CERT_URL)
            .send()
            .await
            .expect("failed to get RDS cert");
        let certificate_pems = resp
            .bytes()
            .await
            .expect("failed to get RDS cert body")
            .to_vec();
        *guard = Some(certificate_pems.clone());
    }
    let certs = x509_cert::Certificate::load_pem_chain(&guard.as_ref().unwrap()[..]).unwrap();
    certs
        .into_iter()
        .map(|cert| Certificate::from_pem(cert.to_pem(LineEnding::LF).unwrap().as_bytes()).unwrap())
        .collect()
}

static MIGRATIONS: &[&str] = &[
    "",
    r#"
    create table benchmark(
        name text primary key,
        -- Whether this benchmark supports stable
        stabilized bool
    );
    create table artifact(
        id smallint primary key generated always as identity,
        name text not null unique, -- usually a sha, but also version numbers
        date timestamptz, -- the date when this rustc was created, not the collection date
        type text not null
    );
    -- This identifies an individual run on the collection server, and is used
    -- to distinguish between multiple collections of data for the same artifact.
    create table collection(
        id integer primary key generated always as identity
    );
    create table error_series(
        id integer primary key generated always as identity,
        crate text not null references benchmark(name) on delete cascade on update cascade,
        UNIQUE(crate)
    );
    create table error(
        series integer not null references error_series(id) on delete cascade on update cascade,
        aid smallint not null references artifact(id) on delete cascade on update cascade,
        error text,
        PRIMARY KEY(series, aid)
    );
    create table pstat_series(
        id integer primary key generated always as identity,
        crate text not null references benchmark(name) on delete cascade on update cascade,
        profile text not null,
        cache text not null,
        statistic text not null,
        UNIQUE(crate, profile, cache, statistic)
    );
    create table pstat(
        series integer references pstat_series(id) on delete cascade on update cascade,
        aid smallint references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        value double precision not null,
        PRIMARY KEY(series, aid, cid)
    );
    create table self_profile_query_series(
        id integer primary key generated always as identity,
        crate text not null references benchmark(name) on delete cascade on update cascade,
        profile text not null,
        cache text not null,
        query text not null,
        UNIQUE(crate, profile, cache, query)
    );
    create table self_profile_query(
        series integer references self_profile_query_series(id) on delete cascade on update cascade,
        aid smallint references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        self_time bigint,
        blocked_time bigint,
        incremental_load_time bigint,
        number_of_cache_hits integer,
        invocation_count integer,
        PRIMARY KEY(series, aid, cid)
    );
    create table pull_request_build(
        bors_sha text unique,
        pr integer not null,
        parent_sha text,
        complete boolean,
        requested timestamptz
    );
    "#,
    // Prevent more than one queued entry per PR without a build
    r#"
    create unique index on pull_request_build (pr) where complete = false;
    "#,
    r#"
    create table artifact_collection_duration(
        aid smallint primary key not null references artifact(id) on delete cascade on update cascade,
        date_recorded timestamptz not null,
        duration integer not null
    );
    "#,
    r#"
    create table collector_progress(
        aid smallint not null references artifact(id) on delete cascade on update cascade,
        step text not null,
        start_time timestamptz,
        end_time timestamptz
    );
    "#,
    r#"alter table collector_progress add unique (aid, step);"#,
    r#"alter table collection add column perf_commit text;"#,
    r#"alter table pull_request_build add column include text;"#,
    r#"alter table pull_request_build add column exclude text;"#,
    r#"
    create table raw_self_profile(
        aid smallint references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        crate text not null references benchmark(name) on delete cascade on update cascade,
        profile text not null,
        cache text not null,
        PRIMARY KEY(aid, cid, crate, profile, cache)
    );
    "#,
    r#"alter table pull_request_build add column runs integer;"#,
    r#"
    create table rustc_compilation(
        aid smallint references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        crate text not null,
        duration bigint not null,
        PRIMARY KEY(aid, cid, crate)
    );
    "#,
    r#"alter table artifact alter column id set data type integer;"#,
    r#"
    alter table artifact_collection_duration alter column aid set data type integer;
    alter table collector_progress alter column aid set data type integer;
    alter table error alter column aid set data type integer;
    alter table pstat alter column aid set data type integer;
    alter table raw_self_profile alter column aid set data type integer;
    alter table rustc_compilation alter column aid set data type integer;
    alter table self_profile_query alter column aid set data type integer;
    "#,
    // For the in_progress_steps() query.
    r#"
    create index if not exists collector_progress_start_time_step_idx on collector_progress (start_time, step) where start_time is not null and end_time is not null;
    "#,
    // We default to secondary for all benchmarks, and then let the collector
    // apply new values once it runs.
    r#"
    alter table benchmark add column category text not null DEFAULT 'secondary';
    "#,
    r#"
    alter table pull_request_build add column commit_date timestamptz;
    "#,
    r#"
    create table runtime_pstat_series(
        id integer primary key generated always as identity,
        benchmark text not null,
        metric text not null,
        UNIQUE(benchmark, metric)
    );
    create table runtime_pstat(
        series integer references runtime_pstat_series(id) on delete cascade on update cascade,
        aid integer references artifact(id) on delete cascade on update cascade,
        cid integer references collection(id) on delete cascade on update cascade,
        value double precision not null,
        PRIMARY KEY(series, aid, cid)
    );
"#,
    r#"
    create table error_new(
        aid integer not null references artifact(id) on delete cascade on update cascade,
        benchmark text not null,
        error text not null,
        primary key(aid, benchmark)
    );
    insert into error_new(aid, benchmark, error)
    select aid, crate, error
    from error
    join error_series es on error.series = es.id;

    drop table error;
    drop table error_series;
    alter table error_new rename to error;
"#,
    r#"
    create table artifact_size(
        aid integer references artifact(id) on delete cascade on update cascade,
        component text not null,
        size integer not null,
        UNIQUE(aid, component)
    );
    "#,
    // Add codegen backend column and add it to the unique constraint.
    // Also rename cache to scenario and statistic to metric, while we're at it.
    r#"
    alter table pstat_series rename column cache to scenario;
    alter table pstat_series rename column statistic to metric;
    alter table pstat_series add backend text not null default 'llvm';
    alter table pstat_series drop constraint pstat_series_crate_profile_cache_statistic_key;
    alter table pstat_series add constraint test_case UNIQUE(crate, profile, scenario, backend, metric);
    "#,
    r#"alter table pull_request_build add column backends text;"#,
    // A pstat series shows 1 target
    r#"
    alter table pstat_series add target text not null default 'x86_64-unknown-linux-gnu';
    alter table pstat_series drop constraint test_case;
    alter table pstat_series add constraint test_case UNIQUE(crate, profile, scenario, backend, target, metric);
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS benchmark_request (
        id           SERIAL PRIMARY KEY,
        tag          TEXT NOT NULL UNIQUE,
        parent_sha   TEXT,
        commit_type  TEXT NOT NULL,
        pr           INTEGER,
        created_at   TIMESTAMPTZ NOT NULL,
        completed_at TIMESTAMPTZ,
        status       TEXT NOT NULL,
        backends     TEXT NOT NULL,
        profiles     TEXT NOT NULL
    );
    CREATE INDEX IF NOT EXISTS benchmark_request_status_idx on benchmark_request (status) WHERE status != 'completed';
    "#,
    // Remove that tag cannot be NULL
    r#"ALTER TABLE benchmark_request ALTER COLUMN tag DROP NOT NULL;"#,
    // Prevent multiple try commits without a `sha` and the same `pr` number
    // being added to the table
    r#"CREATE UNIQUE INDEX benchmark_request_pr_commit_type_idx ON benchmark_request (pr, commit_type) WHERE status != 'completed';"#,
    r#"
    CREATE TABLE IF NOT EXISTS collector_config (
        id                SERIAL PRIMARY KEY,
        target            TEXT NOT NULL,
        name              TEXT NOT NULL UNIQUE,
        date_added        TIMESTAMPTZ DEFAULT NOW() NOT NULL,
        last_heartbeat_at TIMESTAMPTZ,
        benchmark_set     INTEGER NOT NULL,
        is_active         BOOLEAN DEFAULT FALSE NOT NULL
    );
    -- Given the current setup, we do not want 2 collectors that are active
    -- with the same target using the same benchmark set.
    CREATE UNIQUE INDEX collector_config_target_bench_active_uniq ON collector_config
        (target, benchmark_set, is_active) WHERE is_active = TRUE;
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS job_queue (
        id            SERIAL PRIMARY KEY,
        request_tag   TEXT NOT NULL,
        target        TEXT NOT NULL,
        backend       TEXT NOT NULL,
        profile       TEXT NOT NULL,
        benchmark_set INTEGER NOT NULL,
        collector_id  INTEGER,
        created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        started_at    TIMESTAMPTZ,
        completed_at  TIMESTAMPTZ,
        status        TEXT NOT NULL,
        retry         INTEGER DEFAULT 0,

        CONSTRAINT job_queue_request_fk
            FOREIGN KEY (request_tag)
            REFERENCES benchmark_request(tag)
            ON DELETE CASCADE,

        CONSTRAINT job_queue_collector
            FOREIGN KEY (collector_id)
            REFERENCES collector_config(id)
            ON DELETE CASCADE,

        CONSTRAINT job_queue_unique
        UNIQUE (
            request_tag,
            target,
            backend,
            profile,
            benchmark_set
        )
    );
    CREATE INDEX IF NOT EXISTS job_queue_request_tag_idx ON job_queue (request_tag);
    "#,
    // The name is unique and not null in the collector config and is simpler
    // to use as we do not expose the `collector_id` in the code
    r#"
    ALTER TABLE job_queue DROP CONSTRAINT IF EXISTS job_queue_collector;
    ALTER TABLE job_queue ADD COLUMN collector_name TEXT;
    ALTER TABLE job_queue
        ADD CONSTRAINT job_queue_collector
            FOREIGN KEY (collector_name)
            REFERENCES collector_config(name)
            ON DELETE CASCADE;
    ALTER TABLE job_queue DROP COLUMN IF EXISTS collector_id;
    CREATE INDEX IF NOT EXISTS job_queue_status_target_benchmark_set_idx ON job_queue (status, target, benchmark_set);
    "#,
    r#"
    ALTER TABLE benchmark_request ADD COLUMN commit_date TIMESTAMPTZ NULL;
    "#,
    r#"
    ALTER TABLE benchmark_request ADD COLUMN duration_ms INTEGER NULL;
    "#,
    r#"
    ALTER TABLE collector_config ADD COLUMN commit_sha TEXT NULL;
    "#,
    r#"
    CREATE TABLE error_new (
        id      SERIAL PRIMARY KEY,
        aid     INTEGER NOT NULL REFERENCES artifact(id) ON DELETE CASCADE ON UPDATE CASCADE,
        message TEXT NOT NULL,
        context TEXT NOT NULL,
        job_id  INTEGER
    );

    INSERT INTO
        error_new (aid, message, context)
    SELECT
        aid,
        error,
        benchmark
    FROM
        error;

    DROP TABLE error;
    ALTER TABLE error_new RENAME TO error;

    CREATE INDEX error_artifact_idx ON error(aid);
    "#,
    // For completed requests we take the last N completed. As the total number
    // of requests grows to make things fast we need an index on the completed_at
    r#"
    CREATE INDEX benchmark_request_completed_idx ON benchmark_request(completed_at);
    "#,
    r#"
    ALTER TABLE job_queue ADD COLUMN kind TEXT NOT NULL DEFAULT 'compiletime';
    "#,
    r#"
    ALTER TABLE job_queue DROP CONSTRAINT job_queue_unique;
    ALTER TABLE job_queue ADD CONSTRAINT job_queue_unique
        UNIQUE (
            request_tag,
            target,
            backend,
            profile,
            kind,
            benchmark_set
        );
    "#,
    // Add target to runtime_pstat_series
    r#"
    ALTER TABLE runtime_pstat_series ADD target TEXT NOT NULL DEFAULT 'x86_64-unknown-linux-gnu';
    ALTER TABLE runtime_pstat_series DROP CONSTRAINT runtime_pstat_series_benchmark_metric_key;
    ALTER TABLE runtime_pstat_series ADD CONSTRAINT runtime_test_case UNIQUE(benchmark, target, metric);
    "#,
    r#"ALTER TABLE job_queue ADD COLUMN is_optional BOOLEAN NOT NULL DEFAULT FALSE"#,
    r#"ALTER TABLE benchmark_request ADD COLUMN targets TEXT NOT NULL DEFAULT ''"#,
];

#[async_trait::async_trait]
impl ConnectionManager for Postgres {
    type Connection = PostgresConnection;
    async fn open(&self) -> Self::Connection {
        let mut client = make_client(&self.0).await.unwrap();
        let mut should_init = false;
        self.1.call_once(|| {
            should_init = true;
        });
        if should_init {
            client
                .batch_execute(
                    "create table if not exists migrations(id integer primary key, query text);",
                )
                .await
                .unwrap();
            let version = client
                .query_one("select max(id) from migrations", &[])
                .await
                .unwrap();
            let version: Option<i32> = version.get(0);
            for (mid, sql) in MIGRATIONS
                .iter()
                .enumerate()
                .skip(version.unwrap_or(0) as usize + 1)
            {
                let tx = client.transaction().await.unwrap();
                tx.batch_execute(sql).await.unwrap();
                tx.execute(
                    "insert into migrations (id, query) VALUES ($1, $2)",
                    &[&(mid as i32), &sql],
                )
                .await
                .unwrap();
                tx.commit().await.unwrap();
            }
        }
        PostgresConnection::new(client).await
    }
    async fn is_valid(&self, conn: &mut Self::Connection) -> bool {
        !conn.conn.is_closed()
    }
}

#[async_trait::async_trait]
impl Transaction for PostgresTransaction<'_> {
    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error> {
        Ok(self.conn.commit().await?)
    }
    async fn finish(self: Box<Self>) -> Result<(), anyhow::Error> {
        Ok(self.conn.rollback().await?)
    }
    fn conn(&mut self) -> &mut dyn Connection {
        self
    }
    fn conn_ref(&self) -> &dyn Connection {
        self
    }
}

pub struct CachedStatements {
    get_pstat: Statement,
    get_rustc_compilation: Statement,
    get_rustc_compilation_by_crate: Statement,
    insert_pstat: Statement,
    insert_rustc: Statement,
    insert_pstat_series: Statement,
    select_pstat_series: Statement,
    get_error: Statement,
    collection_id: Statement,
    get_benchmarks: Statement,
    insert_runtime_pstat_series: Statement,
    select_runtime_pstat_series: Statement,
    insert_runtime_pstat: Statement,
    get_runtime_pstat: Statement,
    record_artifact_size: Statement,
    get_artifact_size: Statement,
    load_benchmark_request_index: Statement,
    get_compile_test_cases_with_measurements: Statement,
    get_last_n_completed_requests_with_errors: Statement,
    get_jobs_of_in_progress_benchmark_requests: Statement,
    load_pending_benchmark_requests: Statement,
}

pub struct PostgresTransaction<'a> {
    statements: Arc<CachedStatements>,
    conn: tokio_postgres::Transaction<'a>,
}

pub struct PostgresConnection {
    statements: Arc<CachedStatements>,
    conn: tokio_postgres::Client,
}

impl From<PostgresConnection> for tokio_postgres::Client {
    fn from(val: PostgresConnection) -> Self {
        val.conn
    }
}

pub trait PClient {
    type Client: Send + Sync + tokio_postgres::GenericClient;
    fn conn(&self) -> &Self::Client;
    fn conn_mut(&mut self) -> &mut Self::Client;
    fn statements(&self) -> &Arc<CachedStatements>;
}

impl<'a> PClient for PostgresTransaction<'a> {
    type Client = tokio_postgres::Transaction<'a>;
    fn conn(&self) -> &Self::Client {
        &self.conn
    }
    fn conn_mut(&mut self) -> &mut Self::Client {
        &mut self.conn
    }
    fn statements(&self) -> &Arc<CachedStatements> {
        &self.statements
    }
}

impl PClient for ManagedConnection<PostgresConnection> {
    type Client = tokio_postgres::Client;
    fn conn(&self) -> &Self::Client {
        &(**self).conn
    }
    fn conn_mut(&mut self) -> &mut Self::Client {
        &mut (**self).conn
    }
    fn statements(&self) -> &Arc<CachedStatements> {
        &self.statements
    }
}

impl PostgresConnection {
    pub async fn new(conn: tokio_postgres::Client) -> Self {
        PostgresConnection {
            statements: Arc::new(CachedStatements {
                get_pstat: conn
                    .prepare("
                         WITH aids AS (
                             select aid, num from unnest($2::int[]) with ordinality aids(aid, num)
                         ),
                         sids AS (
                             select sid, idx from unnest($1::int[]) with ordinality sids(sid, idx)
                         )
                         select ARRAY(
                             (
                                 select min(pstat.value) from aids
                                     left outer join pstat
                                     on (aids.aid = pstat.aid and pstat.series = sids.sid)
                                     group by aids.num
                                     order by aids.num
                             )
                         ) from
                         sids
                         group by (sids.idx, sids.sid)
                         order by sids.idx
                     ")
                    .await
                    .unwrap(),
                get_rustc_compilation: conn.prepare("
                        select aid, min(total)
                        from (
                            select aid, cast(sum(duration) AS bigint) as total
                            from rustc_compilation
                            where aid = any($1)
                            group by aid, cid
                        ) as duration_per_aid_cid
                        group by aid
                    ").await.unwrap(),
                get_rustc_compilation_by_crate: conn.prepare("
                        select
                            aid,
                            crate,
                            min(duration)
                        from rustc_compilation
                        where aid = any($1)
                        group by (aid, crate)
                    ").await.unwrap(),
                insert_pstat: conn
                    .prepare("insert into pstat (series, aid, cid, value) VALUES ($1, $2, $3, $4)")
                    .await
                    .unwrap(),
                insert_rustc: conn
                    .prepare("insert into rustc_compilation (aid, cid, crate, duration) VALUES ($1, $2, $3, $4)")
                    .await
                    .unwrap(),
                get_error: conn.prepare("select context, message from error where aid = $1").await.unwrap(),
                insert_pstat_series: conn.prepare("insert into pstat_series (crate, profile, scenario, backend, target, metric) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING RETURNING id").await.unwrap(),
                select_pstat_series: conn.prepare("select id from pstat_series where crate = $1 and profile = $2 and scenario = $3 and backend = $4 and target = $5 and metric = $6").await.unwrap(),
                collection_id: conn.prepare("insert into collection (perf_commit) VALUES ($1) returning id").await.unwrap(),
                get_benchmarks: conn.prepare("
                    select name, category
                    from benchmark
                ").await.unwrap(),
                insert_runtime_pstat_series: conn.prepare("INSERT INTO runtime_pstat_series (benchmark, target, metric) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING id").await.unwrap(),
                select_runtime_pstat_series: conn.prepare("SELECT id FROM runtime_pstat_series WHERE benchmark = $1 AND target = $2 AND metric = $3").await.unwrap(),
                insert_runtime_pstat: conn
                    .prepare("insert into runtime_pstat (series, aid, cid, value) VALUES ($1, $2, $3, $4)")
                    .await
                    .unwrap(),
                get_runtime_pstat: conn
                    .prepare("
                         WITH aids AS (
                             select aid, num from unnest($2::int[]) with ordinality aids(aid, num)
                         ),
                         sids AS (
                             select sid, idx from unnest($1::int[]) with ordinality sids(sid, idx)
                         )
                         select ARRAY(
                             (
                                 select min(runtime_pstat.value) from aids
                                     left outer join runtime_pstat
                                     on (aids.aid = runtime_pstat.aid and runtime_pstat.series = sids.sid)
                                     group by aids.num
                                     order by aids.num
                             )
                         ) from
                         sids
                         group by (sids.idx, sids.sid)
                         order by sids.idx
                     ")
                    .await
                    .unwrap(),
                record_artifact_size: conn.prepare("
                    insert into artifact_size (aid, component, size)
                    values ($1, $2, $3)
                    on conflict (aid, component)
                    do update
                    set size = excluded.size
                ").await.unwrap(),
                get_artifact_size: conn.prepare("
                    select component, size from artifact_size
                    where aid = $1
                ").await.unwrap(),
                load_benchmark_request_index: conn.prepare("
                    SELECT tag
                    FROM benchmark_request
                    WHERE tag IS NOT NULL
                ").await.unwrap(),
                get_compile_test_cases_with_measurements: conn.prepare("
                    SELECT DISTINCT crate, profile, scenario, backend, target
                    FROM pstat_series
                    WHERE id IN (
                        SELECT DISTINCT series
                        FROM pstat
                        WHERE aid = $1
                    )
                ").await.unwrap(),
                get_last_n_completed_requests_with_errors: conn.prepare(&format!("
                    WITH completed AS (
                        SELECT {BENCHMARK_REQUEST_COLUMNS}
                        FROM benchmark_request
                        WHERE status = $1
                        -- Select last N completed requests
                        ORDER BY completed_at DESC
                        LIMIT $2
                    ), artifacts AS (
                        SELECT artifact.id, name
                        FROM artifact
                        -- Use right join to only return artifacts for selected requests
                        RIGHT JOIN completed ON artifact.name = completed.tag
                    ), errors AS (
                        SELECT
                            artifacts.name AS tag,
                            error.context,
                            error.message
                        FROM error
                        -- Use right join to only return errors for selected artifacts
                        RIGHT JOIN artifacts ON error.aid = artifacts.id
                    )
                    -- Select request duplicated for each pair of (benchmark, error)
                    SELECT
                        completed.*,
                        errors.context,
                        errors.message
                    FROM completed
                    LEFT JOIN errors ON errors.tag = completed.tag
                    -- Re-sort the requests, because the original order may be lost
                    ORDER BY completed.completed_at DESC
                ")).await.unwrap(),
                get_jobs_of_in_progress_benchmark_requests: conn.prepare(&format!("
                    -- Get in progress requests
                    WITH in_progress AS (
                        SELECT tag, parent_sha
                        FROM benchmark_request
                        WHERE status = '{BENCHMARK_REQUEST_STATUS_IN_PROGRESS_STR}' AND
                              tag IS NOT NULL
                    ),
                    -- Get their parents
                    parents AS (
                        SELECT parent_sha AS tag
                        FROM in_progress
                        WHERE parent_sha is not NULL
                    ),
                    -- Concatenate them together (without duplicates)
                    requests AS (
                        SELECT tag FROM in_progress
                        UNION
                        SELECT tag FROM parents
                    )
                    -- Only get the jobs of in_progress requests
                    SELECT *
                    FROM job_queue
                    INNER JOIN requests ON job_queue.request_tag = requests.tag
                    ORDER BY created_at ASC
                ")).await.unwrap(),
                // Load pending benchmark requests, along with information whether their parent is
                // completed or not
                load_pending_benchmark_requests: conn.prepare(&format!("
                    WITH pending AS (
                        SELECT {BENCHMARK_REQUEST_COLUMNS}
                        FROM benchmark_request AS req
                        WHERE status IN ('{BENCHMARK_REQUEST_STATUS_ARTIFACTS_READY_STR}', '{BENCHMARK_REQUEST_STATUS_IN_PROGRESS_STR}')
                    )
                    SELECT (parent.status = '{BENCHMARK_REQUEST_STATUS_COMPLETED_STR}') AS parent_done, pending.*
                    FROM pending
                    LEFT JOIN benchmark_request as parent ON parent.tag = pending.parent_sha
                ")).await.unwrap(),
            }),
            conn,
        }
    }
}

// `tag` should be kept as the first column
const BENCHMARK_REQUEST_COLUMNS: &str =
    "tag, parent_sha, pr, commit_type, status, created_at, completed_at, backends, profiles, commit_date, duration_ms, targets";

#[async_trait::async_trait]
impl<P> Connection for P
where
    P: Send + Sync + PClient,
{
    async fn maybe_create_indices(&mut self) {}
    async fn transaction(&mut self) -> Box<dyn Transaction + '_> {
        let statements = self.statements().clone();
        let tx = self.conn_mut().transaction().await.unwrap();
        Box::new(PostgresTransaction {
            statements,
            conn: tx,
        })
    }

    async fn load_index(&mut self) -> Index {
        Index {
            commits: self
                .conn()
                .query(
                    "select id, name, date, type from artifact where type = 'master' or type = 'try'",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        Commit {
                            sha: row.get::<_, String>(1).as_str().into(),
                            date: {
                                let timestamp: Option<DateTime<Utc>> = row.get(2);
                                match timestamp {
                                    Some(t) => Date(t),
                                    None => Date(Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap()),
                                }
                            },
                            r#type: CommitType::from_str(&row.get::<_, String>(3)).unwrap(),
                        },
                    )
                })
                .collect(),
            artifacts: self
                .conn()
                .query("select id, name from artifact where type = 'release'", &[])
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        row.get::<_, String>(1).as_str().into(),
                    )
                })
                .collect(),
            pstat_series: self
                .conn()
                .query(
                    "select id, crate, profile, scenario, backend, target, metric from pstat_series;",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        (
                            Benchmark::from(row.get::<_, String>(1).as_str()),
                            Profile::from_str(row.get::<_, String>(2).as_str()).unwrap(),
                            row.get::<_, String>(3).as_str().parse().unwrap(),
                            CodegenBackend::from_str(row.get::<_, String>(4).as_str()).unwrap(),
                            Target::from_str(row.get::<_, String>(5).as_str()).unwrap(),
                            row.get::<_, String>(6).as_str().into(),
                        ),
                    )
                })
                .collect(),
            runtime_pstat_series: self
                .conn()
                .query(
                    "SELECT id, benchmark, target, metric FROM runtime_pstat_series;",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        (
                            row.get::<_, &str>(1).into(),
                            Target::from_str(row.get::<_, &str>(2)).unwrap(),
                            row.get::<_, &str>(3).into(),
                        ),
                    )
                })
                .collect(),
        }
    }
    async fn get_compile_benchmarks(&self) -> Vec<CompileBenchmark> {
        let rows = self
            .conn()
            .query(&self.statements().get_benchmarks, &[])
            .await
            .unwrap();
        rows.into_iter()
            .map(|r| CompileBenchmark {
                name: r.get(0),
                category: r.get(1),
            })
            .collect()
    }

    async fn get_pstats(
        &self,
        pstat_series_row_ids: &[u32],
        artifact_row_ids: &[Option<crate::ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let pstat_series_row_ids = pstat_series_row_ids
            .iter()
            .map(|sid| *sid as i32)
            .collect::<Vec<_>>();
        let artifact_row_ids = artifact_row_ids
            .iter()
            .map(|id| id.map(|id| id.0 as i32))
            .collect::<Vec<_>>();
        let rows = self
            .conn()
            .query(
                &self.statements().get_pstat,
                &[&pstat_series_row_ids, &artifact_row_ids],
            )
            .await
            .unwrap();
        rows.into_iter()
            .map(|row| row.get::<_, Vec<Option<f64>>>(0))
            .collect()
    }
    async fn get_runtime_pstats(
        &self,
        runtime_pstat_series_row_ids: &[u32],
        artifact_row_ids: &[Option<crate::ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let runtime_pstat_series_row_ids = runtime_pstat_series_row_ids
            .iter()
            .map(|sid| *sid as i32)
            .collect::<Vec<_>>();
        let artifact_row_ids = artifact_row_ids
            .iter()
            .map(|id| id.map(|id| id.0 as i32))
            .collect::<Vec<_>>();
        let rows = self
            .conn()
            .query(
                &self.statements().get_runtime_pstat,
                &[&runtime_pstat_series_row_ids, &artifact_row_ids],
            )
            .await
            .unwrap();
        rows.into_iter()
            .map(|row| row.get::<_, Vec<Option<f64>>>(0))
            .collect()
    }
    async fn get_error(&self, artifact_row_id: crate::ArtifactIdNumber) -> HashMap<String, String> {
        let rows = self
            .conn()
            .query(&self.statements().get_error, &[&(artifact_row_id.0 as i32)])
            .await
            .unwrap();
        rows.into_iter()
            .map(|row| (row.get(0), row.get(1)))
            .collect()
    }
    async fn collection_id(&self, version: &str) -> CollectionId {
        CollectionId(
            self.conn()
                .query_one(&self.statements().collection_id, &[&version.trim()])
                .await
                .unwrap()
                .get(0),
        )
    }

    async fn record_statistic(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        benchmark: &str,
        profile: Profile,
        scenario: Scenario,
        backend: CodegenBackend,
        target: Target,
        metric: &str,
        stat: f64,
    ) {
        let profile = profile.to_string();
        let scenario = scenario.to_string();
        let backend = backend.to_string();
        let target = target.to_string();
        let sid = self
            .conn()
            .query_opt(
                &self.statements().select_pstat_series,
                &[&benchmark, &profile, &scenario, &backend, &target, &metric],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => {
                self.conn()
                    .query_opt(
                        &self.statements().insert_pstat_series,
                        &[&benchmark, &profile, &scenario, &backend, &target, &metric],
                    )
                    .await
                    .unwrap();
                self.conn()
                    .query_one(
                        &self.statements().select_pstat_series,
                        &[&benchmark, &profile, &scenario, &backend, &target, &metric],
                    )
                    .await
                    .unwrap()
                    .get(0)
            }
        };
        self.conn()
            .execute(
                &self.statements().insert_pstat,
                &[&sid, &(artifact.0 as i32), &{ collection.0 }, &stat],
            )
            .await
            .unwrap();
    }
    async fn record_runtime_statistic(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        benchmark: &str,
        metric: &str,
        target: Target,
        value: f64,
    ) {
        let target = target.to_string();
        let sid = self
            .conn()
            .query_opt(
                &self.statements().select_runtime_pstat_series,
                &[&benchmark, &target, &metric],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => {
                self.conn()
                    .query_opt(
                        &self.statements().insert_runtime_pstat_series,
                        &[&benchmark, &target, &metric],
                    )
                    .await
                    .unwrap();
                self.conn()
                    .query_one(
                        &self.statements().select_runtime_pstat_series,
                        &[&benchmark, &target, &metric],
                    )
                    .await
                    .unwrap()
                    .get(0)
            }
        };
        self.conn()
            .execute(
                &self.statements().insert_runtime_pstat,
                &[&sid, &(artifact.0 as i32), &{ collection.0 }, &value],
            )
            .await
            .unwrap();
    }

    async fn record_rustc_crate(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        value: Duration,
    ) {
        self.conn()
            .execute(
                &self.statements().insert_rustc,
                &[
                    &(artifact.0 as i32),
                    &{ collection.0 },
                    &krate,
                    &(value.as_nanos() as i64),
                ],
            )
            .await
            .unwrap();
    }

    async fn record_artifact_size(&self, artifact: ArtifactIdNumber, component: &str, size: u64) {
        let size: i32 = size.try_into().expect("Too large artifact");
        self.conn()
            .execute(
                &self.statements().record_artifact_size,
                &[&(artifact.0 as i32), &component, &size],
            )
            .await
            .unwrap();
    }

    async fn get_artifact_size(&self, aid: ArtifactIdNumber) -> HashMap<String, u64> {
        let rows = self
            .conn()
            .query(&self.statements().get_artifact_size, &[&(aid.0 as i32)])
            .await
            .unwrap();

        rows.into_iter()
            .map(|row| (row.get::<_, String>(0), row.get::<_, i32>(1) as u64))
            .collect()
    }

    async fn artifact_id(&self, artifact: &ArtifactId) -> ArtifactIdNumber {
        let info = artifact.info();
        let aid = self
            .conn()
            .query_opt("select id from artifact where name = $1", &[&info.name])
            .await
            .unwrap();

        let aid = match aid {
            Some(aid) => aid.get::<_, i32>(0) as u32,
            None => {
                self.conn()
                    .query_opt("insert into artifact (name, date, type) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING id", &[
                        &info.name,
                        &info.date,
                        &info.kind,
                    ])
                    .await
                    .unwrap();
                self.conn()
                    .query_one("select id from artifact where name = $1", &[&info.name])
                    .await
                    .unwrap()
                    .get::<_, i32>(0) as u32
            }
        };

        ArtifactIdNumber(aid)
    }

    async fn record_error(
        &self,
        artifact: ArtifactIdNumber,
        context: &str,
        message: &str,
        job_id: Option<u32>,
    ) {
        self.conn()
            .execute(
                "insert into error (context, aid, message, job_id) VALUES ($1, $2, $3, $4)",
                &[
                    &context,
                    &(artifact.0 as i32),
                    &message,
                    &job_id.map(|id| id as i32),
                ],
            )
            .await
            .unwrap();
    }

    async fn record_compile_benchmark(
        &self,
        benchmark: &str,
        supports_stable: Option<bool>,
        category: String,
    ) {
        if let Some(r) = self
            .conn()
            .query_opt(
                "select stabilized, category from benchmark where name = $1",
                &[&benchmark],
            )
            .await
            .unwrap()
        {
            if Some(r.get::<_, bool>(0)) == supports_stable && r.get::<_, &str>(1) == category {
                return;
            }
        }
        if let Some(stable) = supports_stable {
            self.conn()
                .execute(
                    "insert into benchmark (name, stabilized, category) VALUES ($1, $2, $3)
                ON CONFLICT (name) DO UPDATE SET stabilized = EXCLUDED.stabilized, category = EXCLUDED.category",
                    &[&benchmark, &stable, &category],
                )
                .await
                .unwrap();
        } else {
            self.conn()
                .execute(
                    "insert into benchmark (name, stabilized, category) VALUES ($1, $2, $3)
                ON CONFLICT (name) DO UPDATE SET category = EXCLUDED.category",
                    &[&benchmark, &false, &category],
                )
                .await
                .unwrap();
        }
    }

    async fn collector_start(&self, aid: ArtifactIdNumber, steps: &[String]) {
        // Clean up -- we'll re-insert any missing things in the loop below.
        self.conn()
            .execute(
                "delete from collector_progress where start_time is null or end_time is null;",
                &[],
            )
            .await
            .unwrap();

        for step in steps {
            self.conn()
                .execute(
                    "insert into collector_progress(aid, step) VALUES ($1, $2)
                    ON CONFLICT DO NOTHING",
                    &[&(aid.0 as i32), &step],
                )
                .await
                .unwrap();
        }
    }
    async fn collector_start_step(&self, aid: ArtifactIdNumber, step: &str) -> bool {
        // If we modified a row, then we populated a start time, so we're good
        // to go. Otherwise we should just skip this step.
        self.conn()
            .execute(
                "update collector_progress set start_time = statement_timestamp() \
                where aid = $1 and step = $2 and start_time is null and end_time is null;",
                &[&(aid.0 as i32), &step],
            )
            .await
            .unwrap()
            == 1
    }
    async fn collector_end_step(&self, aid: ArtifactIdNumber, step: &str) {
        self.conn()
            .execute(
                "update collector_progress set end_time = statement_timestamp() \
                where aid = $1 and step = $2 and start_time is not null;",
                &[&(aid.0 as i32), &step],
            )
            .await
            .unwrap();
    }
    async fn collector_remove_step(&self, aid: ArtifactIdNumber, step: &str) {
        self.conn()
            .execute(
                "delete from collector_progress \
                where aid = $1 and step = $2;",
                &[&(aid.0 as i32), &step],
            )
            .await
            .unwrap();
    }
    async fn parent_of(&self, sha: &str) -> Option<String> {
        self.conn()
            .query_opt(
                "select parent_sha from pull_request_build where bors_sha = $1",
                &[&sha],
            )
            .await
            .unwrap()
            .map(|r| r.get(0))
    }
    async fn pr_of(&self, sha: &str) -> Option<u32> {
        self.conn()
            .query_opt("SELECT pr FROM benchmark_request WHERE tag = $1", &[&sha])
            .await
            .unwrap()
            .map(|r| r.get::<_, i32>(0) as u32)
    }
    async fn record_raw_self_profile(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        benchmark: &str,
        profile: Profile,
        scenario: Scenario,
    ) {
        let profile = profile.to_string();
        let scenario = scenario.to_string();
        self.conn().execute(
            "insert into raw_self_profile (aid, cid, crate, profile, cache) VALUES ($1, $2, $3, $4, $5)",
            &[&(artifact.0 as i32), &collection.0, &benchmark, &profile, &scenario],
        ).await.unwrap();
    }
    async fn list_self_profile(
        &self,
        aid: ArtifactId,
        crate_: &str,
        profile: &str,
        scenario: &str,
    ) -> Vec<(ArtifactIdNumber, i32)> {
        self.conn()
            .query(
                "
            select aid, cid from raw_self_profile where
                crate = $1
                and profile = $2
                and cache = $3
                and aid = (select id from artifact where name = $4);
        ",
                &[
                    &crate_,
                    &profile,
                    &scenario,
                    &match aid {
                        ArtifactId::Commit(c) => c.sha,
                        ArtifactId::Tag(a) => a,
                    },
                ],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|r| (ArtifactIdNumber(r.get::<_, i32>(0) as u32), r.get(1)))
            .collect()
    }

    async fn get_bootstrap(&self, aids: &[ArtifactIdNumber]) -> Vec<Option<Duration>> {
        let mut result = vec![None; aids.len()];

        let aid_to_idx = aids
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, v)| (v, idx))
            .collect::<HashMap<ArtifactIdNumber, usize>>();

        let rows = self
            .conn()
            .query(
                &self.statements().get_rustc_compilation,
                &[&aids.iter().map(|v| v.0 as i32).collect::<Vec<_>>()],
            )
            .await
            .unwrap();

        for row in rows {
            let aid = ArtifactIdNumber(row.get::<_, i32>(0) as u32);
            let min_duration = row.get::<_, i64>(1);

            result[aid_to_idx[&aid]] = Some(Duration::from_nanos(min_duration as u64));
        }

        result
    }

    async fn get_bootstrap_by_crate(
        &self,
        aids: &[ArtifactIdNumber],
    ) -> HashMap<String, Vec<Option<Duration>>> {
        let mut result = HashMap::new();
        let aid_to_idx = aids
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, v)| (v, idx))
            .collect::<HashMap<ArtifactIdNumber, usize>>();
        let rows = self
            .conn()
            .query(
                &self.statements().get_rustc_compilation_by_crate,
                &[&aids.iter().map(|v| v.0 as i32).collect::<Vec<_>>()],
            )
            .await
            .unwrap();

        for row in rows {
            let aid = ArtifactIdNumber(row.get::<_, i32>(0) as u32);
            let krate = row.get::<_, String>(1);
            let min_duration = row.get::<_, i64>(2);

            let v = result
                .entry(krate)
                .or_insert_with(|| vec![None; aids.len()]);
            v[aid_to_idx[&aid]] = Some(Duration::from_nanos(min_duration as u64));
        }

        result
    }

    async fn artifact_by_name(&self, artifact: &str) -> Option<ArtifactId> {
        let row = self
            .conn()
            .query_opt(
                "select date, type from artifact where name = $1",
                &[&artifact],
            )
            .await
            .unwrap()?;
        let date = row.get::<_, Option<DateTime<Utc>>>(0);
        let ty = row.get::<_, String>(1);
        Some(parse_artifact_id(&ty, artifact, date))
    }

    async fn purge_artifact(&self, aid: &ArtifactId) {
        // Once we delete the artifact, all data associated with it should also be deleted
        // thanks to ON DELETE CASCADE.
        let info = aid.info();
        self.conn()
            .execute("DELETE FROM artifact WHERE name = $1", &[&info.name])
            .await
            .unwrap();
        self.conn()
            .execute(
                "DELETE FROM benchmark_request WHERE tag = $1",
                &[&info.name],
            )
            .await
            .unwrap();
    }

    async fn insert_benchmark_request(
        &self,
        benchmark_request: &BenchmarkRequest,
    ) -> anyhow::Result<BenchmarkRequestInsertResult> {
        let row_insert_count = self
            .conn()
            .execute(
                r#"
                INSERT INTO benchmark_request(
                    tag,
                    parent_sha,
                    pr,
                    commit_type,
                    status,
                    created_at,
                    backends,
                    profiles,
                    commit_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT DO NOTHING;
            "#,
                &[
                    &benchmark_request.tag(),
                    &benchmark_request.parent_sha(),
                    &benchmark_request.pr().map(|it| it as i32),
                    &benchmark_request.commit_type,
                    &benchmark_request.status.as_str(),
                    &benchmark_request.created_at,
                    &benchmark_request.backends,
                    &benchmark_request.profiles,
                    &benchmark_request.commit_date,
                ],
            )
            .await
            .context("Failed to insert benchmark request")?;
        if row_insert_count == 0 {
            // Allows us to handle duplicated cases without the database auto
            // erroring
            Ok(BenchmarkRequestInsertResult::NothingInserted)
        } else {
            Ok(BenchmarkRequestInsertResult::Inserted)
        }
    }

    async fn load_benchmark_request_index(&self) -> anyhow::Result<BenchmarkRequestIndex> {
        let requests = self
            .conn()
            .query(&self.statements().load_benchmark_request_index, &[])
            .await
            .context("Cannot load benchmark request index")?;

        let all = requests
            .into_iter()
            .map(|row| row.get::<_, String>(0))
            .collect();
        Ok(BenchmarkRequestIndex { all })
    }

    async fn update_benchmark_request_status(
        &self,
        tag: &str,
        status: BenchmarkRequestStatus,
    ) -> anyhow::Result<()> {
        // We cannot use this function to mark requests as complete, as
        // we need to know if all jobs are complete first.
        if matches!(status, BenchmarkRequestStatus::Completed { .. }) {
            panic!("Please use `mark_benchmark_request_as_completed(...)` to complete benchmark_requests");
        }

        let status_str = status.as_str();
        let modified_rows = self
            .conn()
            .execute(
                r#"
                UPDATE benchmark_request
                SET status = $1
                WHERE tag = $2;"#,
                &[&status_str, &tag],
            )
            .await
            .context("failed to update benchmark request status")?;
        if modified_rows == 0 {
            Err(anyhow::anyhow!(
                "Could not update status of benchmark request with tag `{tag}`, it was not found."
            ))
        } else {
            Ok(())
        }
    }

    async fn attach_shas_to_try_benchmark_request(
        &self,
        pr: u32,
        sha: &str,
        parent_sha: &str,
        commit_date: DateTime<Utc>,
    ) -> anyhow::Result<bool> {
        let modified_rows = self
            .conn()
            .execute(
                "UPDATE benchmark_request
                SET
                    tag = $1,
                    parent_sha = $2,
                    status = $3,
                    commit_date = $6
                WHERE
                    pr = $4
                    AND commit_type = 'try'
                    AND tag IS NULL
                    AND status = $5;",
                &[
                    &sha,
                    &parent_sha,
                    &BENCHMARK_REQUEST_STATUS_ARTIFACTS_READY_STR,
                    &(pr as i32),
                    &BENCHMARK_REQUEST_STATUS_WAITING_FOR_ARTIFACTS_STR,
                    &commit_date,
                ],
            )
            .await
            .context("failed to attach SHAs to try benchmark request")?;

        Ok(modified_rows > 0)
    }

    async fn load_pending_benchmark_requests(&self) -> anyhow::Result<PendingBenchmarkRequests> {
        let rows = self
            .conn()
            .query(&self.statements().load_pending_benchmark_requests, &[])
            .await
            .context("Failed to get pending benchmark requests")?;

        let mut completed_parent_tags = HashSet::new();
        let mut requests = Vec::with_capacity(rows.len());
        for row in rows {
            let parent_done = row.get::<_, Option<bool>>(0);
            let request = row_to_benchmark_request(&row, Some(1));
            if let Some(true) = parent_done {
                if let Some(parent) = request.parent_sha() {
                    completed_parent_tags.insert(parent.to_string());
                }
            }
            requests.push(request);
        }

        Ok(PendingBenchmarkRequests {
            requests,
            completed_parent_tags,
        })
    }

    async fn enqueue_benchmark_job(
        &self,
        request_tag: &str,
        target: Target,
        backend: CodegenBackend,
        profile: Profile,
        benchmark_set: u32,
        kind: BenchmarkJobKind,
        is_optional: bool,
    ) -> JobEnqueueResult {
        // This will return zero rows if the job already exists
        let result = self
            .conn()
            .query(
                r#"
            INSERT INTO job_queue(
                request_tag,
                target,
                backend,
                profile,
                benchmark_set,
                status,
                kind,
                is_optional
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT DO NOTHING
            RETURNING job_queue.id
                "#,
                &[
                    &request_tag,
                    &target,
                    &backend,
                    &profile,
                    &(benchmark_set as i32),
                    &BENCHMARK_JOB_STATUS_QUEUED_STR,
                    &kind,
                    &is_optional,
                ],
            )
            .await;

        match result {
            Ok(rows) => {
                let Some(row) = rows.into_iter().next() else {
                    return JobEnqueueResult::JobAlreadyExisted;
                };
                JobEnqueueResult::JobCreated(row.get::<_, i32>(0) as u32)
            }
            Err(e) => {
                if let Some(db_err) = e.as_db_error() {
                    if db_err.code() == &SqlState::FOREIGN_KEY_VIOLATION {
                        let constraint = db_err.constraint().unwrap_or("benchmark_tag constraint");
                        let detail = db_err.detail().unwrap_or("");
                        return JobEnqueueResult::RequestShaNotFound {
                            error: format!("Foreign key violation on `{constraint}`: {detail}"),
                        };
                    }
                }
                JobEnqueueResult::Other(e.into())
            }
        }
    }

    async fn get_compile_test_cases_with_measurements(
        &self,
        artifact_row_id: &ArtifactIdNumber,
    ) -> anyhow::Result<HashSet<CompileTestCase>> {
        let rows = self
            .conn()
            .query(
                &self.statements().get_compile_test_cases_with_measurements,
                &[&(artifact_row_id.0 as i32)],
            )
            .await
            .context("cannot query compile-time test cases with measurements")?;
        Ok(rows
            .into_iter()
            .map(|row| CompileTestCase {
                benchmark: Benchmark::from(row.get::<_, &str>(0)),
                profile: Profile::from_str(row.get::<_, &str>(1)).unwrap(),
                scenario: row.get::<_, &str>(2).parse().unwrap(),
                backend: CodegenBackend::from_str(row.get::<_, &str>(3)).unwrap(),
                target: Target::from_str(row.get::<_, &str>(4)).unwrap(),
            })
            .collect())
    }

    async fn add_collector_config(
        &self,
        collector_name: &str,
        target: Target,
        benchmark_set: u32,
        is_active: bool,
    ) -> anyhow::Result<CollectorConfig> {
        let row = self
            .conn()
            .query_one(
                "INSERT INTO collector_config(
                     name,
                     target,
                     date_added,
                     last_heartbeat_at,
                     benchmark_set,
                     is_active
                 ) VALUES (
                     $1,
                     $2,
                     NOW(),
                     NOW(),
                     $3,
                     $4
                 )
                RETURNING
                    last_heartbeat_at,
                    date_added
                ",
                &[
                    &collector_name,
                    &target,
                    &(benchmark_set as i32),
                    &is_active,
                ],
            )
            .await
            .context("failed to create collector config")?;

        let collector_config = CollectorConfig {
            name: collector_name.into(),
            target,
            benchmark_set: BenchmarkSet(benchmark_set),
            is_active,
            last_heartbeat_at: row.get::<_, DateTime<Utc>>(0),
            date_added: row.get::<_, DateTime<Utc>>(1),
            commit_sha: None,
        };
        Ok(collector_config)
    }

    async fn start_collector(
        &self,
        collector_name: &str,
        commit_sha: &str,
    ) -> anyhow::Result<Option<CollectorConfig>> {
        let row = self
            .conn()
            .query_opt(
                "
                UPDATE collector_config
                SET
                    last_heartbeat_at = NOW(),
                    commit_sha = $2
                WHERE
                    name = $1 AND
                    is_active = true
                RETURNING
                    target,
                    benchmark_set,
                    is_active,
                    last_heartbeat_at,
                    date_added",
                &[&collector_name, &commit_sha],
            )
            .await?;

        Ok(row
            .map(|row| {
                anyhow::Ok(CollectorConfig {
                    name: collector_name.into(),
                    target: Target::from_str(row.get::<_, &str>(0))
                        .map_err(|e| anyhow::anyhow!(e))?,
                    benchmark_set: BenchmarkSet(row.get::<_, i32>(1) as u32),
                    is_active: row.get::<_, bool>(2),
                    last_heartbeat_at: row.get::<_, DateTime<Utc>>(3),
                    date_added: row.get::<_, DateTime<Utc>>(4),
                    commit_sha: Some(commit_sha.to_string()),
                })
            })
            .transpose()?)
    }

    async fn dequeue_benchmark_job(
        &self,
        collector_name: &str,
        target: Target,
        benchmark_set: BenchmarkSet,
    ) -> anyhow::Result<Option<(BenchmarkJob, ArtifactId)>> {
        // We take the oldest job from the job_queue matching the benchmark_set,
        // target and status of 'queued' or 'in_progress'
        // If a job was dequeued, we increment its retry (dequeue) count
        let row_opt = self
            .conn()
            .query_opt(
                "
                WITH picked AS (
                    SELECT
                        id
                    FROM
                        job_queue
                    WHERE
                        -- Take queued or in-progress jobs
                        (status = $1 OR status = $5)
                        AND target = $2
                        AND benchmark_set = $3
                    ORDER BY
                        -- Prefer in-progress jobs that have not been finished
                        -- previously, so that we can finish them.
                        CASE
                            WHEN status = $5 THEN 0
                            WHEN status = $1 THEN 1
                            ELSE 2
                        END,
                        request_tag,
                        created_at
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                ), updated AS (
                    UPDATE
                        job_queue
                    SET
                        collector_name = $4,
                        started_at = NOW(),
                        status = $5,
                        retry = retry + 1
                    FROM
                        picked
                    WHERE
                        job_queue.id = picked.id
                    RETURNING job_queue.*
                )
                SELECT
                    updated.id,
                    updated.backend,
                    updated.profile,
                    updated.request_tag,
                    updated.created_at,
                    updated.started_at,
                    updated.retry,
                    updated.kind,
                    updated.is_optional,
                    br.commit_type,
                    br.commit_date
                FROM updated
                JOIN benchmark_request as br ON br.tag = updated.request_tag;",
                &[
                    &BENCHMARK_JOB_STATUS_QUEUED_STR,
                    &target,
                    &(benchmark_set.0 as i32),
                    &collector_name,
                    &BENCHMARK_JOB_STATUS_IN_PROGRESS_STR,
                ],
            )
            .await?;

        match row_opt {
            None => Ok(None),
            Some(row) => {
                let job = BenchmarkJob {
                    id: row.get::<_, i32>(0) as u32,
                    target,
                    backend: CodegenBackend::from_str(row.get::<_, &str>(1))
                        .map_err(|e| anyhow::anyhow!(e))?,
                    profile: Profile::from_str(row.get::<_, &str>(2))
                        .map_err(|e| anyhow::anyhow!(e))?,
                    request_tag: row.get::<_, String>(3),
                    benchmark_set,
                    created_at: row.get::<_, DateTime<Utc>>(4),
                    // The job is now in an in_progress state
                    status: BenchmarkJobStatus::InProgress {
                        started_at: row.get::<_, DateTime<Utc>>(5),
                        collector_name: collector_name.into(),
                    },
                    deque_counter: row.get::<_, i32>(6) as u32,
                    kind: BenchmarkJobKind::from_str(row.get::<_, &str>(7))
                        .map_err(|e| anyhow::anyhow!(e))?,
                    is_optional: row.get::<_, bool>(8),
                };
                let commit_type = row.get::<_, &str>(9);
                let commit_date = row.get::<_, Option<DateTime<Utc>>>(10);

                let commit_date = Date(commit_date.ok_or_else(|| {
                    anyhow::anyhow!("Dequeuing job for a benchmark request without commit date")
                })?);
                let artifact_id = match commit_type {
                    BENCHMARK_REQUEST_TRY_STR => ArtifactId::Commit(Commit {
                        sha: job.request_tag.clone(),
                        date: commit_date,
                        r#type: CommitType::Try,
                    }),
                    BENCHMARK_REQUEST_MASTER_STR => ArtifactId::Commit(Commit {
                        sha: job.request_tag.clone(),
                        date: commit_date,
                        r#type: CommitType::Master,
                    }),
                    BENCHMARK_REQUEST_RELEASE_STR => ArtifactId::Tag(job.request_tag.clone()),
                    _ => panic!(
                        "Invalid commit type {commit_type} for benchmark request {}",
                        job.request_tag
                    ),
                };

                Ok(Some((job, artifact_id)))
            }
        }
    }

    async fn maybe_mark_benchmark_request_as_completed(&self, tag: &str) -> anyhow::Result<bool> {
        // Find if the benchmark is completed and update it's status to completed
        // in one SQL block
        let row = self
            .conn()
            .query_opt(
                "
                UPDATE
                    benchmark_request
                SET
                    status = $1,
                    completed_at = NOW(),
                    duration_ms = (
                        SELECT (MAX(EXTRACT('epoch' FROM job_queue.completed_at)) -
                                MIN(EXTRACT('epoch' FROM job_queue.started_at))) * 1000 AS duration_ms
                        FROM
                            job_queue
                        WHERE job_queue.request_tag = $2
                    )
                WHERE
                    benchmark_request.tag = $2
                    AND benchmark_request.status != $1
                    AND NOT EXISTS (
                        SELECT
                            1
                        FROM
                            job_queue
                        WHERE
                            job_queue.request_tag = benchmark_request.tag
                            AND job_queue.status NOT IN ($3, $4)
                            AND job_queue.is_optional = FALSE
                    )
                    AND (
                        benchmark_request.parent_sha IS NULL
                        OR NOT EXISTS (
                            SELECT
                                1
                            FROM
                                job_queue
                            WHERE
                                job_queue.request_tag = benchmark_request.parent_sha
                                AND job_queue.status NOT IN ($3, $4)
                       )
                )
                RETURNING
                    benchmark_request.tag;
                ",
                &[
                    &BENCHMARK_REQUEST_STATUS_COMPLETED_STR,
                    &tag,
                    &BENCHMARK_JOB_STATUS_SUCCESS_STR,
                    &BENCHMARK_JOB_STATUS_FAILURE_STR,
                ],
            )
            .await
            .context("Failed to mark benchmark_request as completed")?;
        // The affected tag is returned by the query thus we can use the row's
        // presence to determine if the request was marked as completed
        Ok(row.is_some())
    }

    async fn mark_benchmark_job_as_completed(
        &self,
        id: u32,
        conclusion: BenchmarkJobConclusion,
    ) -> anyhow::Result<()> {
        self.conn()
            .execute(
                "
                UPDATE
                    job_queue
                SET
                    status = $1,
                    completed_at = NOW()
                WHERE
                    id = $2",
                &[&conclusion.as_str(), &(id as i32)],
            )
            .await
            .context("Failed to mark benchmark job as completed")?;
        Ok(())
    }

    async fn get_last_n_completed_benchmark_requests(
        &self,
        count: u64,
    ) -> anyhow::Result<Vec<BenchmarkRequestWithErrors>> {
        let rows = self
            .conn()
            .query(
                &self.statements().get_last_n_completed_requests_with_errors,
                &[&BENCHMARK_REQUEST_STATUS_COMPLETED_STR, &(count as i64)],
            )
            .await?;

        // Iterate through the requests and aggregate their errors
        // Make sure to keep their original order
        let mut requests = vec![];
        // tag -> errors
        let mut errors: HashMap<String, HashMap<String, String>> = Default::default();

        for row in rows {
            let tag = row.get::<_, &str>(0);
            let error_benchmark = row.get::<_, Option<String>>(12);
            let error_content = row.get::<_, Option<String>>(13);

            // We already saw this request, just add errors
            if let Some(errors) = errors.get_mut(tag) {
                if let Some(benchmark) = error_benchmark {
                    errors.insert(benchmark, error_content.unwrap_or_default());
                }
            } else {
                // We see this request for the first time
                let request = row_to_benchmark_request(&row, None);
                let request_errors = if let Some(benchmark) = error_benchmark {
                    HashMap::from([(benchmark, error_content.unwrap_or_default())])
                } else {
                    HashMap::new()
                };
                errors.insert(tag.to_string(), request_errors);
                requests.push(request);
            }
        }

        Ok(requests
            .into_iter()
            .map(|request| {
                let errors = errors.remove(request.tag().unwrap()).unwrap_or_default();
                BenchmarkRequestWithErrors { request, errors }
            })
            .collect())
    }

    async fn get_jobs_of_in_progress_benchmark_requests(
        &self,
    ) -> anyhow::Result<HashMap<String, Vec<BenchmarkJob>>> {
        let rows = self
            .conn()
            .query(
                &self.statements().get_jobs_of_in_progress_benchmark_requests,
                &[],
            )
            .await?;

        let mut request_to_jobs: HashMap<String, Vec<BenchmarkJob>> = HashMap::new();
        for row in rows {
            let started_at = row.get::<_, Option<DateTime<Utc>>>(7);
            let status = row.get::<_, &str>(9);
            let collector_name = row.get::<_, Option<String>>(11);
            let status = match status {
                BENCHMARK_JOB_STATUS_QUEUED_STR => BenchmarkJobStatus::Queued,
                BENCHMARK_JOB_STATUS_IN_PROGRESS_STR => BenchmarkJobStatus::InProgress {
                    started_at: started_at.expect("started_at was null for an in progress job"),
                    collector_name: collector_name
                        .expect("Collector is missing for an in progress job"),
                },
                BENCHMARK_JOB_STATUS_FAILURE_STR | BENCHMARK_JOB_STATUS_SUCCESS_STR => {
                    BenchmarkJobStatus::Completed {
                        started_at: started_at.expect("started_at was null for a finished job"),
                        completed_at: row.get::<_, DateTime<Utc>>(8),
                        collector_name: collector_name
                            .expect("Collector is missing for an in progress job"),
                        success: status == BENCHMARK_JOB_STATUS_SUCCESS_STR,
                    }
                }
                _ => panic!("Invalid job status {status}"),
            };
            let job = BenchmarkJob {
                id: row.get::<_, i32>(0) as u32,
                request_tag: row.get::<_, String>(1),
                target: Target::from_str(row.get::<_, &str>(2)).map_err(|e| anyhow::anyhow!(e))?,
                backend: CodegenBackend::from_str(row.get::<_, &str>(3))
                    .map_err(|e| anyhow::anyhow!(e))?,
                profile: Profile::from_str(row.get::<_, &str>(4))
                    .map_err(|e| anyhow::anyhow!(e))?,
                benchmark_set: BenchmarkSet(row.get::<_, i32>(5) as u32),
                created_at: row.get::<_, DateTime<Utc>>(6),
                status,
                deque_counter: row.get::<_, i32>(10) as u32,
                kind: BenchmarkJobKind::from_str(row.get::<_, &str>(12))
                    .map_err(|e| anyhow::anyhow!(e))?,
                is_optional: row.get::<_, bool>(13),
            };
            request_to_jobs
                .entry(job.request_tag.clone())
                .or_default()
                .push(job);
        }
        Ok(request_to_jobs)
    }

    async fn get_collector_configs(&self) -> anyhow::Result<Vec<CollectorConfig>> {
        let rows = self
            .conn()
            .query(
                "SELECT
                    name,
                    target,
                    benchmark_set,
                    is_active,
                    last_heartbeat_at,
                    date_added,
                    commit_sha
                FROM
                    collector_config;",
                &[],
            )
            .await?;

        let configs = rows
            .into_iter()
            .map(|row| {
                Ok(CollectorConfig {
                    name: row.get::<_, String>(0),
                    target: Target::from_str(row.get::<_, &str>(1))
                        .map_err(|e| anyhow::anyhow!(e))?,
                    benchmark_set: BenchmarkSet(row.get::<_, i32>(2) as u32),
                    is_active: row.get::<_, bool>(3),
                    last_heartbeat_at: row.get::<_, DateTime<Utc>>(4),
                    date_added: row.get::<_, DateTime<Utc>>(5),
                    commit_sha: row.get::<_, Option<String>>(6),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(configs)
    }

    async fn update_collector_heartbeat(&self, collector_name: &str) -> anyhow::Result<()> {
        self.conn()
            .query(
                r#"
                UPDATE collector_config
                SET last_heartbeat_at = NOW()
                WHERE name = $1
                "#,
                &[&collector_name],
            )
            .await?;
        Ok(())
    }

    fn supports_job_queue(&self) -> bool {
        true
    }
}

fn row_to_benchmark_request(row: &Row, row_offset: Option<usize>) -> BenchmarkRequest {
    let row_offset = row_offset.unwrap_or(0);
    let tag = row.get::<_, Option<String>>(row_offset);
    let parent_sha = row.get::<_, Option<String>>(1 + row_offset);
    let pr = row.get::<_, Option<i32>>(2 + row_offset);
    let commit_type = row.get::<_, &str>(3 + row_offset);
    let status = row.get::<_, &str>(4 + row_offset);
    let created_at = row.get::<_, DateTime<Utc>>(5 + row_offset);
    let completed_at = row.get::<_, Option<DateTime<Utc>>>(6 + row_offset);
    let backends = row.get::<_, String>(7 + row_offset);
    let profiles = row.get::<_, String>(8 + row_offset);
    let commit_date = row.get::<_, Option<DateTime<Utc>>>(9 + row_offset);
    let duration_ms = row.get::<_, Option<i32>>(10 + row_offset);
    let targets = row.get::<_, String>(11 + row_offset);

    let pr = pr.map(|v| v as u32);

    let status =
        BenchmarkRequestStatus::from_str_and_completion_date(status, completed_at, duration_ms)
            .unwrap_or_else(|e| {
                panic!("Invalid BenchmarkRequestStatus data in the database for tag {tag:?}: {e:?}")
            });

    match commit_type {
        BENCHMARK_REQUEST_TRY_STR => BenchmarkRequest {
            commit_type: BenchmarkRequestType::Try {
                sha: tag,
                parent_sha,
                pr: pr.expect("Try commit in the DB without a PR"),
            },
            commit_date,
            created_at,
            status,
            backends,
            profiles,
            targets,
        },
        BENCHMARK_REQUEST_MASTER_STR => BenchmarkRequest {
            commit_type: BenchmarkRequestType::Master {
                sha: tag.expect("Master commit in the DB without a SHA"),
                parent_sha: parent_sha.expect("Master commit in the DB without a parent SHA"),
                pr: pr.expect("Master commit in the DB without a PR"),
            },
            commit_date,
            created_at,
            status,
            backends,
            profiles,
            targets,
        },
        BENCHMARK_REQUEST_RELEASE_STR => BenchmarkRequest {
            commit_type: BenchmarkRequestType::Release {
                tag: tag.expect("Release commit in the DB without a SHA"),
            },
            commit_date,
            created_at,
            status,
            backends,
            profiles,
            targets,
        },
        _ => panic!("Invalid `commit_type` for `BenchmarkRequest` {commit_type}",),
    }
}

fn parse_artifact_id(ty: &str, sha: &str, date: Option<DateTime<Utc>>) -> ArtifactId {
    match ty {
        "master" => ArtifactId::Commit(Commit {
            sha: sha.to_owned(),
            date: Date(date.expect("date present for master commits")),
            r#type: CommitType::Master,
        }),
        "try" => ArtifactId::Commit(Commit {
            sha: sha.to_owned(),
            date: date
                .map(Date)
                .unwrap_or_else(|| Date::ymd_hms(2000, 1, 1, 0, 0, 0)),
            r#type: CommitType::Try,
        }),
        "release" => ArtifactId::Tag(sha.to_owned()),
        _ => panic!("unknown artifact type: {ty:?}"),
    }
}

macro_rules! impl_to_postgresql_via_to_string {
    ($t:ty) => {
        impl tokio_postgres::types::ToSql for $t {
            fn to_sql(
                &self,
                ty: &tokio_postgres::types::Type,
                out: &mut bytes::BytesMut,
            ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                self.to_string().to_sql(ty, out)
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool {
                <String as tokio_postgres::types::ToSql>::accepts(ty)
            }

            // Only compile if the type is acceptable
            tokio_postgres::types::to_sql_checked!();
        }
    };
}

impl_to_postgresql_via_to_string!(BenchmarkRequestType);
impl_to_postgresql_via_to_string!(Target);
impl_to_postgresql_via_to_string!(CodegenBackend);
impl_to_postgresql_via_to_string!(Profile);
impl_to_postgresql_via_to_string!(BenchmarkJobKind);

#[cfg(test)]
mod tests {
    use super::make_certificates;

    // Makes sure we successfully parse the RDS certificates and load them into native-tls compatible
    // format.
    #[tokio::test]
    async fn can_make_certificates() {
        let certs = make_certificates().await;
        assert!(!certs.is_empty());
    }
}
