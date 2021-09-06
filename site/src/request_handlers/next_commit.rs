use crate::load::{MissingReason, SiteCtxt};
use collector::api::next_commit;

use std::sync::Arc;

pub async fn handle_next_commit(ctxt: Arc<SiteCtxt>) -> next_commit::Response {
    let next_commit = ctxt.missing_commits().await.into_iter().next();

    let next_commit = if let Some((commit, missing_reason)) = next_commit {
        // If we're going to run a master commit next, make sure
        // it's been enqueued in the pull_request_build table
        if let MissingReason::Master { pr, ref parent_sha } = missing_reason {
            let conn = ctxt.conn().await;
            // TODO: add capability of doing the following in one step
            // to avoid possibile illegal inbetween states.
            conn.queue_pr(pr, None, None, None).await;
            conn.pr_attach_commit(pr, &commit.sha, parent_sha).await;
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
        Some(next_commit::Commit {
            sha: commit.sha,
            include,
            exclude,
            runs,
        })
    } else {
        None
    };

    next_commit::Response {
        commit: next_commit,
    }
}
