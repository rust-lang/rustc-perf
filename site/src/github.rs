pub mod client;
pub mod comparison_summary;

use crate::api::github::Commit;
use crate::load::{SiteCtxt, TryCommit};

use serde::Deserialize;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub use comparison_summary::post_finished;

/// Enqueues try build artifacts and posts a message about them on the original rollup PR
pub async fn unroll_rollup(
    ci_client: client::Client,
    main_repo_client: client::Client,
    rollup_merges: impl Iterator<Item = &Commit>,
    previous_master: &str,
    rollup_pr_number: u32,
) -> Result<(), String> {
    let mapping = enqueue_unrolled_try_builds(ci_client, rollup_merges, previous_master)
        .await?
        .into_iter()
        .fold(String::new(), |mut string, c| {
            use std::fmt::Write;
            write!(
                &mut string,
                "|#{pr}|[{commit}](https://github.com/rust-lang-ci/rust/commit/{commit})|\n",
                pr = c.original_pr_number,
                commit = c.sha
            )
            .unwrap();
            string
        });
    let msg =
        format!("📌 Perf builds for each rolled up PR:\n\n\
        |PR# | Perf Build Sha|\n|----|-----|\n\
        {mapping}\nIn the case of a perf regression, \
        run the following command for each PR you suspect might be the cause: `@rust-timer build $SHA`");
    main_repo_client.post_comment(rollup_pr_number, msg).await;
    Ok(())
}

/// Enqueues try builds on the try-perf branch for every rollup merge in `rollup_merges`.
/// Returns a mapping between the rollup merge commit and the try build sha.
async fn enqueue_unrolled_try_builds<'a>(
    client: client::Client,
    rollup_merges: impl Iterator<Item = &'a Commit>,
    previous_master: &str,
) -> Result<Vec<UnrolledCommit<'a>>, String> {
    let mut mapping = Vec::new();
    for rollup_merge in rollup_merges {
        // Grab the number of the rolled up PR from its commit message
        let original_pr_number = ROLLEDUP_PR_NUMBER
            .captures(&rollup_merge.message)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .ok_or_else(|| {
                format!(
                    "Could not get PR number from message: '{}'",
                    rollup_merge.message
                )
            })?;

        // Fetch the rollup merge commit which should have two parents.
        // The first parent is in the chain of rollup merge commits all the way back to `previous_master`.
        // The second parent is the head of the PR that was rolled up. We want the second parent.
        let commit = client.get_commit(&rollup_merge.sha).await.map_err(|e| {
            format!(
                "Error getting rollup merge commit '{}': {e:?}",
                rollup_merge.sha
            )
        })?;
        assert!(
            commit.parents.len() == 2,
            "What we thought was a merge commit was not a merge commit. sha: {}",
            rollup_merge.sha
        );
        let rolled_up_head = &commit.parents[1].sha;

        // Reset perf-tmp to the previous master
        client
            .update_branch("perf-tmp", previous_master)
            .await
            .map_err(|e| format!("Error updating perf-tmp with previous master: {e:?}"))?;

        // Merge in the rolled up PR's head commit into the previous master
        let sha = client
            .merge_branch(
                "perf-tmp",
                rolled_up_head,
                &format!("Unrolled build for #{}", original_pr_number),
            )
            .await
            .map_err(|e| format!("Error merging commit into perf-tmp: {e:?}"))?;

        // Force the `try-perf` branch to point to what the perf-tmp branch points to
        client
            .update_branch("try-perf", &sha)
            .await
            .map_err(|e| format!("Error updating the try-perf branch: {e:?}"))?;

        mapping.push(UnrolledCommit {
            original_pr_number,
            rollup_merge,
            sha,
        });
        // Wait to ensure there's enough time for GitHub to checkout these changes before they are overwritten
        tokio::time::sleep(std::time::Duration::from_secs(15)).await
    }

    Ok(mapping)
}

/// A commit representing a rolled up PR as if it had been merged into master directly
pub struct UnrolledCommit<'a> {
    /// The PR number that was rolled up
    pub original_pr_number: &'a str,
    /// The original rollup merge commit
    pub rollup_merge: &'a Commit,
    /// The sha of the new unrolled merge commit
    pub sha: String,
}

