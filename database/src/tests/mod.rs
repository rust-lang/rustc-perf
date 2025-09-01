#![allow(dead_code)]

pub mod builder;

use chrono::Utc;
use std::future::Future;
use tokio_postgres::config::Host;
use tokio_postgres::Config;

use crate::pool::postgres::make_client;
use crate::tests::builder::CollectorBuilder;
use crate::{
    ArtifactId, ArtifactIdNumber, BenchmarkRequest, CollectorConfig, Commit, CommitType,
    Connection, Date, Pool,
};

enum TestDb {
    Postgres {
        original_db_url: String,
        db_name: String,
    },
    SQLite,
}

/// Represents a connection to a Postgres database that can be
/// used in integration tests to test logic that interacts with
/// a database.
pub struct TestContext {
    test_db: TestDb,
    pool: Pool,
    // Pre-cached DB connection
    connection: Box<dyn Connection>,
}

/// Basic lifecycle functions
impl TestContext {
    async fn new_postgres(db_url: &str) -> Self {
        let config: Config = db_url.parse().expect("Cannot parse connection string");

        // Create a new database that will be used for this specific test
        let client = make_client(db_url)
            .await
            .expect("Cannot connect to database");
        let db_name = format!("db{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        client
            .execute(&format!("CREATE DATABASE {db_name}"), &[])
            .await
            .expect("Cannot create database");
        drop(client);

        let host = match &config
            .get_hosts()
            .first()
            .expect("connection string must contain at least one host")
        {
            Host::Tcp(host) => host,

            // This variant only exists on Unix targets, so the arm itself is
            // cfg-gated to keep non-unix builds happy.
            #[cfg(unix)]
            Host::Unix(_) => panic!("Unix sockets in Postgres connection string are not supported"),
        };

        // We need to connect to the database against, because Postgres doesn't allow
        // changing the active database mid-connection.
        // There does not seem to be a way to turn the config back into a connection
        // string, so construct it manually.
        let test_db_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.get_user().unwrap(),
            String::from_utf8(config.get_password().unwrap().to_vec()).unwrap(),
            host,
            &config.get_ports()[0],
            db_name
        );
        let pool = Pool::open(test_db_url.as_str());
        let connection = pool.connection().await;

        Self {
            test_db: TestDb::Postgres {
                original_db_url: db_url.to_string(),
                db_name,
            },
            pool,
            connection,
        }
    }

    async fn new_sqlite() -> Self {
        let pool = Pool::open(":memory:");
        let connection = pool.connection().await;
        Self {
            test_db: TestDb::SQLite,
            pool,
            connection,
        }
    }

    pub fn db(&self) -> &dyn Connection {
        self.connection.as_ref()
    }

    async fn finish(self) {
        // Cleanup the test database
        // First, we need to stop using the database
        drop(self.connection);
        drop(self.pool);

        match self.test_db {
            TestDb::Postgres {
                original_db_url,
                db_name,
            } => {
                // Then we need to connect to the default database and drop our test DB
                let client = make_client(&original_db_url)
                    .await
                    .expect("Cannot connect to database");
                client
                    .execute(&format!("DROP DATABASE {db_name}"), &[])
                    .await
                    .unwrap();
            }
            TestDb::SQLite => {}
        }
    }
}

/// Test helpers
impl TestContext {
    /// Create a new master benchmark request and add it to the DB.
    pub async fn insert_master_request(
        &self,
        sha: &str,
        parent: &str,
        pr: u32,
    ) -> BenchmarkRequest {
        let req = BenchmarkRequest::create_master(sha, parent, pr, Utc::now());
        self.db().insert_benchmark_request(&req).await.unwrap();
        req
    }

    pub async fn complete_request(&self, tag: &str) {
        // Note: this assumes that there are not non-completed jobs in the DB for the request
        self.db()
            .maybe_mark_benchmark_request_as_completed(tag)
            .await
            .unwrap();
    }

    pub async fn upsert_master_artifact(&self, sha: &str) -> ArtifactIdNumber {
        self.db()
            .artifact_id(&ArtifactId::Commit(Commit {
                sha: sha.to_string(),
                date: Date(Utc::now()),
                r#type: CommitType::Master,
            }))
            .await
    }

    pub async fn add_collector(&self, collector: CollectorBuilder) -> CollectorConfig {
        self.db()
            .add_collector_config(
                &collector.name,
                collector.target,
                collector.benchmark_set.get_id(),
                true,
            )
            .await
            .unwrap()
    }
}

/// Runs a test against an actual postgres database.
pub async fn run_postgres_test<F, Fut>(f: F)
where
    F: Fn(TestContext) -> Fut,
    Fut: Future<Output = anyhow::Result<TestContext>>,
{
    // Postgres
    if let Ok(db_url) = std::env::var("TEST_DB_URL") {
        eprintln!("Running test with Postgres");
        let ctx = TestContext::new_postgres(&db_url).await;
        let ctx = f(ctx).await.expect("Postgres test failed");
        ctx.finish().await;
    } else {
        // The github CI does not yet support running containers on Windows,
        // meaning that the test suite would fail.
        if cfg!(unix) {
            panic!("Aborting; `TEST_DB_URL` was not passed");
        } else {
            eprintln!(
                "Skipping database test on platform {} `TEST_DB_URL` was not passed",
                std::env::consts::OS
            );
        }
    }
}

/// Runs a test against an actual database.
/// Checks both Postgres and SQLite.
#[allow(dead_code)]
pub async fn run_db_test<F, Fut>(f: F)
where
    F: Fn(TestContext) -> Fut + Clone,
    Fut: Future<Output = anyhow::Result<TestContext>>,
{
    run_postgres_test(f.clone()).await;
    // SQLite
    eprintln!("Running test with SQLite");
    let ctx = TestContext::new_sqlite().await;
    let ctx = f(ctx).await.expect("SQLite test failed");
    ctx.finish().await;
}
