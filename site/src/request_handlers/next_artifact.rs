use crate::load::{MissingReason, SiteCtxt};
use collector::api::next_artifact;

use std::sync::Arc;

pub async fn handle_next_artifact(ctxt: Arc<SiteCtxt>) -> next_artifact::Response {
    // Prefer benchmarking released artifacts first
    match ctxt.missing_published_artifacts().await {
        Ok(next_artifact) => {
            if let Some(next_artifact) = next_artifact.into_iter().next() {
                log::debug!("next_artifact: {next_artifact}");
                return next_artifact::Response {
                    artifact: Some(next_artifact::NextArtifact::Release(next_artifact)),
                };
            }
        }
        Err(error) => log::error!("Failed to fetch missing artifacts: {error:?}"),
    }

    let next_commit = ctxt.missing_commits().await.into_iter().next();

    let next_commit = if let Some((commit, missing_reason)) = next_commit {
        let missing_reason_dbg = format!("{:?}", missing_reason);
        // If we're going to run a master commit next, make sure
        // it's been enqueued in the pull_request_build table
        if let MissingReason::Master {
            pr, ref parent_sha, ..
        } = missing_reason
        {
            let conn = ctxt.conn().await;
            // TODO: add capability of doing the following in one step
            // to avoid possibile illegal inbetween states.
            conn.queue_pr(pr, None, None, None).await;
            if !conn
                .pr_attach_commit(pr, &commit.sha, parent_sha, None)
                .await
            {
                log::error!("failed to attach commit {} to PR queue", commit.sha);
            }
        }
        let (include, exclude, runs) = match missing_reason {
            crate::load::MissingReason::Try {
                include,
                exclude,
                runs,
                ..
            } => (include, exclude, runs),
            crate::load::MissingReason::InProgress(Some(previous)) => {
                if let crate::load::MissingReason::Try {
                    include,
                    exclude,
                    runs,
                    ..
                } = *previous
                {
                    (include, exclude, runs)
                } else {
                    (None, None, None)
                }
            }
            _ => (None, None, None),
        };
        log::debug!(
            "next_commit: {} (missing: {})",
            commit.sha,
            missing_reason_dbg
        );
        Some(next_artifact::NextArtifact::Commit {
            commit,
            include,
            exclude,
            runs,
        })
    } else {
        None
    };

    next_artifact::Response {
        artifact: next_commit,
    }
}
