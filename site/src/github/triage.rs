use crate::github::client::{Client, GraphQLClient, ResponseComment};
use crate::request_handlers::parse_unrolled_build_message;
use database::QueuedCommit;

pub struct TriageBuild {
    rollup_pr_number: u32,
    triage_comment: ResponseComment,
}

pub const TRIAGE_MARKER: &str = "<!-- rust-timer: triage -->";

/// Returns `Some` if this commit is part of a triage run.
pub async fn is_triage_run(
    commit: &QueuedCommit,
    client: &mut Client,
    graph_qlclient: &mut GraphQLClient,
) -> anyhow::Result<Option<TriageBuild>> {
    // Find the rollup PR
    let commit_title = client.get_commit(&commit.sha).await?;
    let Ok(unrolled_build) = parse_unrolled_build_message(&commit_title.commit.message) else {
        // The commit title parsed successfully during the `@rust-timer triage` command, so if it does not parse here it's not a triage run
        return Ok(None);
    };

    // Find the triage run on the rollup PR
    let rollup_comments = graph_qlclient
        .get_comments(unrolled_build.rollup_pr_number)
        .await?;
    let Some(triage_comment) = rollup_comments.into_iter().rev().find(|c| {
        c.author.login == "rust-timer"
            && c.body.contains(TRIAGE_MARKER)
            && c.body.contains(&commit.sha)
    }) else {
        // This was a try job on the unrolled build that did not originate from a triage command
        return Ok(None);
    };

    Ok(Some(TriageBuild {
        rollup_pr_number: unrolled_build.rollup_pr_number,
        triage_comment,
    }))
}
