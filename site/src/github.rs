use crate::api::github::Issue;
use crate::comparison::{
    write_summary_table, Comparison, ComparisonConfidence, ComparisonSummary, Direction,
};
use crate::load::{Config, SiteCtxt, TryCommit};

use anyhow::Context as _;
use database::{ArtifactId, Benchmark, QueuedCommit};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

use collector::category::Category;
use std::collections::{HashMap, HashSet};
use std::{fmt::Write, sync::Arc, time::Duration};

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub async fn get_authorized_users() -> Result<Vec<usize>, String> {
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

// Returns the PR number
pub async fn pr_and_try_for_rollup(
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

pub struct RollupBranch {
    pub master_base_sha: String,
    pub rolled_up_pr_number: u32,
    pub name: String,
}

pub async fn branch_for_rollup(
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
) -> anyhow::Result<Commit> {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: InnerCommit,
    pub parents: Vec<CommitParent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InnerCommit {
    #[serde(default)]
    pub message: String,
    pub tree: CommitTree,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitTree {
    pub sha: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitParent {
    pub sha: String,
}

pub async fn enqueue_sha(issue: Issue, ctxt: &SiteCtxt, commit: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let commit_response = get_commit(&client, ctxt, &issue.repository_url, &commit)
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
        issue: issue.clone(),
    };
    let queued = {
        let conn = ctxt.conn().await;
        conn.pr_attach_commit(
            issue.number,
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
        post_comment(&ctxt.config, issue.number, msg).await;
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
        .json(&PostComment {
            body: body.to_owned(),
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token));

    if let Err(e) = req.send().await {
        eprintln!("failed to post comment: {:?}", e);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostComment {
    pub body: String,
}

/// Post messages to GitHub for all queued commits that have
/// not yet been marked as completed.
pub async fn post_finished(ctxt: &SiteCtxt) {
    // If the github token is not configured, do not run this -- we don't want
    // to mark things as complete without posting the comment.
    if ctxt.config.keys.github_api_token.is_none() {
        return;
    }
    let conn = ctxt.conn().await;
    let index = ctxt.index.load();
    let mut known_commits = index
        .commits()
        .into_iter()
        .map(|c| c.sha.to_string())
        .collect::<HashSet<_>>();
    let (master_commits, queued_pr_commits, in_progress_artifacts) = futures::join!(
        collector::master_commits(),
        conn.queued_commits(),
        conn.in_progress_artifacts()
    );
    let master_commits = match master_commits {
        Ok(mcs) => mcs.into_iter().map(|c| c.sha).collect::<HashSet<_>>(),
        Err(e) => {
            log::error!("posting finished did not load master commits: {:?}", e);
            // If we can't fetch master commits, return.
            // We'll eventually try again later
            return;
        }
    };

    for aid in in_progress_artifacts {
        match aid {
            ArtifactId::Commit(c) => {
                known_commits.remove(&c.sha);
            }
            ArtifactId::Tag(_) => {
                // do nothing, for now, though eventually we'll want an artifact queue
            }
        }
    }
    for queued_commit in queued_pr_commits
        .into_iter()
        .filter(|c| known_commits.contains(&c.sha))
    {
        if let Some(completed) = conn.mark_complete(&queued_commit.sha).await {
            assert_eq!(completed, queued_commit);

            let is_master_commit = master_commits.contains(&queued_commit.sha);
            post_comparison_comment(ctxt, queued_commit, is_master_commit).await;
        }
    }
}

/// Posts a comment to GitHub summarizing the comparison of the queued commit with its parent
///
/// `is_master_commit` is used to differentiate messages for try runs and post-merge runs.
async fn post_comparison_comment(ctxt: &SiteCtxt, commit: QueuedCommit, is_master_commit: bool) {
    let comparison_url = format!(
        "https://perf.rust-lang.org/compare.html?start={}&end={}",
        commit.parent_sha, commit.sha
    );
    let (summary, direction) = categorize_benchmark(
        ctxt,
        commit.sha.clone(),
        commit.parent_sha,
        is_master_commit,
    )
    .await;

    let body = format!(
        "Finished benchmarking commit ({sha}): [comparison url]({url}).

**Summary**: {summary}
{rest}",
        sha = commit.sha,
        url = comparison_url,
        summary = summary,
        rest = if is_master_commit {
            master_run_body(direction)
        } else {
            try_run_body(direction)
        }
    );

    post_comment(&ctxt.config, commit.pr, body).await;
}

fn master_run_body(direction: Option<Direction>) -> String {
    let label = match direction {
        Some(Direction::Regression | Direction::Mixed) => "+perf-regression",
        Some(Direction::Improvement) | None => "-perf-regression",
    };
    let next_steps = match direction {
        Some(Direction::Regression | Direction::Mixed) => {
            "\n\n**Next Steps**: If you can justify the \
                regressions found in this perf run, please indicate this with \
                `@rustbot label: +perf-regression-triaged` along with \
                sufficient written justification. If you cannot justify the regressions \
                please open an issue or create a new PR that fixes the regressions, \
                add a comment linking to the newly created issue or PR, \
                and then add the `perf-regression-triaged` label to this PR."
        }
        Some(Direction::Improvement) | None => "",
    };

    format!(
        "
{next_steps}

@rustbot label: {label}",
        next_steps = next_steps,
        label = label
    )
}

fn try_run_body(direction: Option<Direction>) -> String {
    let label = match direction {
        Some(Direction::Regression | Direction::Mixed) => "+perf-regression",
        Some(Direction::Improvement) | None => "-perf-regression",
    };
    let next_steps = match direction {
        Some(Direction::Regression | Direction::Mixed) => {
            "\n\n**Next Steps**: If you can justify the regressions found in \
            this try perf run, please indicate this with \
            `@rustbot label: +perf-regression-triaged` along with \
            sufficient written justification. If you cannot justify the regressions \
            please fix the regressions and do another perf run. If the next run \
            shows neutral or positive results, the label will be automatically removed."
        }
        Some(Direction::Improvement) | None => "",
    };

    format!(
        "
Benchmarking this pull request likely means that it is \
perf-sensitive, so we're automatically marking it as not fit \
for rolling up. While you can manually mark this PR as fit \
for rollup, we strongly recommend not doing so since this PR led to changes in \
compiler perf.{next_steps}

@bors rollup=never
@rustbot label: +S-waiting-on-review -S-waiting-on-perf {label}",
        next_steps = next_steps,
        label = label
    )
}

pub type BenchmarkMap = HashMap<Benchmark, Category>;

async fn get_benchmark_map(ctxt: &SiteCtxt) -> BenchmarkMap {
    let benchmarks = ctxt.pool.connection().await.get_benchmarks().await;
    benchmarks
        .into_iter()
        .map(|bench| {
            (
                bench.name.as_str().into(),
                Category::from_db_representation(&bench.category).unwrap(),
            )
        })
        .collect()
}

async fn categorize_benchmark(
    ctxt: &SiteCtxt,
    commit_sha: String,
    parent_sha: String,
    is_master_commit: bool,
) -> (String, Option<Direction>) {
    let comparison = match crate::comparison::compare(
        collector::Bound::Commit(parent_sha),
        collector::Bound::Commit(commit_sha),
        "instructions:u".to_owned(),
        ctxt,
    )
    .await
    {
        Ok(Some(c)) => c,
        _ => return (String::from("ERROR categorizing benchmark run!"), None),
    };

    let benchmark_map = get_benchmark_map(ctxt).await;
    let (primary, secondary) = split_comparison(comparison, benchmark_map);

    const DISAGREEMENT: &str = "If you disagree with this performance assessment, \
    please file an issue in [rust-lang/rustc-perf](https://github.com/rust-lang/rustc-perf/issues/new).";

    if primary.is_none() && secondary.is_none() {
        return (
            format!(
                "This benchmark run did not return any relevant results.\n\n{}",
                DISAGREEMENT
            ),
            None,
        );
    }

    let (primary_short_summary, primary_direction) =
        generate_short_summary(primary.as_ref(), is_master_commit);
    let (secondary_short_summary, secondary_direction) =
        generate_short_summary(secondary.as_ref(), is_master_commit);

    let mut result = format!(
        r#"
- Primary benchmarks: {primary_short_summary}
- Secondary benchmarks: {secondary_short_summary}
"#
    );
    write!(result, "\n\n").unwrap();

    if primary_direction.is_some() || secondary_direction.is_some() {
        let (primary, secondary) = (
            primary.unwrap_or_else(|| ComparisonSummary::empty()),
            secondary.unwrap_or_else(|| ComparisonSummary::empty()),
        );
        write_summary_table(&primary, &secondary, &mut result);
    }

    write!(result, "\n{}", DISAGREEMENT).unwrap();

    let direction = primary_direction.or(secondary_direction);
    (result, direction)
}

fn generate_short_summary(
    summary: Option<&ComparisonSummary>,
    is_master_commit: bool,
) -> (String, Option<Direction>) {
    // Add an "s" to a word unless there's only one.
    fn ending(word: &'static str, count: usize) -> std::borrow::Cow<'static, str> {
        if count == 1 {
            return word.into();
        }
        format!("{}s", word).into()
    }

    match summary {
        Some(summary) => {
            if comparison_is_relevant(summary.confidence(), is_master_commit) {
                let direction = summary.direction().unwrap();
                let num_improvements = summary.number_of_improvements();
                let num_regressions = summary.number_of_regressions();

                let text = match direction {
                    Direction::Improvement => format!(
                        "🎉 relevant {} found",
                        ending("improvement", num_improvements)
                    ),
                    Direction::Regression => format!(
                        "😿 relevant {} found",
                        ending("regression", num_regressions)
                    ),
                    Direction::Mixed => "mixed results".to_string(),
                };
                (text, Some(direction))
            } else {
                (
                    format!(
                        "no relevant changes found. {} results were found to be statistically significant but too small to be relevant.",
                        summary.num_significant_changes(),
                    ),
                    None
                )
            }
        }
        None => ("no relevant changes found".to_string(), None),
    }
}

/// Splits comparison into primary and secondary summaries based on benchmark category
fn split_comparison(
    comparison: Comparison,
    map: BenchmarkMap,
) -> (Option<ComparisonSummary>, Option<ComparisonSummary>) {
    let mut primary = HashSet::new();
    let mut secondary = HashSet::new();

    for statistic in comparison.statistics {
        let category: Category = map
            .get(&statistic.benchmark())
            .copied()
            .unwrap_or_else(|| Category::Secondary);
        if let Category::Primary = category {
            primary.insert(statistic);
        } else {
            secondary.insert(statistic);
        }
    }

    let primary = Comparison {
        a: comparison.a.clone(),
        b: comparison.b.clone(),
        statistics: primary,
        new_errors: comparison.new_errors.clone(),
    };
    let secondary = Comparison {
        a: comparison.a,
        b: comparison.b,
        statistics: secondary,
        new_errors: comparison.new_errors,
    };
    (
        ComparisonSummary::summarize_comparison(&primary),
        ComparisonSummary::summarize_comparison(&secondary),
    )
}

/// Whether a comparison is relevant enough to show
fn comparison_is_relevant(confidence: ComparisonConfidence, is_master_commit: bool) -> bool {
    if is_master_commit {
        confidence.is_definitely_relevant()
    } else {
        // is try run
        confidence.is_atleast_probably_relevant()
    }
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
