use crate::pool::{Connection, ConnectionManager, ManagedConnection, Transaction};
use crate::{
    ArtifactCollection, ArtifactId, ArtifactIdNumber, Benchmark, CodegenBackend, CollectionId,
    Commit, CommitType, CompileBenchmark, Date, Index, Profile, QueuedCommit, Scenario,
};
use anyhow::Context as _;
use chrono::{DateTime, TimeZone, Utc};
use hashbrown::HashMap;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio_postgres::GenericClient;
use tokio_postgres::Statement;

pub struct Postgres(String, std::sync::Once);

impl Postgres {
    pub fn new(url: String) -> Self {
        Postgres(url, std::sync::Once::new())
    }
}

const CERT_URL: &str = "https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem";

lazy_static::lazy_static! {
    static ref CERTIFICATE_PEMS: Vec<u8> = {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(CERT_URL)
            .send()
            .expect("failed to get RDS cert");
         resp.bytes().expect("failed to get RDS cert body").to_vec()
    };
}

async fn make_client(db_url: &str) -> anyhow::Result<tokio_postgres::Client> {
    if db_url.contains("rds.amazonaws.com") {
        let mut builder = TlsConnector::builder();
        for cert in make_certificates() {
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
                eprintln!("database connection error: {}", e);
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
                eprintln!("database connection error: {}", e);
            }
        });

        Ok(db_client)
    }
}
fn make_certificates() -> Vec<Certificate> {
    use x509_cert::der::pem::LineEnding;
    use x509_cert::der::EncodePem;

    let certs = x509_cert::Certificate::load_pem_chain(&CERTIFICATE_PEMS[..]).unwrap();
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
impl<'a> Transaction for PostgresTransaction<'a> {
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
    record_duration: Statement,
    in_progress_steps: Statement,
    get_benchmarks: Statement,
    insert_runtime_pstat_series: Statement,
    select_runtime_pstat_series: Statement,
    insert_runtime_pstat: Statement,
    get_runtime_pstat: Statement,
    record_artifact_size: Statement,
    get_artifact_size: Statement,
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
                get_error: conn.prepare("select benchmark, error from error where aid = $1").await.unwrap(),
                insert_pstat_series: conn.prepare("insert into pstat_series (crate, profile, scenario, backend, metric) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING RETURNING id").await.unwrap(),
                select_pstat_series: conn.prepare("select id from pstat_series where crate = $1 and profile = $2 and scenario = $3 and backend = $4 and metric = $5").await.unwrap(),
                collection_id: conn.prepare("insert into collection (perf_commit) VALUES ($1) returning id").await.unwrap(),
                record_duration: conn.prepare("
                    insert into artifact_collection_duration (
                        aid,
                        date_recorded,
                        duration
                    ) VALUES ($1, CURRENT_TIMESTAMP, $2)
                    ON CONFLICT DO NOTHING
                ").await.unwrap(),
                in_progress_steps: conn.prepare("
                select step,
                    end_time is not null,
                    extract(epoch from interval '0 seconds'::interval +
                        coalesce(end_time, statement_timestamp()) - start_time)::int4,
                    extract(
                        epoch from interval '0 seconds'::interval +
                        (select cp.end_time - cp.start_time
                        from collector_progress as cp
                        join artifact on artifact.id = cp.aid
                            where
                                cp.aid != $1
                                and cp.step = collector_progress.step
                                and cp.start_time is not null
                                and cp.end_time is not null
                                and artifact.type = 'master'
                            order by start_time desc
                            limit 1
                        ))::int4
                from collector_progress where aid = $1 order by step
                ").await.unwrap(),
                get_benchmarks: conn.prepare("
                    select name, category
                    from benchmark
                ").await.unwrap(),
                insert_runtime_pstat_series: conn.prepare("insert into runtime_pstat_series (benchmark, metric) VALUES ($1, $2) ON CONFLICT DO NOTHING RETURNING id").await.unwrap(),
                select_runtime_pstat_series: conn.prepare("select id from runtime_pstat_series where benchmark = $1 and metric = $2").await.unwrap(),
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
                ").await.unwrap()
            }),
            conn,
        }
    }
}

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

    async fn record_duration(&self, artifact: ArtifactIdNumber, duration: Duration) {
        self.conn()
            .execute(
                &self.statements().record_duration,
                &[&(artifact.0 as i32), &(duration.as_secs() as i32)],
            )
            .await
            .unwrap();
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
                            r#type: CommitType::from_str(&row.get::<_, String>(3)).unwrap()
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
                    "select id, crate, profile, scenario, backend, metric from pstat_series;",
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
                            row.get::<_, String>(5).as_str().into(),
                        ),
                    )
                })
                .collect(),
            runtime_pstat_series: self
                .conn()
                .query(
                    "select id, benchmark, metric from runtime_pstat_series;",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        (
                            row.get::<_, String>(1).as_str().into(),
                            row.get::<_, String>(2).as_str().into(),
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
    async fn queue_pr(
        &self,
        pr: u32,
        include: Option<&str>,
        exclude: Option<&str>,
        runs: Option<i32>,
    ) {
        if let Err(e) = self.conn()
            .execute(
                "insert into pull_request_build (pr, complete, requested, include, exclude, runs) VALUES ($1, false, CURRENT_TIMESTAMP, $2, $3, $4)",
                &[&(pr as i32), &include, &exclude, &runs],
            )
            .await {
            log::error!("failed to queue_pr({}, {:?}, {:?}, {:?}): {:?}", pr, include, exclude, runs, e);
        }
    }
    async fn pr_attach_commit(
        &self,
        pr: u32,
        sha: &str,
        parent_sha: &str,
        commit_date: Option<DateTime<Utc>>,
    ) -> bool {
        self.conn()
            .execute(
                "update pull_request_build SET bors_sha = $1, parent_sha = $2, commit_date = $4
                where pr = $3 and bors_sha is null",
                &[&sha, &parent_sha, &(pr as i32), &commit_date],
            )
            .await
            .unwrap()
            > 0
    }
    async fn queued_commits(&self) -> Vec<QueuedCommit> {
        let rows = self
            .conn()
            .query(
                "select pr, bors_sha, parent_sha, include, exclude, runs, commit_date from pull_request_build
                where complete is false and bors_sha is not null
                order by requested asc",
                &[],
            )
            .await
            .unwrap();
        rows.into_iter()
            .map(|row| QueuedCommit {
                pr: row.get::<_, i32>(0) as u32,
                sha: row.get(1),
                parent_sha: row.get(2),
                include: row.get(3),
                exclude: row.get(4),
                runs: row.get(5),
                commit_date: row.get::<_, Option<_>>(6).map(Date),
            })
            .collect()
    }
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit> {
        let row = self
            .conn()
            .query_opt(
                "update pull_request_build SET complete = true
                where bors_sha = $1
                returning pr, bors_sha, parent_sha, include, exclude, runs, commit_date",
                &[&sha],
            )
            .await
            .unwrap()?;
        Some(QueuedCommit {
            pr: row.get::<_, i32>(0) as u32,
            sha: row.get(1),
            parent_sha: row.get(2),
            include: row.get(3),
            exclude: row.get(4),
            runs: row.get(5),
            commit_date: row.get::<_, Option<_>>(6).map(Date),
        })
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
        metric: &str,
        stat: f64,
    ) {
        let profile = profile.to_string();
        let scenario = scenario.to_string();
        let backend = backend.to_string();
        let sid = self
            .conn()
            .query_opt(
                &self.statements().select_pstat_series,
                &[&benchmark, &profile, &scenario, &backend, &metric],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => {
                self.conn()
                    .query_opt(
                        &self.statements().insert_pstat_series,
                        &[&benchmark, &profile, &scenario, &backend, &metric],
                    )
                    .await
                    .unwrap();
                self.conn()
                    .query_one(
                        &self.statements().select_pstat_series,
                        &[&benchmark, &profile, &scenario, &backend, &metric],
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
        value: f64,
    ) {
        let sid = self
            .conn()
            .query_opt(
                &self.statements().select_runtime_pstat_series,
                &[&benchmark, &metric],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => {
                self.conn()
                    .query_opt(
                        &self.statements().insert_runtime_pstat_series,
                        &[&benchmark, &metric],
                    )
                    .await
                    .unwrap();
                self.conn()
                    .query_one(
                        &self.statements().select_runtime_pstat_series,
                        &[&benchmark, &metric],
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

    async fn record_error(&self, artifact: ArtifactIdNumber, krate: &str, error: &str) {
        self.conn()
            .execute(
                "insert into error (benchmark, aid, error) VALUES ($1, $2, $3)",
                &[&krate, &(artifact.0 as i32), &error],
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
                where aid = $1 and step = $2 and end_time is null;",
                &[&(aid.0 as i32), &step],
            )
            .await
            .unwrap()
            == 1
    }
    async fn collector_end_step(&self, aid: ArtifactIdNumber, step: &str) {
        let did_modify = self
            .conn()
            .execute(
                "update collector_progress set end_time = statement_timestamp() \
                where aid = $1 and step = $2 and start_time is not null and end_time is null;",
                &[&(aid.0 as i32), &step],
            )
            .await
            .unwrap()
            == 1;
        if !did_modify {
            log::error!("did not end {} for {:?}", step, aid);
        }
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
    async fn in_progress_artifacts(&self) -> Vec<ArtifactId> {
        let rows = self
            .conn()
            .query(
                "select distinct aid from collector_progress where end_time is null order by aid limit 1",
                &[],
            )
            .await
            .unwrap();
        let aids = rows
            .into_iter()
            .map(|row| row.get::<_, i32>(0))
            .collect::<Vec<_>>();

        let mut artifacts = Vec::new();
        for aid in aids {
            let row = self
                .conn()
                .query_one(
                    "select name, date, type from artifact where id = $1",
                    &[&aid],
                )
                .await;

            let row = match row {
                Ok(row) => row,
                Err(err) => {
                    log::error!("skipping aid={} -- no such artifact: {:?}", aid, err);
                    continue;
                }
            };

            let ty = row.get::<_, String>(2);
            artifacts.push(match ty.as_str() {
                "try" | "master" => ArtifactId::Commit(Commit {
                    sha: row.get(0),
                    date: row
                        .get::<_, Option<_>>(1)
                        .map(Date)
                        .unwrap_or_else(|| Date::ymd_hms(2001, 1, 1, 0, 0, 0)),
                    r#type: CommitType::from_str(&ty).unwrap(),
                }),
                "release" => ArtifactId::Tag(row.get(0)),
                _ => {
                    log::error!("unknown ty {:?}", ty);
                    continue;
                }
            });
        }
        artifacts
    }
    async fn in_progress_steps(&self, artifact: &ArtifactId) -> Vec<crate::Step> {
        let aid = self.artifact_id(artifact).await;

        let steps = self
            .conn()
            .query(&self.statements().in_progress_steps, &[&(aid.0 as i32)])
            .await
            .unwrap();

        steps
            .into_iter()
            .map(|row| crate::Step {
                name: row.get(0),
                is_done: row.get(1),
                duration: Duration::from_secs(row.get::<_, Option<i32>>(2).unwrap_or(0) as u64),
                expected: Duration::from_secs(row.get::<_, Option<i32>>(3).unwrap_or(0) as u64),
            })
            .collect()
    }
    async fn last_n_artifact_collections(&self, n: u32) -> Vec<ArtifactCollection> {
        self.conn()
            .query(
                "select art.name, art.date, art.type, acd.date_recorded, acd.duration \
                from artifact_collection_duration as acd \
                join artifact as art on art.id = acd.aid \
                order by date_recorded desc \
                limit $1;",
                &[&(n as i64)],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|r| {
                let sha = r.get::<_, String>(0);
                let date = r.get::<_, Option<DateTime<Utc>>>(1);
                let ty = r.get::<_, String>(2);

                ArtifactCollection {
                    artifact: parse_artifact_id(&ty, &sha, date),
                    end_time: r.get(3),
                    duration: Duration::from_secs(r.get::<_, i32>(4) as u64),
                }
            })
            .collect()
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
            .query_opt(
                "select pr from pull_request_build where bors_sha = $1",
                &[&sha],
            )
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
            .execute("delete from artifact where name = $1", &[&info.name])
            .await
            .unwrap();
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
        _ => panic!("unknown artifact type: {:?}", ty),
    }
}

#[cfg(test)]
mod tests {
    use super::make_certificates;

    // Makes sure we successfully parse the RDS certificates and load them into native-tls compatible
    // format.
    #[test]
    fn can_make_certificates() {
        let certs = make_certificates();
        assert!(!certs.is_empty());
    }
}
