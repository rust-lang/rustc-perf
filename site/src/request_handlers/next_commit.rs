use crate::load::SiteCtxt;
use collector::api::next_commit;

use std::sync::Arc;

pub async fn handle_next_commit(ctxt: Arc<SiteCtxt>) -> next_commit::Response {
    let commit = ctxt.missing_commits().await.into_iter().next();

    let commit = if let Some((commit, missing_reason)) = commit {
        // If we're going to run a master commit next, make sure
        // it's been enqueued in the pull_request_build table
        if let Some((pr, parent_sha)) = missing_reason.master_commit_pr_and_parent() {
            let conn = ctxt.conn().await;
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

    next_commit::Response { commit }
}
