use crate::load::SiteCtxt;
use collector::api::next_commit;

use std::sync::Arc;

pub async fn handle_next_commit(ctxt: Arc<SiteCtxt>) -> next_commit::Response {
    let commit = ctxt
        .missing_commits()
        .await
        .into_iter()
        .next()
        .map(|(commit, missing_reason)| {
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
            next_commit::Commit {
                sha: commit.sha,
                include,
                exclude,
                runs,
            }
        });

    next_commit::Response { commit }
}
