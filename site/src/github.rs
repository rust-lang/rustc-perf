use crate::api::{github, ServerResult};
use crate::comparison::{ComparisonSummary, Direction};
use crate::load::{Config, SiteCtxt, TryCommit};

use anyhow::Context as _;
use database::ArtifactId;
use hashbrown::HashSet;
use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

use std::{fmt::Write, sync::Arc, time::Duration};

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

lazy_static::lazy_static! {
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?"#).unwrap();
    static ref BODY_QUEUE: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+queue(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?"#).unwrap();
    static ref BODY_MAKE_PR_FOR: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+make-pr-for\s+(\w+)(?:\W|$)"#).unwrap();
    static ref BODY_UDPATE_PR_FOR: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+update-branch-for\s+(\w+)(?:\W|$)"#).unwrap();
}

async fn get_authorized_users() -> ServerResult<Vec<usize>> {
    let url = format!("{}/permissions/perf.json", ::rust_team_data::v1::BASE_URL);
    let client = reqwest::Client::new();
    client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("failed to fetch authorized users: {}", err))?
        .error_for_status()
        .map_err(|err| format!("failed to fetch authorized users: {}", err))?
        .json::<rust_team_data::v1::Permission>()
        .await
        .map_err(|err| format!("failed to fetch authorized users: {}", err))
        .map(|perms| perms.github_ids)
}

pub async fn handle_github(
    request: github::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<github::Response> {
    if request.comment.body.contains(" homu: ") {
        if let Some(sha) = handle_homu_res(&request).await {
            return enqueue_sha(request, &ctxt, sha).await;
        }
    }

    if !request.comment.body.contains("@rust-timer ") {
        return Ok(github::Response);
    }

    if request.comment.author_association != github::Association::Owner
        && !get_authorized_users()
            .await?
            .contains(&request.comment.user.id)
    {
        post_comment(
            &ctxt.config,
            request.issue.number,
            "Insufficient permissions to issue commands to rust-timer.",
        )
        .await;
        return Ok(github::Response);
    }

    if let Some(captures) = BODY_QUEUE.captures(&request.comment.body) {
        let include = captures.get(1).map(|v| v.as_str());
        let exclude = captures.get(2).map(|v| v.as_str());
        let runs = captures.get(3).and_then(|v| v.as_str().parse::<i32>().ok());
        {
            let conn = ctxt.conn().await;
            conn.queue_pr(request.issue.number, include, exclude, runs)
                .await;
        }
        post_comment(
            &ctxt.config,
            request.issue.number,
            "Awaiting bors try build completion.

@rustbot label: +S-waiting-on-perf",
        )
        .await;
        return Ok(github::Response);
    }

    if let Some(captures) = BODY_TRY_COMMIT.captures(&request.comment.body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str().to_owned()) {
            let include = captures.get(2).map(|v| v.as_str());
            let exclude = captures.get(3).map(|v| v.as_str());
            let runs = captures.get(4).and_then(|v| v.as_str().parse::<i32>().ok());
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            {
                let conn = ctxt.conn().await;
                conn.queue_pr(request.issue.number, include, exclude, runs)
                    .await;
            }
            let f = enqueue_sha(request, &ctxt, commit.to_owned());
            return f.await;
        }
    }

    let captures = BODY_MAKE_PR_FOR
        .captures_iter(&request.comment.body)
        .collect::<Vec<_>>();
    for capture in captures {
        if let Some(rollup_merge) = capture.get(1).map(|c| c.as_str().to_owned()) {
            let rollup_merge =
                rollup_merge.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            let client = reqwest::Client::new();
            pr_and_try_for_rollup(
                &client,
                ctxt.clone(),
                &request.issue.repository_url,
                &rollup_merge,
                &request.comment.html_url,
            )
            .await
            .map_err(|e| format!("{:?}", e))?;
        }
    }

    let captures = BODY_UDPATE_PR_FOR
        .captures_iter(&request.comment.body)
        .collect::<Vec<_>>();
    for capture in captures {
        if let Some(rollup_merge) = capture.get(1).map(|c| c.as_str().to_owned()) {
            let rollup_merge =
                rollup_merge.trim_start_matches("https://github.com/rust-lang/rust/commit/");

            // This just creates or updates the branch for this merge commit.
            // Intended for resolving the race condition of master merging in
            // between us updating the commit and merging things.
            let client = reqwest::Client::new();
            let branch =
                branch_for_rollup(&client, &ctxt, &request.issue.repository_url, rollup_merge)
                    .await
                    .map_err(|e| e.to_string())?;
            post_comment(
                &ctxt.config,
                request.issue.number,
                &format!("Master base SHA: {}", branch.master_base_sha),
            )
            .await;
        }
    }

    Ok(github::Response)
}

