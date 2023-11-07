use std::str;
use std::sync::Arc;

use crate::api::status;
use crate::db::{ArtifactId, Lookup};
use crate::load::SiteCtxt;

pub async fn handle_status_page(ctxt: Arc<SiteCtxt>) -> status::Response {
    let idx = ctxt.index.load();
    let last_commit = idx.commits().last().cloned();

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

    let errors = if let Some(last) = &last_commit {
        conn.get_error(ArtifactId::from(last.clone()).lookup(&idx).unwrap())
            .await
    } else {
        Default::default()
    };
    let benchmark_state = errors
        .into_iter()
        .map(|(name, error)| {
            let error = prettify_log(&error).unwrap_or(error);
            status::BenchmarkStatus { name, error }
        })
        .collect::<Vec<_>>();

    status::Response {
        last_commit,
        benchmarks: benchmark_state,
        missing,
        current,
        most_recent_end: conn
            .last_artifact_collection()
            .await
            .map(|d| d.end_time.timestamp()),
    }
}

fn prettify_log(log: &str) -> Option<String> {
    let mut lines = log.lines();
    let first = lines.next()?;
    let log = &first[first.find('"')? + 1..];
    let log = &log[..log.find("\" }")?];
    Some(log.replace("\\n", "\n"))
}
