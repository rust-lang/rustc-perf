use crate::api::{github, ServerResult};
use crate::load::{Config, InputData, TryCommit};
use hashbrown::HashSet;
use serde::Deserialize;

use regex::Regex;
use reqwest::header::USER_AGENT;

lazy_static::lazy_static! {
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)"#).unwrap();
    static ref BODY_QUEUE: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+queue(?:\W|$)"#).unwrap();
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
    data: &InputData,
) -> ServerResult<github::Response> {
    if request.comment.body.contains(" homu: ") {
        if let Some(sha) = handle_homu_res(&request).await {
            return enqueue_sha(request, data, sha).await;
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
            &data.config,
            request.issue.number,
            "Insufficient permissions to issue commands to rust-timer.",
        )
        .await;
        return Ok(github::Response);
    }

    if BODY_QUEUE.is_match(&request.comment.body) {
        {
            let conn = data.conn().await;
            conn.queue_pr(request.issue.number).await;
        }
        post_comment(
            &data.config,
            request.issue.number,
            "Awaiting bors try build completion",
        )
        .await;
        return Ok(github::Response);
    }

    if let Some(captures) = BODY_TRY_COMMIT.captures(&request.comment.body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str().to_owned()) {
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            {
                let conn = data.conn().await;
                conn.queue_pr(request.issue.number).await;
            }
            let f = enqueue_sha(request, data, commit.to_owned());
            return f.await;
        }
    }

    Ok(github::Response)
}

async fn enqueue_sha(
    request: github::Request,
    data: &InputData,
    commit: String,
) -> ServerResult<github::Response> {
    let timer_token = data
        .config
        .keys
        .github
        .clone()
        .expect("needs rust-timer token");
    let client = reqwest::Client::new();
    let url = format!("{}/commits/{}", request.issue.repository_url, commit);
    let commit_response = client
        .get(&url)
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .map_err(|_| String::from("cannot get commit"))?;
    let commit_response = match commit_response.text().await {
        Ok(c) => c,
        Err(err) => {
            return Err(format!("Failed to decode response for {}: {:?}", url, err));
        }
    };
    let commit_response: github::Commit = match serde_json::from_str(&commit_response) {
        Ok(c) => c,
        Err(e) => {
            return Err(format!(
                "cannot deserialize commit ({:?}): {:?}",
                commit_response, e
            ));
        }
    };
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
        let conn = data.conn().await;
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
        post_comment(&data.config, request.issue.number, msg).await;
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
    let timer_token = cfg.keys.github.clone().expect("needs rust-timer token");
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

pub async fn post_finished(data: &InputData) {
    let conn = data.conn().await;
    let index = data.index.load();
    let commits = index
        .commits()
        .into_iter()
        .map(|c| c.sha.to_string())
        .collect::<HashSet<_>>();
    let queued = conn.queued_commits().await;

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
            post_comment(
                &data.config,
                commit.pr,
                format!(
                    "Finished benchmarking try commit ({}): [comparison url]({}).

                    Benchmarking this pull request likely means that it is \
                    perf-sensitive, so we're automatically marking it as not fit \
                    for rolling up. Please note that if the perf results are \
                    neutral, you should likely undo the rollup=never given below \
                    by specifying `rollup-` to bors.

                    Importantly, though, if the results of this run are \
                    non-neutral **do not** roll this PR up -- it will mask other \
                    regressions or improvements in the roll up.

                    @bors rollup=never",
                    commit.sha, comparison_url
                ),
            )
            .await;
        }
    }
}