// Returns the PR number
async fn pr_and_try_for_rollup(
    client: &reqwest::Client,
    ctxt: Arc<SiteCtxt>,
    repository_url: &str,
    rollup_merge_sha: &str,
    origin_url: &str,
) -> anyhow::Result<u32> {
    log::trace!(
        "creating PR for {:?} {:?}",
        repository_url,
        rollup_merge_sha
    );
    let branch = branch_for_rollup(client, &ctxt, repository_url, rollup_merge_sha).await?;

    let pr = create_pr(
        client,
        &ctxt,
        repository_url,
        &format!(
            "[DO NOT MERGE] perf-test for #{}",
            branch.rolled_up_pr_number
        ),
        &format!("rust-timer:{}", branch.name),
        "master",
        &format!(
            "This is an automatically generated pull request (from [here]({})) to \
            run perf tests for #{} which merged in a rollup.

r? @ghost",
            origin_url, branch.rolled_up_pr_number
        ),
    )
    .await
    .context("Created PR")?;

    let pr_number = pr.number;
    let rollup_merge_sha = rollup_merge_sha.to_owned();
    tokio::task::spawn(async move {
        // Give github time to create the merge commit reference
        tokio::time::sleep(Duration::from_secs(30)).await;
        // This provides the master SHA so that we can check that we only queue
        // an appropriate try build. If there's ever a race condition, i.e.,
        // master was pushed while this command was running, the user will have to
        // take manual action to detect it.
        //
        // Eventually we'll want to handle this automatically, but that's a ways
        // off: we'd need to store the state in the database and handle the try
        // build starting and generally that's a lot of work for not too much gain.
        post_comment(
            &ctxt.config,
            pr.number,
            &format!(
                "@bors try @rust-timer queue

The try commit's (master) parent should be {master}. If it isn't, \
then please:

 * Stop this try build (`try-`).
 * Run `@rust-timer update-pr-for {merge}`.
 * Rerun `bors try`.

You do not need to reinvoke the queue command as long as the perf \
build hasn't yet started.",
                master = branch.master_base_sha,
                merge = rollup_merge_sha,
            ),
        )
        .await;
    });

    Ok(pr_number)
}

struct RollupBranch {
    master_base_sha: String,
    rolled_up_pr_number: u32,
    name: String,
}

async fn branch_for_rollup(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    rollup_merge_sha: &str,
) -> anyhow::Result<RollupBranch> {
    let rollup_merge = get_commit(&client, &ctxt, repository_url, rollup_merge_sha)
        .await
        .context("got rollup merge")?;

    let mut current = rollup_merge.clone();
    loop {
        log::trace!("searching for auto branch, at {:?}", current.sha);
        if current.commit.message.starts_with("Auto merge") {
            break;
        }
        assert_eq!(current.parents.len(), 2);
        current = get_commit(&client, &ctxt, repository_url, &current.parents[0].sha)
            .await
            .context("success master get")?;
    }
    let old_master_commit = current;

    let current_master_commit = get_commit(&client, &ctxt, repository_url, "master")
        .await
        .context("success master get")?;

    let revert_sha = create_commit(
        &client,
        &ctxt,
        "https://api.github.com/repos/rust-timer/rust",
        &format!("Revert to {}", old_master_commit.sha),
        &old_master_commit.commit.tree.sha,
        &[&current_master_commit.sha],
    )
    .await
    .context("create revert")?;

    let merge_sha = create_commit(
        &client,
        &ctxt,
        "https://api.github.com/repos/rust-timer/rust",
        &format!(
            "rust-timer simulated merge of {}\n\nOriginal message:\n{}",
            rollup_merge.sha, rollup_merge.commit.message
        ),
        &rollup_merge.commit.tree.sha,
        &[&revert_sha],
    )
    .await
    .context("create merge commit")?;

    let rolled_up_pr_number = if let Some(stripped) = rollup_merge
        .commit
        .message
        .strip_prefix("Rollup merge of #")
    {
        stripped
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    } else {
        anyhow::bail!(
            "not a rollup merge commit: {:?}",
            rollup_merge.commit.message
        )
    };

    let branch = format!("try-for-{}", rolled_up_pr_number);
    create_ref(
        &client,
        &ctxt,
        "https://api.github.com/repos/rust-timer/rust",
        &format!("refs/heads/{}", branch),
        &merge_sha,
    )
    .await
    .context("created branch")?;

    Ok(RollupBranch {
        rolled_up_pr_number,
        master_base_sha: current_master_commit.sha,
        name: branch,
    })
}

