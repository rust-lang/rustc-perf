use std::future::Future;
use tokio_postgres::config::Host;
use tokio_postgres::Config;

use crate::pool::postgres::make_client;
use crate::Pool;

/// Represents a connection to a Postgres database that can be
/// used in integration tests to test logic that interacts with
/// a database.
pub(crate) struct TestContext {
    db_name: String,
    original_db_url: String,
    // Pre-cached client to avoid creating unnecessary connections in tests
    client: Pool,
}

impl TestContext {
    async fn new(db_url: &str) -> Self {
        let config: Config = db_url.parse().expect("Cannot parse connection string");

        // Create a new database that will be used for this specific test
        let client = make_client(&db_url)
            .await
            .expect("Cannot connect to database");
        let db_name = format!("db{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        client
            .execute(&format!("CREATE DATABASE {db_name}"), &[])
            .await
            .expect("Cannot create database");
        drop(client);

        // We need to connect to the database against, because Postgres doesn't allow
        // changing the active database mid-connection.
        // There does not seem to be a way to turn the config back into a connection
        // string, so construct it manually.
        let test_db_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.get_user().unwrap(),
            String::from_utf8(config.get_password().unwrap().to_vec()).unwrap(),
            match &config.get_hosts()[0] {
                Host::Tcp(host) => host,
                Host::Unix(_) =>
                    panic!("Unix sockets in Postgres connection string are not supported"),
            },
            &config.get_ports()[0],
            db_name
        );
        let pool = Pool::open(test_db_url.as_str());

        Self {
            db_name,
            original_db_url: db_url.to_string(),
            client: pool,
        }
    }

    pub(crate) fn db_client(&self) -> &Pool {
        &self.client
    }

    async fn finish(self) {
        // Cleanup the test database
        // First, we need to stop using the database
        drop(self.client);

        // Then we need to connect to the default database and drop our test DB
        let client = make_client(&self.original_db_url)
            .await
            .expect("Cannot connect to database");
        client
            .execute(&format!("DROP DATABASE {}", self.db_name), &[])
            .await
            .unwrap();
    }
}

pub(crate) async fn run_db_test<F, Fut>(f: F)
where
    F: FnOnce(TestContext) -> Fut,
    Fut: Future<Output = anyhow::Result<TestContext>>,
{
    if let Ok(db_url) = std::env::var("TEST_DB_URL") {
        let ctx = TestContext::new(&db_url).await;
        let ctx = f(ctx).await.expect("Test failed");
        ctx.finish().await;
    } else {
        eprintln!("Skipping test because TEST_DB_URL was not passed");
    }
}
