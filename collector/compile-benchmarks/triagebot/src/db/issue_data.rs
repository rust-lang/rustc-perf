//! The `issue_data` table provides a way to track extra metadata about an
//! issue/PR.
//!
//! Each issue has a unique "key" where you can store data under. Typically
//! that key should be the name of the handler. The data can be anything that
//! can be serialized to JSON.
//!
//! Note that this uses crude locking, so try to keep the duration between
//! loading and saving to a minimum.

use crate::github::Issue;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::Json;
use tokio_postgres::{Client as DbClient, Transaction};

pub struct IssueData<'db, T>
where
    T: for<'a> Deserialize<'a> + Serialize + Default + std::fmt::Debug + Sync + PartialEq + Clone,
{
    transaction: Transaction<'db>,
    repo: String,
    issue_number: i32,
    key: String,
    pub data: T,
    initial_data: T,
}

impl<'db, T> IssueData<'db, T>
where
    T: for<'a> Deserialize<'a> + Serialize + Default + std::fmt::Debug + Sync + PartialEq + Clone,
{
    pub async fn load(
        db: &'db mut DbClient,
        issue: &Issue,
        key: &str,
    ) -> Result<IssueData<'db, T>> {
        let repo = issue.repository().to_string();
        let issue_number = issue.number as i32;
        let transaction = db.transaction().await?;
        transaction
            .execute("LOCK TABLE issue_data", &[])
            .await
            .context("locking issue data")?;
        let data = transaction
            .query_opt(
                "SELECT data FROM issue_data WHERE \
                 repo = $1 AND issue_number = $2 AND key = $3",
                &[&repo, &issue_number, &key],
            )
            .await
            .context("selecting issue data")?
            .map(|row| row.get::<usize, Json<T>>(0).0)
            .unwrap_or_default();

        let initial_data = data.clone();

        Ok(IssueData {
            transaction,
            repo,
            issue_number,
            key: key.to_string(),
            data,
            initial_data,
        })
    }

    pub async fn save(self) -> Result<()> {
        // Avoid writing to the DB needlessly.
        if self.data != self.initial_data {
            self.transaction
                .execute(
                    "INSERT INTO issue_data (repo, issue_number, key, data) \
                 VALUES ($1, $2, $3, $4) \
                 ON CONFLICT (repo, issue_number, key) DO UPDATE SET data=EXCLUDED.data",
                    &[&self.repo, &self.issue_number, &self.key, &Json(&self.data)],
                )
                .await
                .context("inserting issue data")?;
        }
        self.transaction
            .commit()
            .await
            .context("committing issue data")?;
        Ok(())
    }
}