#[derive(serde::Serialize)]
struct CreateRefRequest<'a> {
    // Must start with `refs/` and have at least two slashes.
    // e.g. `refs/heads/master`.
    #[serde(rename = "ref")]
    ref_: &'a str,
    sha: &'a str,
}

pub async fn create_ref(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    ref_: &str,
    sha: &str,
) -> anyhow::Result<()> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/git/refs", repository_url);
    let response = client
        .post(&url)
        .json(&CreateRefRequest { ref_, sha })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("POST git/refs failed")?;
    if response.status() != reqwest::StatusCode::CREATED {
        anyhow::bail!("{:?} != 201 CREATED", response.status());
    }

    Ok(())
}

#[derive(serde::Serialize)]
struct CreatePrRequest<'a> {
    title: &'a str,
    // username:branch if cross-repo
    head: &'a str,
    // branch to pull into (e.g, master)
    base: &'a str,
    #[serde(rename = "body")]
    description: &'a str,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePrResponse {
    pub number: u32,
    pub html_url: String,
    pub comments_url: String,
}

pub async fn create_pr(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    title: &str,
    head: &str,
    base: &str,
    description: &str,
) -> anyhow::Result<CreatePrResponse> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/pulls", repository_url);
    let response = client
        .post(&url)
        .json(&CreatePrRequest {
            title,
            head,
            base,
            description,
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("POST pulls failed")?;
    if response.status() != reqwest::StatusCode::CREATED {
        anyhow::bail!("{:?} != 201 CREATED", response.status());
    }

    Ok(response.json().await.context("deserializing failed")?)
}

#[derive(serde::Serialize)]
struct CreateCommitRequest<'a> {
    message: &'a str,
    tree: &'a str,
    parents: &'a [&'a str],
}

#[derive(serde::Deserialize)]
struct CreateCommitResponse {
    sha: String,
}

pub async fn create_commit(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    message: &str,
    tree: &str,
    parents: &[&str],
) -> anyhow::Result<String> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/git/commits", repository_url);
    let commit_response = client
        .post(&url)
        .json(&CreateCommitRequest {
            message,
            tree,
            parents,
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("POST git/commits failed")?;
    if commit_response.status() != reqwest::StatusCode::CREATED {
        anyhow::bail!("{:?} != 201 CREATED", commit_response.status());
    }

    Ok(commit_response
        .json::<CreateCommitResponse>()
        .await
        .context("deserializing failed")?
        .sha)
}

pub async fn get_commit(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    sha: &str,
) -> anyhow::Result<github::Commit> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/commits/{}", repository_url, sha);
    let commit_response = client
        .get(&url)
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("cannot get commit")?;
    let commit_response = match commit_response.text().await {
        Ok(c) => c,
        Err(err) => {
            anyhow::bail!("Failed to decode response for {}: {:?}", url, err);
        }
    };
    match serde_json::from_str(&commit_response) {
        Ok(c) => Ok(c),
        Err(e) => Err(anyhow::anyhow!(
            "cannot deserialize commit ({}): {:?}",
            commit_response,
            e
        )),
    }
}