lazy_static::lazy_static! {
    static ref ROLLUP_PR_NUMBER: regex::Regex =
        regex::Regex::new(r#"^Auto merge of #(\d+)"#).unwrap();
    static ref ROLLEDUP_PR_NUMBER: regex::Regex =
        regex::Regex::new(r#"^Rollup merge of #(\d+)"#).unwrap();
}

// Gets the pr number for the associated rollup PR message. Returns None if this is not a rollup PR
pub async fn rollup_pr_number(
    client: &client::Client,
    message: &str,
) -> Result<Option<u32>, String> {
    if !message.starts_with("Auto merge of") {
        return Ok(None);
    }

    let number = ROLLUP_PR_NUMBER
        .captures(&message)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().parse::<u64>())
        .transpose()
        .map_err(|e| format!("Error parsing PR number from '{message}': {e:?}"))?;

    let number = match number {
        Some(n) => n,
        None => return Ok(None),
    };

    let issue = client
        .get_issue(number)
        .await
        .map_err(|e| format!("Error fetching PR #{number} {e:?}"))?;

    Ok(issue
        .labels
        .iter()
        .any(|l| l.name == "rollup")
        .then(|| issue.number))
}

pub async fn enqueue_sha(
    ctxt: &SiteCtxt,
    main_client: &client::Client,
    ci_client: &client::Client,
    pr_number: u32,
    commit: String,
) -> Result<(), String> {
    let commit_response = ci_client
        .get_commit(&commit)
        .await
        .map_err(|e| e.to_string())?;
    if commit_response.parents.len() != 2 {
        log::error!(
            "Bors try commit {} unexpectedly has {} parents.",
            commit_response.sha,
            commit_response.parents.len()
        );
        return Ok(());
    }
    let try_commit = TryCommit {
        sha: commit_response.sha.clone(),
        parent_sha: commit_response.parents[0].sha.clone(),
    };
    let queued = {
        let conn = ctxt.conn().await;
        conn.pr_attach_commit(pr_number, &try_commit.sha, &try_commit.parent_sha)
            .await
    };
    if queued {
        let msg = format!(
            "Queued {} with parent {}, future [comparison URL]({}).",
            try_commit.sha,
            try_commit.parent_sha,
            try_commit.comparison_url(),
        );
        main_client.post_comment(pr_number, msg).await;
    }
    Ok(())
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
    let start_idx = comment_body.find(start).expect("found homu") + start.len();
    let end_idx = start_idx + comment_body[start_idx..].find(" -->").unwrap();

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

#[derive(serde::Deserialize)]
pub(crate) struct PullRequest {
    pub number: u64,
    pub title: String,
}

#[derive(serde::Deserialize)]
struct PullRequestResponse {
    items: Vec<PullRequest>,
}

/// Fetch all merged PRs that are labeled with `perf-regression` and not `perf-regression-triaged`
pub(crate) async fn untriaged_perf_regressions() -> Result<Vec<PullRequest>, BoxedError> {
    let url = "https://api.github.com/search/issues?q=repo:rust-lang/rust+label:perf-regression+-label:perf-regression-triaged+is:merged".to_owned();
    let request = github_request(&url);
    Ok(send_request::<PullRequestResponse>(request).await?.items)
}

/// Get the title of a PR with the given number
pub(crate) async fn pr_title(pr: u32) -> String {
    let url = format!("https://api.github.com/repos/rust-lang/rust/pulls/{}", pr);
    let request = github_request(&url);

    async fn send(request: reqwest::RequestBuilder) -> Result<String, BoxedError> {
        let body = send_request::<serde_json::Value>(request).await?;
        Ok(body
            .get("title")
            .ok_or_else(malformed_json_error)?
            .as_str()
            .ok_or_else(malformed_json_error)?
            .to_owned())
    }
    match send(request).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error fetching url: {}", e);
            String::from("<UNKNOWN>")
        }
    }
}

fn github_request(url: &str) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();
    let mut request = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "rustc-perf");
    if let Some(token) = std::env::var("GITHUB_TOKEN").ok() {
        request = request.header("Authorization", format!("token {}", token));
    }
    request
}

async fn send_request<T: serde::de::DeserializeOwned>(
    request: reqwest::RequestBuilder,
) -> Result<T, BoxedError> {
    Ok(request
        .send()
        .await?
        .error_for_status()?
        .json::<T>()
        .await?)
}

fn malformed_json_error() -> String {
    "JSON was malformed".to_owned()
}
