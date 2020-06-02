use crate::pool::{Connection, ConnectionManager, ManagedConnection, Transaction};
use crate::{Commit, Crate, Date, Index, Profile, QueuedCommit};
use anyhow::Context as _;
use chrono::{DateTime, TimeZone, Utc};
use hashbrown::HashMap;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::convert::TryFrom;
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

const CERT_URL: &str = "https://s3.amazonaws.com/rds-downloads/rds-ca-2019-root.pem";

lazy_static::lazy_static! {
    static ref CERTIFICATE_PEM: Vec<u8> = {
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
        let cert = &CERTIFICATE_PEM[..];
        let cert = Certificate::from_pem(&cert).context("made certificate")?;
        let connector = TlsConnector::builder()
            .add_root_certificate(cert)
            .build()
            .context("built TlsConnector")?;
        let connector = MakeTlsConnector::new(connector);

        let (db_client, connection) = match tokio_postgres::connect(&db_url, connector).await {
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
            match tokio_postgres::connect(&db_url, tokio_postgres::NoTls).await {
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
            for mid in (version.unwrap_or(0) as usize + 1)..MIGRATIONS.len() {
                let sql = MIGRATIONS[mid];
                let tx = client.transaction().await.unwrap();
                tx.batch_execute(&sql).await.unwrap();
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
    insert_pstat: Statement,
    get_self_profile_query: Statement,
    insert_self_profile_query: Statement,
    get_error: Statement,
    insert_error: Statement,
}

pub struct PostgresTransaction<'a> {
    statements: Arc<CachedStatements>,
    conn: tokio_postgres::Transaction<'a>,
}

pub struct PostgresConnection {
    statements: Arc<CachedStatements>,
    conn: tokio_postgres::Client,
}

impl Into<tokio_postgres::Client> for PostgresConnection {
    fn into(self) -> tokio_postgres::Client {
        self.conn
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
        &(&**self).conn
    }
    fn conn_mut(&mut self) -> &mut Self::Client {
        &mut (&mut **self).conn
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
                         WITH cids AS (
                             select cid, num from unnest($2::int[]) with ordinality cids(cid, num)
                         ),
                         sids AS (
                             select sid, idx from unnest($1::int[]) with ordinality sids(sid, idx)
                         )
                         select ARRAY(
                             (
                                 select min(pstat.value) from cids
                                     left outer join pstat
                                     on (cids.cid = pstat.cid and pstat.series = sids.sid)
                                     group by cids.num
                                     order by cids.num
                             )
                         ) from
                         sids
                         group by (sids.idx, sids.sid)
                         order by sids.idx
                     ")
                    .await
                    .unwrap(),
                insert_pstat: conn
                    .prepare("insert into pstat(series, cid, value) VALUES ($1, $2, $3)")
                    .await
                    .unwrap(),
                get_self_profile_query: conn
                    .prepare(
                        "select
                        self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count
                        from self_profile_query
                        where series = $1 and cid = $2 order by self_time asc;
                        ",
                    )
                    .await
                    .unwrap(),
                insert_self_profile_query: conn
                    .prepare(
                        "insert into self_profile_query(
                            series,
                            cid,
                            self_time,
                            blocked_time,
                            incremental_load_time,
                            number_of_cache_hits,
                            invocation_count
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                    )
                    .await
                    .unwrap(),
                get_error: conn.prepare("select crate, error from error_series
                    left join error on error.series = error_series.id and aid = $1").await.unwrap(),
                insert_error: conn.prepare("select 1;").await.unwrap(),
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
    async fn load_index(&mut self) -> Index {
        Index {
            commits: self.conn().query("select id, name, date from artifact where type = 'master' or type = 'try' order by date", &[]).await.unwrap().into_iter().map(|row| {
                (row.get::<_, i16>(0) as u32, Commit {
                    sha: row.get::<_, String>(1).as_str().into(),
                    date: {
                        let timestamp: Option<DateTime<Utc>> = row.get(2);
                        match timestamp {
                            Some(t) => Date(t),
                            None => Date(Utc.ymd(2001, 01, 01).and_hms(0,0,0)),
                        }
                    }
                })
            }).collect(),
            artifacts: self.conn().query("select id, name from artifact where type = 'release'", &[]).await.unwrap().into_iter().map(|row| {
                (row.get::<_, i16>(0) as u32, row.get::<_, String>(1).as_str().into())
            }).collect(),
            errors: self.conn().query("select id, crate from error_series", &[]).await.unwrap().into_iter().map(|row| {
                (row.get::<_, i32>(0) as u32, row.get::<_, String>(1).as_str().into())
            }).collect(),
            pstats: self.conn().query("select id, crate, profile, cache, statistic from pstat_series;", &[]).await.unwrap().into_iter().map(|row| {
                (row.get::<_, i32>(0) as u32, (
                    Crate::from(row.get::<_, String>(1).as_str()),
                    match row.get::<_, String>(2).as_str() {
                        "check" => Profile::Check,
                        "opt" => Profile::Opt,
                        "debug" => Profile::Debug,
                        o => unreachable!("{}: not a profile", o),
                    },
                    row.get::<_, String>(3).as_str().parse().unwrap(),
                    row.get::<_, String>(4).as_str().into(),
                ))
            }).collect(),
            queries: self.conn().query("select id, crate, profile, cache, query from self_profile_query_series;", &[]).await.unwrap().into_iter().map(|row| {
                (row.get::<_, i32>(0) as u32, (
                    Crate::from(row.get::<_, String>(1).as_str()),
                    match row.get::<_, String>(2).as_str() {
                        "check" => Profile::Check,
                        "opt" => Profile::Opt,
                        "debug" => Profile::Debug,
                        o => unreachable!("{}: not a profile", o),
                    },
                    row.get::<_, String>(3).as_str().parse().unwrap(),
                    row.get::<_, String>(4).as_str().into(),
                ))
            }).collect(),
        }
    }
    async fn get_pstats(
        &self,
        series: &[u32],
        cids: &[Option<crate::ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let series = series.iter().map(|sid| *sid as i32).collect::<Vec<_>>();
        let cids = cids
            .iter()
            .map(|id| id.map(|id| id.0 as i32))
            .collect::<Vec<_>>();
        let rows = self
            .conn()
            .query(&self.statements().get_pstat, &[&series, &cids])
            .await
            .unwrap();
        rows.into_iter()
            .map(|row| row.get::<_, Vec<Option<f64>>>(0))
            .collect()
    }
    async fn insert_pstat(&self, series: u32, cid: crate::ArtifactIdNumber, stat: f64) {
        self.conn()
            .execute(
                &self.statements().insert_pstat,
                &[&(series as i32), &(cid.0 as i32), &stat],
            )
            .await
            .unwrap();
    }
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: crate::ArtifactIdNumber,
    ) -> Option<crate::QueryDatum> {
        let row = self
            .conn()
            .query_opt(
                &self.statements().get_self_profile_query,
                &[&(series as i32), &(cid.0 as i32)],
            )
            .await
            .unwrap()?;
        let self_time: i64 = row.get(0);
        let blocked_time: i64 = row.get(1);
        let incremental_load_time: i64 = row.get(2);
        Some(crate::QueryDatum {
            self_time: Duration::from_nanos(self_time as u64),
            blocked_time: Duration::from_nanos(blocked_time as u64),
            incremental_load_time: Duration::from_nanos(incremental_load_time as u64),
            number_of_cache_hits: row.get::<_, i32>(3) as u32,
            invocation_count: row.get::<_, i32>(4) as u32,
        })
    }
    async fn insert_self_profile_query(
        &self,
        series: u32,
        cid: crate::ArtifactIdNumber,
        data: crate::QueryDatum,
    ) {
        self.conn()
            .execute(
                &self.statements().insert_self_profile_query,
                &[
                    &(series as i32),
                    &(cid.0 as i32),
                    &i64::try_from(data.self_time.as_nanos()).unwrap(),
                    &i64::try_from(data.blocked_time.as_nanos()).unwrap(),
                    &i64::try_from(data.incremental_load_time.as_nanos()).unwrap(),
                    &(data.number_of_cache_hits as i32),
                    &(data.invocation_count as i32),
                ],
            )
            .await
            .unwrap();
    }
    async fn get_error(&self, cid: crate::ArtifactIdNumber) -> HashMap<String, Option<String>> {
        let rows = self
            .conn()
            .query(&self.statements().get_error, &[&(cid.0 as i16)])
            .await
            .unwrap();
        rows.into_iter()
            .map(|row| (row.get(0), row.get(1)))
            .collect()
    }
    async fn insert_error(&self, series: u32, cid: crate::ArtifactIdNumber, text: String) {
        self.conn()
            .execute(
                &self.statements().insert_error,
                &[&(series as i32), &(cid.0 as i32), &text],
            )
            .await
            .unwrap();
    }
    async fn queue_pr(&self, pr: u32) {
        self.conn()
            .execute(
                "insert into pull_request_build (pr, requested) VALUES ($1, CURRENT_TIMESTAMP)",
                &[&(pr as i32)],
            )
            .await
            .unwrap();
    }
    async fn pr_attach_commit(&self, pr: u32, sha: &str, parent_sha: &str) -> bool {
        self.conn()
            .execute(
                "update pull_request_build SET bors_sha = $1, parent_sha = $2
                where pr = $3 and bors_sha is null",
                &[&sha, &parent_sha, &(pr as i32)],
            )
            .await
            .unwrap()
            > 0
    }
    async fn queued_commits(&self) -> Vec<QueuedCommit> {
        let rows = self
            .conn()
            .query(
                "select pr, bors_sha, parent_sha from pull_request_build
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
            })
            .collect()
    }
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit> {
        let row = self
            .conn()
            .query_opt(
                "update pull_request_build SET complete = true
                where sha = $1
                returning pr, bors_sha, parent_sha",
                &[&sha],
            )
            .await
            .unwrap()?;
        Some(QueuedCommit {
            pr: row.get::<_, i32>(0) as u32,
            sha: row.get(1),
            parent_sha: row.get(2),
        })
    }
}
