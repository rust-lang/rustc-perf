pub mod client;
pub mod comparison_summary;
pub mod triage;

use crate::job_queue::{build_queue, estimate_queue_times_using_conn};
use crate::load::SiteCtxt;
use anyhow::bail;
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

pub const RUST_REPO_GITHUB_API_URL: &str = "https://api.github.com/repos/rust-lang/rust";

/// Comments that are temporary and do not add any value once there has been a new development
/// (a rustc build or a perf. run was finished) are marked with this comment.
///
/// They are removed once a perf. run comparison summary is posted on a PR.
pub const COMMENT_MARK_TEMPORARY: &str = "<!-- rust-timer: temporary -->";

use crate::github::client::Commit;
use database::Connection;

/// Enqueues the given SHA and returns a message that should be sent as a comment to the corresponding PR.
/// If no benchmark request was found to which the commit SHA could be attached, returns `Ok(None)`.
pub async fn enqueue_sha(
    ctxt: &SiteCtxt,
    mut commit: Commit,
    pr_number: u32,
) -> Result<Option<String>, String> {
    if commit.parents.len() != 2 {
        return Err(format!(
            "Bors try commit {} unexpectedly has {} parents.",
            commit.sha,
            commit.parents.len()
        ));
    }
    let sha = commit.sha;
    let parent_sha = commit.parents.remove(0).sha;
    let conn = ctxt.conn().await;

    let queued = conn.attach_shas_to_try_benchmark_request(
            pr_number,
            &sha,
            &parent_sha,
            commit.commit.committer.date,
            )
            .await
            .map_err(|error| format!("Cannot attach SHAs to try benchmark request on PR {pr_number} and SHA {sha}: {error:?}"))?;
    if !queued {
        return Ok(None);
    }

    let (preceding_artifacts, expected_duration) = estimate_queue_info(conn.as_ref(), &sha)
        .await
        .map_err(|e| format!("{e:?}"))?;

    let verb = if preceding_artifacts == 1 {
        "is"
    } else {
        "are"
    };
    let suffix = if preceding_artifacts == 1 { "" } else { "s" };
    let queue_msg = format!(
        r#"There {verb} currently {preceding_artifacts} preceding artifact{suffix} in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~{:.1} hours until the benchmark run finishes."#,
        expected_duration.as_secs_f64() / 3600.0
    );

    Ok(Some(format!(
        "Queued {sha} with parent {parent_sha}, future [comparison URL]({}).\n{queue_msg}",
        comparison_url(&sha, &parent_sha),
    )))
}

fn comparison_url(commit: &str, parent: &str) -> String {
    format!("https://perf.rust-lang.org/compare.html?start={parent}&end={commit}")
}

/// Counts how many artifacts are in the queue before the specified commit, and what is the expected
/// duration until the specified commit will be finished.
async fn estimate_queue_info(
    conn: &dyn Connection,
    commit_sha: &str,
) -> anyhow::Result<(usize, Duration)> {
    let queue = build_queue(conn).await?;
    let estimates = estimate_queue_times_using_conn(conn, &queue).await?;

    for (i, (req, estimate)) in queue.iter().zip(estimates).enumerate() {
        if req.tag() == Some(commit_sha) {
            let now = Utc::now();
            return Ok((i, (estimate - now).to_std().unwrap_or_default()));
        }
    }

    bail!("Tried to estimate queue info for commit `{commit_sha}` which is not in queue");
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum HomuComment {
    TryBuildCompleted { merge_sha: String },
}

/// Parse comment from homu containing try build sha
pub async fn parse_homu_comment(comment_body: &str) -> Option<String> {
    if !comment_body.contains("Try build successful") {
        return None;
    }

    let start = "<!-- homu: ";
    let start_idx = comment_body.find(start)? + start.len();
    let end_idx = start_idx + comment_body[start_idx..].find(" -->")?;

    let sha = match serde_json::from_str(&comment_body[start_idx..end_idx]) {
        Ok(HomuComment::TryBuildCompleted { merge_sha }) => merge_sha,
        Err(err) => {
            log::warn!(
                "failed to parse try build result; comment: {:?}, part: {:?}, err: {:?}",
                comment_body,
                &comment_body[start_idx..end_idx],
                err
            );
            return None;
        }
    };

    Some(sha)
}
