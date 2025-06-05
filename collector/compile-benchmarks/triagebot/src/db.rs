use crate::{db::jobs::*, handlers::Context, jobs::jobs};
use anyhow::Context as _;
use chrono::Utc;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::sync::{Arc, LazyLock, Mutex};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio_postgres::Client as DbClient;

pub mod issue_data;
pub mod jobs;
pub mod notifications;
pub mod review_prefs;
pub mod rustc_commits;
pub mod users;

const CERT_URL: &str = "https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem";

static CERTIFICATE_PEMS: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(CERT_URL).send().expect("failed to get RDS cert");
    resp.bytes().expect("failed to get RDS cert body").to_vec()
});

pub struct ClientPool {
    connections: Arc<Mutex<Vec<tokio_postgres::Client>>>,
    permits: Arc<Semaphore>,
    db_url: String,
}

pub struct PooledClient {
    client: Option<tokio_postgres::Client>,
    #[allow(unused)] // only used for drop impl
    permit: OwnedSemaphorePermit,
    pool: Arc<Mutex<Vec<tokio_postgres::Client>>>,
}

impl Drop for PooledClient {
    fn drop(&mut self) {
        let mut clients = self.pool.lock().unwrap_or_else(|e| e.into_inner());
        clients.push(self.client.take().unwrap());
    }
}

impl std::ops::Deref for PooledClient {
    type Target = tokio_postgres::Client;

    fn deref(&self) -> &Self::Target {
        self.client.as_ref().unwrap()
    }
}

impl std::ops::DerefMut for PooledClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.client.as_mut().unwrap()
    }
}

impl ClientPool {
    pub fn new(db_url: String) -> ClientPool {
        ClientPool {
            connections: Arc::new(Mutex::new(Vec::with_capacity(16))),
            permits: Arc::new(Semaphore::new(16)),
            db_url,
        }
    }

    pub async fn get(&self) -> PooledClient {
        let permit = self.permits.clone().acquire_owned().await.unwrap();
        {
            let mut slots = self.connections.lock().unwrap_or_else(|e| e.into_inner());
            // Pop connections until we hit a non-closed connection (or there are no
            // "possibly open" connections left).
            while let Some(c) = slots.pop() {
                if !c.is_closed() {
                    return PooledClient {
                        client: Some(c),
                        permit,
                        pool: self.connections.clone(),
                    };
                }
            }
        }

        PooledClient {
            client: Some(make_client(&self.db_url).await.unwrap()),
            permit,
            pool: self.connections.clone(),
        }
    }
}

