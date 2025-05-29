//! # Scheduled Jobs
//!
//! Scheduled jobs essentially come in two flavors: automatically repeating
//! (cron) jobs and one-off jobs.
//!
//! The core trait here is the `Job` trait, which *must* define the name of the
//! job (to be used as an identifier in the database) and the function to run
//! when the job runs.
//!
//! The metadata is a serde_json::Value
//! Please refer to https://docs.rs/serde_json/latest/serde_json/value/fn.from_value.html
//! on how to interpret it as an instance of type T, implementing Serialize/Deserialize.
//!
//! The schedule is a cron::Schedule
//! Please refer to https://docs.rs/cron/latest/cron/struct.Schedule.html for further info
//!
//! ## Example, sending a zulip message once a week
//!
//! To give an example, let's imagine we want to sends a Zulip message every
//! Friday at 11:30am ET into #t-release with a "@T-release meeting!"" content.
//!
//! To begin, let's create a generic zulip message Job:
//!    #[derive(Serialize, Deserialize)]
//!    struct ZulipMetadata {
//!      pub message: String
//!      pub channel: String,
//!    }
//!    struct ZulipMessageJob;
//!    impl Job for ZulipMessageJob { ... }
//!
//! (Imagine that this job requires a channel and a message in the metadata.)
//!
//! If we wanted to have a default scheduled message, we could add the following to
//! `default_jobs`:
//!     JobSchedule {
//!         name: ZulipMessageJob.name(),
//!         schedule: Schedule::from_str("0 30 11 * * FRI *").unwrap(),
//!         metadata: serde_json::value::to_value(ZulipMetadata {
//!             message: "@T-release meeting!".to_string()
//!             channel: "T-release".to_string(),
//!         }).unwrap(),
//!     }

use std::str::FromStr;

use async_trait::async_trait;
use cron::Schedule;

use crate::handlers::pull_requests_assignment_update::PullRequestAssignmentUpdate;
use crate::{
    db::jobs::JobSchedule,
    handlers::{docs_update::DocsUpdateJob, rustc_commits::RustcCommitsJob, Context},
};

/// How often new cron-based jobs will be placed in the queue.
/// This is the minimum period *between* a single cron task's executions.
pub const JOB_SCHEDULING_CADENCE_IN_SECS: u64 = 1800;

/// How often the database is inspected for jobs which need to execute.
/// This is the granularity at which events will occur.
pub const JOB_PROCESSING_CADENCE_IN_SECS: u64 = 60;

// The default jobs list that are currently scheduled to run
pub fn jobs() -> Vec<Box<dyn Job + Send + Sync>> {
    vec![
        Box::new(DocsUpdateJob),
        Box::new(RustcCommitsJob),
        Box::new(PullRequestAssignmentUpdate),
    ]
}

// Definition of the schedule repetition for the jobs we want to run.
pub fn default_jobs() -> Vec<JobSchedule> {
    vec![
        JobSchedule {
            name: DocsUpdateJob.name(),
            // Around 9am Pacific time on every Monday.
            schedule: Schedule::from_str("0 00 17 * * Mon *").unwrap(),
            metadata: serde_json::Value::Null,
        },
        JobSchedule {
            name: RustcCommitsJob.name(),
            // Every 30 minutes...
            schedule: Schedule::from_str("* 0,30 * * * * *").unwrap(),
            metadata: serde_json::Value::Null,
        },
        JobSchedule {
            name: PullRequestAssignmentUpdate.name(),
            // Every 30 minutes
            schedule: Schedule::from_str("* 0,30 * * * * *").unwrap(),
            metadata: serde_json::Value::Null,
        },
    ]
}

#[async_trait]
pub trait Job {
    fn name(&self) -> &str;

    async fn run(&self, ctx: &Context, metadata: &serde_json::Value) -> anyhow::Result<()>;
}

#[test]
fn jobs_defined() {
    // This checks that we don't panic (during schedule parsing) and that all names are unique
    // Checks we don't panic here, mostly for the schedule parsing.
    let all_jobs = jobs();
    let mut all_job_names: Vec<_> = all_jobs.into_iter().map(|j| j.name().to_string()).collect();
    all_job_names.sort();
    let mut unique_all_job_names = all_job_names.clone();
    unique_all_job_names.sort();
    unique_all_job_names.dedup();
    assert_eq!(all_job_names, unique_all_job_names);

    // Also ensure that our default jobs are release jobs
    let default_jobs = default_jobs();
    default_jobs
        .iter()
        .for_each(|j| assert!(all_job_names.contains(&j.name.to_string())));
}
