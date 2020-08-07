use crate::pool::{Connection, ConnectionManager, ManagedConnection, Transaction};
use crate::{
    ArtifactId, ArtifactIdNumber, Cache, CollectionId, Commit, Crate, Date, Index, Profile,
    QueuedCommit,
};
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
    get_self_profile: Statement,
    insert_self_profile_query: Statement,
    select_self_query_series: Statement,
    insert_self_query_series: Statement,
    insert_pstat_series: Statement,
    select_pstat_series: Statement,
    get_error: Statement,
    collection_id: Statement,
    record_duration: Statement,
    in_progress_steps: Statement,
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
                insert_pstat: conn
                    .prepare("insert into pstat (series, aid, cid, value) VALUES ($1, $2, $3, $4)")
                    .await
                    .unwrap(),
                get_self_profile_query: conn
                    .prepare(
                        "select
                        self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count
                        from self_profile_query
                        where series = $1 and aid = $2 order by self_time asc;
                        ",
                    )
                    .await
                    .unwrap(),
                get_self_profile: conn.prepare("
                    select
                        query, self_time, blocked_time, incremental_load_time, number_of_cache_hits, invocation_count
                    from self_profile_query_series
                    join self_profile_query on self_profile_query_series.id = self_profile_query.series
                    where
                        crate = $1
                        and profile = $2
                        and cache = $3
                        and aid = $4
                ").await.unwrap(),
                insert_self_profile_query: conn
                    .prepare(
                        "insert into self_profile_query(
                            series,
                            aid,
                            cid,
                            self_time,
                            blocked_time,
                            incremental_load_time,
                            number_of_cache_hits,
                            invocation_count
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    )
                    .await
                    .unwrap(),
                get_error: conn.prepare("select crate, error from error_series
                    left join error on error.series = error_series.id and aid = $1").await.unwrap(),
                select_self_query_series: conn.prepare("select id from self_profile_query_series where crate = $1 and profile = $2 and cache = $3 and query = $4").await.unwrap(),
                insert_self_query_series: conn.prepare("insert into self_profile_query_series (crate, profile, cache, query) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id").await.unwrap(),
                insert_pstat_series: conn.prepare("insert into pstat_series (crate, profile, cache, statistic) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING RETURNING id").await.unwrap(),
                select_pstat_series: conn.prepare("select id from pstat_series where crate = $1 and profile = $2 and cache = $3 and statistic = $4").await.unwrap(),
                collection_id: conn.prepare("insert into collection (perf_commit) VALUES ($1) returning id").await.unwrap(),
                record_duration: conn.prepare("
                    insert into artifact_collection_duration (
                        aid,
                        date_recorded,
                        duration
                    ) VALUES ($1, CURRENT_TIMESTAMP, $2)
                ").await.unwrap(),
                in_progress_steps: conn.prepare("
                select step,
                    end_time is not null,
                    extract(epoch from interval '0 seconds'::interval +
                        coalesce(end_time, statement_timestamp()) - start_time)::int4,
                    extract(
                        epoch from interval '0 seconds'::interval +
                        (select end_time - start_time
                        from collector_progress as cp
                            where
                                cp.aid != $1
                                and cp.step = collector_progress.step
                                and cp.start_time is not null
                                and cp.end_time is not null
                            order by start_time desc
                            limit 1
                        ))::int4
                from collector_progress where aid = $1 order by step
                ").await.unwrap(),
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
                &[&(artifact.0 as i16), &(duration.as_secs() as i32)],
            )
            .await
            .unwrap();
    }

    async fn load_index(&mut self) -> Index {
        Index {
            commits: self
                .conn()
                .query(
                    "select id, name, date from artifact where type = 'master' or type = 'try'",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i16>(0) as u32,
                        Commit {
                            sha: row.get::<_, String>(1).as_str().into(),
                            date: {
                                let timestamp: Option<DateTime<Utc>> = row.get(2);
                                match timestamp {
                                    Some(t) => Date(t),
                                    None => Date(Utc.ymd(2001, 01, 01).and_hms(0, 0, 0)),
                                }
                            },
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
                        row.get::<_, i16>(0) as u32,
                        row.get::<_, String>(1).as_str().into(),
                    )
                })
                .collect(),
            errors: self
                .conn()
                .query("select id, crate from error_series", &[])
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
            pstats: self
                .conn()
                .query(
                    "select id, crate, profile, cache, statistic from pstat_series;",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        (
                            Crate::from(row.get::<_, String>(1).as_str()),
                            match row.get::<_, String>(2).as_str() {
                                "check" => Profile::Check,
                                "opt" => Profile::Opt,
                                "debug" => Profile::Debug,
                                "doc" => Profile::Doc,
                                o => unreachable!("{}: not a profile", o),
                            },
                            row.get::<_, String>(3).as_str().parse().unwrap(),
                            row.get::<_, String>(4).as_str().into(),
                        ),
                    )
                })
                .collect(),
            queries: self
                .conn()
                .query(
                    "select id, crate, profile, cache, query from self_profile_query_series;",
                    &[],
                )
                .await
                .unwrap()
                .into_iter()
                .map(|row| {
                    (
                        row.get::<_, i32>(0) as u32,
                        (
                            Crate::from(row.get::<_, String>(1).as_str()),
                            match row.get::<_, String>(2).as_str() {
                                "check" => Profile::Check,
                                "opt" => Profile::Opt,
                                "debug" => Profile::Debug,
                                "doc" => Profile::Doc,
                                o => unreachable!("{}: not a profile", o),
                            },
                            row.get::<_, String>(3).as_str().parse().unwrap(),
                            row.get::<_, String>(4).as_str().into(),
                        ),
                    )
                })
                .collect(),
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
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: crate::ArtifactIdNumber,
    ) -> Option<crate::QueryDatum> {
        let row = self
            .conn()
            .query_opt(
                &self.statements().get_self_profile_query,
                &[&(series as i32), &(cid.0 as i16)],
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
    async fn get_self_profile(
        &self,
        cid: ArtifactIdNumber,
        crate_: &str,
        profile: &str,
        cache: &str,
    ) -> HashMap<crate::QueryLabel, crate::QueryDatum> {
        let rows = self
            .conn()
            .query(
                &self.statements().get_self_profile,
                &[&crate_, &profile, &cache, &(cid.0 as i16)],
            )
            .await
            .unwrap();

        rows.into_iter()
            .map(|r| {
                let self_time: i64 = r.get(1);
                let blocked_time: i64 = r.get(2);
                let incremental_load_time: i64 = r.get(3);
                (
                    r.get::<_, &str>(0).into(),
                    crate::QueryDatum {
                        self_time: Duration::from_nanos(self_time as u64),
                        blocked_time: Duration::from_nanos(blocked_time as u64),
                        incremental_load_time: Duration::from_nanos(incremental_load_time as u64),
                        number_of_cache_hits: r.get::<_, i32>(4) as u32,
                        invocation_count: r.get::<_, i32>(5) as u32,
                    },
                )
            })
            .collect()
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
    async fn queue_pr(&self, pr: u32, include: Option<&str>, exclude: Option<&str>) {
        self.conn()
            .execute(
                "insert into pull_request_build (pr, complete, requested, include, exclude) VALUES ($1, false, CURRENT_TIMESTAMP, $2, $3) ON CONFLICT DO NOTHING",
                &[&(pr as i32), &include, &exclude],
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
                "select pr, bors_sha, parent_sha, include, exclude from pull_request_build
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
            })
            .collect()
    }
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit> {
        let row = self
            .conn()
            .query_opt(
                "update pull_request_build SET complete = true
                where bors_sha = $1
                returning pr, bors_sha, parent_sha, include, exclude",
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
        krate: &str,
        profile: Profile,
        cache: Cache,
        statistic: &str,
        value: f64,
    ) {
        let profile = profile.to_string();
        let cache = cache.to_string();
        let sid = self
            .conn()
            .query_opt(
                &self.statements().insert_pstat_series,
                &[&krate, &profile, &cache, &statistic],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => self
                .conn()
                .query_one(
                    &self.statements().select_pstat_series,
                    &[&krate, &profile, &cache, &statistic],
                )
                .await
                .unwrap()
                .get(0),
        };
        self.conn()
            .execute(
                &self.statements().insert_pstat,
                &[&sid, &(artifact.0 as i16), &(collection.0 as i32), &value],
            )
            .await
            .unwrap();
    }

    async fn artifact_id(&self, artifact: &ArtifactId) -> ArtifactIdNumber {
        let (name, date, ty) = match artifact {
            ArtifactId::Commit(commit) => (
                commit.sha.to_string(),
                if commit.is_try() {
                    None
                } else {
                    Some(commit.date.0)
                },
                if commit.is_try() { "try" } else { "master" },
            ),
            ArtifactId::Artifact(a) => (a.clone(), None, "release"),
        };

        let aid = self.conn()
            .query_opt("insert into artifact (name, date, type) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING id", &[
                &name,
                &date,
                &ty,
            ])
            .await
            .unwrap();
        if let Some(row) = aid {
            return ArtifactIdNumber(row.get::<_, i16>(0) as u32);
        }
        ArtifactIdNumber(
            self.conn()
                .query_one("select id from artifact where name = $1", &[&name])
                .await
                .unwrap()
                .get::<_, i16>(0) as u32,
        )
    }

    async fn record_self_profile_query(
        &self,
        collection: CollectionId,
        artifact: ArtifactIdNumber,
        krate: &str,
        profile: Profile,
        cache: Cache,
        query: &str,
        qd: crate::QueryDatum,
    ) {
        let profile = profile.to_string();
        let cache = cache.to_string();
        let sid = self
            .conn()
            .query_opt(
                &self.statements().insert_self_query_series,
                &[&krate, &profile, &cache, &query],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => self
                .conn()
                .query_one(
                    &self.statements().select_self_query_series,
                    &[&krate, &profile, &cache, &query],
                )
                .await
                .unwrap()
                .get(0),
        };
        self.conn()
            .execute(
                &self.statements().insert_self_profile_query,
                &[
                    &(sid as i32),
                    &(artifact.0 as i16),
                    &(collection.0 as i32),
                    &i64::try_from(qd.self_time.as_nanos()).unwrap(),
                    &i64::try_from(qd.blocked_time.as_nanos()).unwrap(),
                    &i64::try_from(qd.incremental_load_time.as_nanos()).unwrap(),
                    &(qd.number_of_cache_hits as i32),
                    &(qd.invocation_count as i32),
                ],
            )
            .await
            .unwrap();
    }

    async fn record_error(&self, artifact: ArtifactIdNumber, krate: &str, error: &str) {
        let sid = self
            .conn()
            .query_opt(
                "insert into error_series (crate) VALUES ($1) ON CONFLICT DO NOTHING RETURNING id",
                &[&krate],
            )
            .await
            .unwrap();
        let sid: i32 = match sid {
            Some(id) => id.get(0),
            None => self
                .conn()
                .query_one("select id from error_series where crate = $1", &[&krate])
                .await
                .unwrap()
                .get(0),
        };
        self.conn()
            .execute(
                "insert into error (series, aid, error) VALUES ($1, $2, $3)",
                &[&sid, &(artifact.0 as i16), &error],
            )
            .await
            .unwrap();
    }
    async fn record_benchmark(&self, krate: &str, supports_stable: bool) {
        if let Some(r) = self
            .conn()
            .query_opt(
                "select stabilized from benchmark where name = $1",
                &[&krate],
            )
            .await
            .unwrap()
        {
            if r.get::<_, bool>(0) == supports_stable {
                return;
            }
        }
        self.conn()
            .execute(
                "insert into benchmark (name, stabilized) VALUES ($1, $2)
                ON CONFLICT (name) DO UPDATE SET stabilized = EXCLUDED.stabilized",
                &[&krate, &supports_stable],
            )
            .await
            .unwrap();
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
                    &[&(aid.0 as i16), &step],
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
                &[&(aid.0 as i16), &step],
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
                &[&(aid.0 as i16), &step],
            )
            .await
            .unwrap()
            == 1;
        if !did_modify {
            log::error!("did not end {} for {:?}", step, aid);
        }
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
            .map(|row| row.get::<_, i16>(0))
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
                        .unwrap_or_else(|| Date::ymd_hms(2001, 01, 01, 0, 0, 0)),
                }),
                "release" => ArtifactId::Artifact(row.get(0)),
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
            .query(&self.statements().in_progress_steps, &[&(aid.0 as i16)])
            .await
            .unwrap();

        steps
            .into_iter()
            .map(|row| crate::Step {
                name: row.get(0),
                is_done: row.get(1),
                duration: Duration::from_secs(row.get::<_, Option<i32>>(2).unwrap_or(0) as u64),
                expected: Duration::from_secs(row.get::<_, i32>(3) as u64),
            })
            .collect()
    }
    async fn last_end_time(&self) -> Option<DateTime<Utc>> {
        self.conn()
            .query_opt(
                "select date_recorded + (duration || 'seconds')::interval \
                from artifact_collection_duration \
                order by date_recorded desc \
                limit 1;",
                &[],
            )
            .await
            .unwrap()
            .map(|r| r.get(0))
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
        krate: &str,
        profile: Profile,
        cache: Cache,
    ) {
        let profile = profile.to_string();
        let cache = cache.to_string();
        self.conn().execute(
            "insert into raw_self_profile (aid, cid, crate, profile, cache) VALUES ($1, $2, $3, $4, $5)",
            &[&(artifact.0 as i16), &collection.0, &krate, &profile, &cache],
        ).await.unwrap();
    }
    async fn list_self_profile(
        &self,
        aid: ArtifactId,
        crate_: &str,
        profile: &str,
        cache: &str,
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
                    &cache,
                    &match aid {
                        ArtifactId::Commit(c) => c.sha,
                        ArtifactId::Artifact(a) => a,
                    },
                ],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|r| (ArtifactIdNumber(r.get::<_, i16>(0) as u32), r.get(1)))
            .collect()
    }
}
