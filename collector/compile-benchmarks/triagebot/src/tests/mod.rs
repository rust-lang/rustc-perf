use crate::db;
use crate::db::users::record_username;
use crate::db::{make_client, ClientPool, PooledClient};
use crate::github::GithubClient;
use crate::handlers::Context;
use octocrab::Octocrab;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_postgres::config::Host;
use tokio_postgres::Config;

pub(crate) mod github;

/// Represents a connection to a Postgres database that can be
/// used in integration tests to test logic that interacts with
/// a database.
pub(crate) struct TestContext {
    ctx: Context,
    db_name: String,
    original_db_url: String,
    // Pre-cached client to avoid creating unnecessary connections in tests
    client: PooledClient,
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
        let pool = ClientPool::new(test_db_url.clone());
        let mut client = pool.get().await;
        db::run_migrations(&mut client)
            .await
            .expect("Cannot run database migrations");

        let octocrab = Octocrab::builder().build().unwrap();
        let github = GithubClient::new(
            "gh-test-fake-token".to_string(),
            "https://api.github.com".to_string(),
            "https://api.github.com/graphql".to_string(),
            "https://raw.githubusercontent.com".to_string(),
        );
        let ctx = Context {
            github,
            db: pool,
            username: "triagebot-test".to_string(),
            octocrab,
            workqueue: Arc::new(RwLock::new(Default::default())),
        };

        Self {
            db_name,
            original_db_url: db_url.to_string(),
            ctx,
            client,
        }
    }

    /// Returns a fake handler context.
    /// We currently do not mock outgoing nor incoming GitHub API calls,
    /// so the API endpoints will not be actually working.
    pub(crate) fn handler_ctx(&self) -> &Context {
        &self.ctx
    }

    pub(crate) fn db_client(&self) -> &PooledClient {
        &self.client
    }

    pub(crate) fn db_client_mut(&mut self) -> &mut PooledClient {
        &mut self.client
    }

    pub(crate) async fn add_user(&self, name: &str, id: u64) {
        record_username(self.db_client(), id, name)
            .await
            .expect("Cannot create user");
    }

    async fn finish(self) {
        // Cleanup the test database
        // First, we need to stop using the database
        drop(self.client);
        drop(self.ctx);

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

pub(crate) async fn run_db_test<F, Fut, Ctx>(f: F)
where
    F: FnOnce(TestContext) -> Fut,
    Fut: Future<Output = anyhow::Result<Ctx>>,
    Ctx: Into<TestContext>,
{
    if let Ok(db_url) = std::env::var("TEST_DB_URL") {
        let ctx = TestContext::new(&db_url).await;
        let ctx: TestContext = f(ctx).await.expect("Test failed").into();
        ctx.finish().await;
    } else {
        eprintln!("Skipping test because TEST_DB_URL was not passed");
    }
}
