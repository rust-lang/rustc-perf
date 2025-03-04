use std::str;
use std::sync::Arc;

use crate::api::status;
use crate::api::status::FinishedRun;
use crate::load::SiteCtxt;

use database::{ArtifactId, Lookup};

// How many historical (finished) runs should be returned from the status API.
const FINISHED_RUN_COUNT: u64 = 5;

pub async fn handle_status_page(ctxt: Arc<SiteCtxt>) -> status::Response {
    let missing = ctxt.missing_commits().await;

    // FIXME: no current builds
    let conn = ctxt.conn().await;
    let current = if let Some(artifact) = conn.in_progress_artifacts().await.pop() {
        let steps = conn
            .in_progress_steps(&artifact)
            .await
            .into_iter()
            .map(|s| crate::api::status::Step {
                step: s.name,
                is_done: s.is_done,
                expected_duration: s.expected.as_secs(),
                current_progress: s.duration.as_secs(),
            })
            .collect();

        Some(crate::api::status::CurrentState {
            artifact,
            progress: steps,
        })
    } else {
        None
    };

    // FIXME: load at least one master commit with errors, if no master commit is in last N commits?
    // FIXME: cache this whole thing, or write a specific SQL query for it
    let mut finished_runs = Vec::new();
    let idx = ctxt.index.load();

    let recent_collections = conn
        .last_n_artifact_collections(FINISHED_RUN_COUNT as u32)
        .await;
    for collection in recent_collections {
        let errors = conn
            .get_error(collection.artifact.lookup(&idx).unwrap())
            .await;
        let mut pr = None;
        if let ArtifactId::Commit(ref commit) = collection.artifact {
            pr = conn.pr_of(&commit.sha).await;
        }
        finished_runs.push(FinishedRun {
            artifact: collection.artifact,
            pr,
            errors: errors
                .into_iter()
                .map(|(name, error)| {
                    let error = prettify_log(&error).unwrap_or(error);
                    status::BenchmarkError { name, error }
                })
                .collect::<Vec<_>>(),
            duration: collection.duration.as_secs(),
            finished_at: collection.end_time.timestamp() as u64,
        });
    }

    status::Response {
        finished_runs,
        missing,
        current,
    }
}

fn prettify_log(log: &str) -> Option<String> {
    let mut lines = log.lines();
    let first = lines.next()?;
    let log = &first[first.find('"')? + 1..];
    let log = &log[..log.find("\" }")?];
    Some(log.replace("\\n", "\n"))
}