async fn enqueue_sha(
    request: github::Request,
    ctxt: &SiteCtxt,
    commit: String,
) -> ServerResult<github::Response> {
    let client = reqwest::Client::new();
    let commit_response = get_commit(&client, ctxt, &request.issue.repository_url, &commit)
        .await
        .map_err(|e| e.to_string())?;
    if commit_response.parents.len() != 2 {
        log::error!(
            "Bors try commit {} unexpectedly has {} parents.",
            commit_response.sha,
            commit_response.parents.len()
        );
        return Ok(github::Response);
    }
    let try_commit = TryCommit {
        sha: commit_response.sha.clone(),
        parent_sha: commit_response.parents[0].sha.clone(),
        issue: request.issue.clone(),
    };
    let queued = {
        let conn = ctxt.conn().await;
        conn.pr_attach_commit(
            request.issue.number,
            &commit_response.sha,
            &commit_response.parents[0].sha,
        )
        .await
    };
    if queued {
        let msg = format!(
            "Queued {} with parent {}, future [comparison URL]({}).",
            commit_response.sha,
            commit_response.parents[0].sha,
            try_commit.comparison_url(),
        );
        post_comment(&ctxt.config, request.issue.number, msg).await;
    }
    Ok(github::Response)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum HomuComment {
    TryBuildCompleted { merge_sha: String },
}

async fn handle_homu_res(request: &github::Request) -> Option<String> {
    if !request.comment.body.contains("Try build successful") {
        return None;
    }

    let start = "<!-- homu: ";
    let start_idx = request.comment.body.find(start).expect("found homu") + start.len();
    let end_idx = start_idx + request.comment.body[start_idx..].find(" -->").unwrap();

    let sha = match serde_json::from_str(&request.comment.body[start_idx..end_idx]) {
        Ok(HomuComment::TryBuildCompleted { merge_sha }) => merge_sha,
        Err(err) => {
            log::warn!(
                "failed to parse try build result; comment: {:?}, part: {:?}, err: {:?}",
                request.comment.body,
                &request.comment.body[start_idx..end_idx],
                err
            );
            return None;
        }
    };

    Some(sha)
}

pub async fn post_comment<B>(cfg: &Config, pr: u32, body: B)
where
    B: Into<String>,
{
    let body = body.into();
    let timer_token = cfg
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let client = reqwest::Client::new();
    let req = client
        .post(&format!(
            "https://api.github.com/repos/rust-lang/rust/issues/{}/comments",
            pr
        ))
        .json(&github::PostComment {
            body: body.to_owned(),
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token));

    if let Err(e) = req.send().await {
        eprintln!("failed to post comment: {:?}", e);
    }
}

pub async fn post_finished(ctxt: &SiteCtxt) {
    // If the github token is not configured, do not run this -- we don't want
    // to mark things as complete without posting the comment.
    if ctxt.config.keys.github_api_token.is_none() {
        return;
    }
    let conn = ctxt.conn().await;
    let index = ctxt.index.load();
    let mut commits = index
        .commits()
        .into_iter()
        .map(|c| c.sha.to_string())
        .collect::<HashSet<_>>();
    let queued = conn.queued_commits().await;

    for aid in conn.in_progress_artifacts().await {
        match aid {
            ArtifactId::Commit(c) => {
                commits.remove(&c.sha);
            }
            ArtifactId::Artifact(_) => {
                // do nothing, for now, though eventually we'll want an artifact
                // queue
            }
        }
    }
    for commit in queued {
        if !commits.contains(&commit.sha) {
            continue;
        }

        // This commit has been benchmarked.

        if let Some(completed) = conn.mark_complete(&commit.sha).await {
            assert_eq!(completed, commit);

            let comparison_url = format!(
                "https://perf.rust-lang.org/compare.html?start={}&end={}",
                commit.parent_sha, commit.sha
            );
            let (summary, direction) = categorize_benchmark(&commit, ctxt).await;
            let label = match direction {
                Some(Direction::Regression | Direction::Mixed) => "+perf-regression",
                Some(Direction::Improvement) | None => "-perf-regression",
            };
            let msg = direction
                .map(|d| {
                    format!(
                        "While you can manually mark this PR as fit \
            for rollup, we strongly recommend not doing so since this PR led to changes in \
            compiler perf.{}",
                        match d {
                            Direction::Regression | Direction::Mixed =>
                                "\n\n**Next Steps**: If you can justify the \
                regressions found in this perf run, please indicate this with \
                `@rustbot label: +perf-regression-triaged` along with \
                sufficient written justification. If you cannot justify the regressions \
                please fix the regressions and do another perf run. If the next run shows \
                neutral or positive results, the label will be automatically removed.",
                            Direction::Improvement => "",
                        }
                    )
                })
                .unwrap_or(String::new());

            post_comment(
                &ctxt.config,
                commit.pr,
                format!(
                    "Finished benchmarking try commit ({}): [comparison url]({}).

**Summary**: {}

Benchmarking this pull request likely means that it is \
perf-sensitive, so we're automatically marking it as not fit \
for rolling up. {} 

@bors rollup=never
@rustbot label: +S-waiting-on-review -S-waiting-on-perf {}",
                    commit.sha, comparison_url, summary, msg, label
                ),
            )
            .await;
        }
    }
}

async fn categorize_benchmark(
    commit: &database::QueuedCommit,
    ctxt: &SiteCtxt,
) -> (String, Option<Direction>) {
    let comparison = match crate::comparison::compare(
        collector::Bound::Commit(commit.parent_sha.clone()),
        collector::Bound::Commit(commit.sha.clone()),
        "instructions:u".to_owned(),
        ctxt,
    )
    .await
    {
        Ok(Some(c)) => c,
        _ => return (String::from("ERROR categorizing benchmark run!"), None),
    };
    const DISAGREEMENT: &str = "If you disagree with this performance assessment, \
    please file an issue in [rust-lang/rustc-perf](https://github.com/rust-lang/rustc-perf/issues/new).";
    let (summary, direction) = match ComparisonSummary::summarize_comparison(&comparison) {
        Some(s) if s.direction().is_some() => {
            let direction = s.direction().unwrap();
            (s, direction)
        }
        _ => {
            return (
                format!(
                    "This benchmark run did not return any significant changes.\n\n{}",
                    DISAGREEMENT
                ),
                None,
            )
        }
    };

    let category = match direction {
        Direction::Improvement => "improvements 🎉",
        Direction::Regression => "regressions 😿",
        Direction::Mixed => "mixed results 🤷",
    };
    let mut result = format!(
        "This change led to significant {} in compiler performance.\n",
        category
    );
    for change in summary.ordered_changes() {
        write!(result, "- ").unwrap();
        change.summary_line(&mut result, None)
    }
    write!(result, "\n{}", DISAGREEMENT).unwrap();
    (result, Some(direction))
}

pub(crate) struct PullRequest {
    pub number: u64,
    pub title: String,
}

/// Fetch all merged PRs that are labeled with `perf-regression` and not `perf-regression-triaged`
pub(crate) async fn untriaged_perf_regressions() -> Result<Vec<PullRequest>, BoxedError> {
    let url = "https://api.github.com/search/issues?q=repo:rust-lang/rust+label:perf-regression+-label:perf-regression-triaged+is:merged".to_owned();
    let request = github_request(&url);
    let body = send_request(request).await?;
    Ok(body
        .get("items")
        .ok_or_else(malformed_json_error)?
        .as_array()
        .ok_or_else(malformed_json_error)?
        .iter()
        .map(|v| {
            let title = v
                .get("title")
                .ok_or_else(malformed_json_error)?
                .as_str()
                .ok_or_else(malformed_json_error)?
                .to_owned();
            let number = v
                .get("number")
                .ok_or_else(malformed_json_error)?
                .as_u64()
                .ok_or_else(malformed_json_error)?;
            Ok(PullRequest { title, number })
        })
        .collect::<Result<_, BoxedError>>()?)
}

/// Get the title of a PR with the given number
pub(crate) async fn pr_title(pr: u32) -> String {
    let url = format!("https://api.github.com/repos/rust-lang/rust/pulls/{}", pr);
    let request = github_request(&url);

    async fn send(request: reqwest::RequestBuilder) -> Result<String, BoxedError> {
        let body = send_request(request).await?;
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

async fn send_request(request: reqwest::RequestBuilder) -> Result<serde_json::Value, BoxedError> {
    Ok(request
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?)
}

fn malformed_json_error() -> String {
    "JSON was malformed".to_owned()
}
