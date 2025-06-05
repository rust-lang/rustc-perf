//! The `jobs` table provides a way to have scheduled jobs
use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use cron::Schedule;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client as DbClient;
use uuid::Uuid;

pub struct JobSchedule {
    pub name: &'static str,
    pub schedule: Schedule,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub scheduled_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

pub async fn insert_job(
    db: &DbClient,
    name: &str,
    scheduled_at: &DateTime<Utc>,
    metadata: &serde_json::Value,
) -> Result<()> {
    tracing::trace!("insert_job(name={})", name);

    db.execute(
        "INSERT INTO jobs (name, scheduled_at, metadata) VALUES ($1, $2, $3) 
            ON CONFLICT (name, scheduled_at) DO UPDATE SET metadata = EXCLUDED.metadata",
        &[&name, &scheduled_at, &metadata],
    )
    .await
    .context("Inserting job")?;

    Ok(())
}

pub async fn delete_job(db: &DbClient, id: &Uuid) -> Result<()> {
    tracing::trace!("delete_job(id={})", id);

    db.execute("DELETE FROM jobs WHERE id = $1", &[&id])
        .await
        .context("Deleting job")?;

    Ok(())
}

pub async fn update_job_error_message(db: &DbClient, id: &Uuid, message: &String) -> Result<()> {
    tracing::trace!("update_job_error_message(id={})", id);

    db.execute(
        "UPDATE jobs SET error_message = $2 WHERE id = $1",
        &[&id, &message],
    )
    .await
    .context("Updating job error message")?;

    Ok(())
}

pub async fn update_job_executed_at(db: &DbClient, id: &Uuid) -> Result<()> {
    tracing::trace!("update_job_executed_at(id={})", id);

    db.execute("UPDATE jobs SET executed_at = now() WHERE id = $1", &[&id])
        .await
        .context("Updating job executed at")?;

    Ok(())
}

pub async fn get_job_by_name_and_scheduled_at(
    db: &DbClient,
    name: &str,
    scheduled_at: &DateTime<Utc>,
) -> Result<Job> {
    tracing::trace!(
        "get_job_by_name_and_scheduled_at(name={}, scheduled_at={})",
        name,
        scheduled_at
    );

    let job = db
        .query_one(
            "SELECT * FROM jobs WHERE name = $1 AND scheduled_at = $2",
            &[&name, &scheduled_at],
        )
        .await
        .context("Select job by name and scheduled at")?;

    deserialize_job(&job)
}

// Selects all jobs with:
//  - scheduled_at in the past
//  - error_message is null or executed_at is at least 60 minutes ago (intended to make repeat executions rare enough)
pub async fn get_jobs_to_execute(db: &DbClient) -> Result<Vec<Job>> {
    let jobs = db
        .query(
            "
        SELECT * FROM jobs WHERE scheduled_at <= now() AND (error_message IS NULL OR executed_at <= now() - INTERVAL '60 minutes')",
            &[],
        )
        .await
        .context("Getting jobs data")?;

    let mut data = Vec::with_capacity(jobs.len());
    for job in jobs {
        let serialized_job = deserialize_job(&job);
        data.push(serialized_job.unwrap());
    }

    Ok(data)
}

fn deserialize_job(row: &tokio_postgres::row::Row) -> Result<Job> {
    let id: Uuid = row.try_get(0)?;
    let name: String = row.try_get(1)?;
    let scheduled_at: DateTime<Utc> = row.try_get(2)?;
    let metadata: serde_json::Value = row.try_get(3)?;
    let executed_at: Option<DateTime<Utc>> = row.try_get(4)?;
    let error_message: Option<String> = row.try_get(5)?;

    Ok(Job {
        id,
        name,
        scheduled_at,
        metadata,
        executed_at,
        error_message,
    })
}