pub async fn make_client(db_url: &str) -> anyhow::Result<tokio_postgres::Client> {
    if db_url.contains("rds.amazonaws.com") {
        let mut builder = TlsConnector::builder();
        for cert in make_certificates() {
            builder.add_root_certificate(cert);
        }
        let connector = builder.build().context("built TlsConnector")?;
        let connector = MakeTlsConnector::new(connector);

        let (db_client, connection) = match tokio_postgres::connect(&db_url, connector).await {
            Ok(v) => v,
            Err(e) => {
                anyhow::bail!("failed to connect to DB: {}", e);
            }
        };
        tokio::task::spawn(async move {
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

fn make_certificates() -> Vec<Certificate> {
    use x509_cert::der::pem::LineEnding;
    use x509_cert::der::EncodePem;

    let certs = x509_cert::Certificate::load_pem_chain(&CERTIFICATE_PEMS[..]).unwrap();
    certs
        .into_iter()
        .map(|cert| Certificate::from_pem(cert.to_pem(LineEnding::LF).unwrap().as_bytes()).unwrap())
        .collect()
}

// Makes sure we successfully parse the RDS certificates and load them into native-tls compatible
// format.
#[test]
fn cert() {
    make_certificates();
}

pub async fn run_migrations(client: &mut DbClient) -> anyhow::Result<()> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS database_versions (
                zero INTEGER PRIMARY KEY,
                migration_counter INTEGER
            );",
            &[],
        )
        .await
        .context("creating database versioning table")?;

    client
        .execute(
            "INSERT INTO database_versions (zero, migration_counter)
        VALUES (0, 0)
        ON CONFLICT DO NOTHING",
            &[],
        )
        .await
        .context("inserting initial database_versions")?;

    let migration_idx: i32 = client
        .query_one("SELECT migration_counter FROM database_versions", &[])
        .await
        .context("getting migration counter")?
        .get(0);
    let migration_idx = migration_idx as usize;

    for (idx, migration) in MIGRATIONS.iter().enumerate() {
        if idx >= migration_idx {
            let tx = client
                .transaction()
                .await
                .context("Cannot create migration transactin")?;
            tx.execute(*migration, &[])
                .await
                .with_context(|| format!("executing {}th migration", idx))?;
            tx.execute(
                "UPDATE database_versions SET migration_counter = $1",
                &[&(idx as i32 + 1)],
            )
            .await
            .with_context(|| format!("updating migration counter to {}", idx))?;
            tx.commit()
                .await
                .context("Cannot commit migration transaction")?;
        }
    }

    Ok(())
}

pub async fn schedule_jobs(db: &DbClient, jobs: Vec<JobSchedule>) -> anyhow::Result<()> {
    for job in jobs {
        let mut upcoming = job.schedule.upcoming(Utc).take(1);

        if let Some(scheduled_at) = upcoming.next() {
            schedule_job(db, job.name, job.metadata, scheduled_at).await?;
        }
    }

    Ok(())
}

pub async fn schedule_job(
    db: &DbClient,
    job_name: &str,
    job_metadata: serde_json::Value,
    when: chrono::DateTime<Utc>,
) -> anyhow::Result<()> {
    let all_jobs = jobs();
    if !all_jobs.iter().any(|j| j.name() == job_name) {
        anyhow::bail!("Job {} does not exist in the current job list.", job_name);
    }

    if let Err(_) = get_job_by_name_and_scheduled_at(&db, job_name, &when).await {
        // mean there's no job already in the db with that name and scheduled_at
        insert_job(&db, job_name, &when, &job_metadata).await?;
    }

    Ok(())
}

pub async fn run_scheduled_jobs(ctx: &Context) -> anyhow::Result<()> {
    let db = &ctx.db.get().await;
    let jobs = get_jobs_to_execute(&db).await?;
    tracing::trace!("jobs to execute: {:#?}", jobs);

    for job in jobs.iter() {
        update_job_executed_at(&db, &job.id).await?;

        match handle_job(&ctx, &job.name, &job.metadata).await {
            Ok(_) => {
                tracing::trace!("job successfully executed (id={})", job.id);
                delete_job(&db, &job.id).await?;
            }
            Err(e) => {
                tracing::error!("job failed on execution (id={:?}, error={:?})", job.id, e);
                update_job_error_message(&db, &job.id, &e.to_string()).await?;
            }
        }
    }

    Ok(())
}

// Try to handle a specific job
async fn handle_job(
    ctx: &Context,
    name: &String,
    metadata: &serde_json::Value,
) -> anyhow::Result<()> {
    for job in jobs() {
        if &job.name() == &name {
            return job.run(ctx, metadata).await;
        }
    }
    tracing::trace!(
        "handle_job fell into default case: (name={:?}, metadata={:?})",
        name,
        metadata
    );

    Ok(())
}

// Important notes when adding migrations:
// - Each DB change is an element in this array and must be a single SQL instruction
// - The total # of items in this array must be equal to the value of `database_versions.migration_counter`
static MIGRATIONS: &[&str] = &[
    "
CREATE TABLE notifications (
    notification_id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    origin_url TEXT NOT NULL,
    origin_html TEXT,
    time TIMESTAMP WITH TIME ZONE
);
",
    "
CREATE TABLE users (
    user_id BIGINT PRIMARY KEY,
    username TEXT NOT NULL
);
",
    "ALTER TABLE notifications ADD COLUMN short_description TEXT;",
    "ALTER TABLE notifications ADD COLUMN team_name TEXT;",
    "ALTER TABLE notifications ADD COLUMN idx INTEGER;",
    "ALTER TABLE notifications ADD COLUMN metadata TEXT;",
    "
CREATE TABLE rustc_commits (
    sha TEXT PRIMARY KEY,
    parent_sha TEXT NOT NULL,
    time TIMESTAMP WITH TIME ZONE
);
",
    "ALTER TABLE rustc_commits ADD COLUMN pr INTEGER;",
    "
CREATE TABLE issue_data (
    repo TEXT,
    issue_number INTEGER,
    key TEXT,
    data JSONB,
    PRIMARY KEY (repo, issue_number, key)
);
",
    "
CREATE TABLE jobs (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT NOT NULL,
    scheduled_at TIMESTAMP WITH TIME ZONE NOT NULL,
    metadata JSONB,
    executed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT
);
",
    "
CREATE UNIQUE INDEX jobs_name_scheduled_at_unique_index
    ON jobs (
        name, scheduled_at
    );
",
    "
CREATE table review_prefs (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id BIGINT REFERENCES users(user_id),
    assigned_prs INT[] NOT NULL DEFAULT array[]::INT[]
);",
    "
CREATE EXTENSION IF NOT EXISTS intarray;",
    "
CREATE UNIQUE INDEX IF NOT EXISTS review_prefs_user_id ON review_prefs(user_id);
 ",
    "
ALTER TABLE review_prefs ADD COLUMN IF NOT EXISTS max_assigned_prs INTEGER DEFAULT NULL;
",
    "
ALTER TABLE review_prefs ADD COLUMN IF NOT EXISTS rotation_mode TEXT NOT NULL DEFAULT 'on-rotation';
",
];
