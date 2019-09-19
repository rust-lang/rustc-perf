use crate::api::{github, ServerResult};
use crate::load::{Config, InputData, TryCommit};
use futures::compat::Future01CompatExt;
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
    let client = reqwest::r#async::Client::new();
    client
        .get(&url)
        .send()
        .compat()
        .await
        .map_err(|err| format!("failed to fetch authorized users: {}", err))?
        .error_for_status()
        .map_err(|err| format!("failed to fetch authorized users: {}", err))?
        .json::<rust_team_data::v1::Permission>()
        .compat()
        .await
        .map_err(|err| format!("failed to fetch authorized users: {}", err))
        .map(|perms| perms.github_ids)
}

pub async fn handle_github(
    request: github::Request,
    data: &InputData,
) -> ServerResult<github::Response> {
    if request.comment.body.contains(" homu: ") {
        if let Some(sha) = handle_homu_res(&request, data).await {
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
            &request.issue,
            "Insufficient permissions to issue commands to rust-timer.",
        )
        .await;
        return Ok(github::Response);
    }

    if BODY_QUEUE.is_match(&request.comment.body) {
        {
            let mut persistent = data.persistent.lock();
            persistent.pending_try_builds.insert(request.issue.number);
            persistent.write().expect("successful encode");
        }
        post_comment(
            &data.config,
            &request.issue,
            "Awaiting bors try build completion",
        )
        .await;
        return Ok(github::Response);
    }

    if let Some(captures) = BODY_TRY_COMMIT.captures(&request.comment.body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str().to_owned()) {
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
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
    let client = reqwest::r#async::Client::new();
    let url = format!("{}/commits/{}", request.issue.repository_url, commit);
    let mut commit_response = client
        .get(&url)
        .send()
        .compat()
        .await
        .map_err(|_| String::from("cannot get commit"))?;
    let commit_response = commit_response.json::<github::Commit>().compat().await;
    let commit_response = match commit_response {
        Err(e) => return Err(format!("cannot deserialize commit: {:?}", e)),
        Ok(c) => c,
    };
    if commit_response.parents.len() != 2 {
        let msg = format!(
            "Bors try commit {} unexpectedly has {} parents.",
            commit_response.sha,
            commit_response.parents.len()
        );
        post_comment(&data.config, &request.issue, msg).await;
        return Ok(github::Response);
    }
    let try_commit = TryCommit {
        sha: commit_response.sha.clone(),
        parent_sha: commit_response.parents[0].sha.clone(),
        issue: request.issue.clone(),
    };
    {
        let mut persistent = data.persistent.lock();
        if !persistent
            .try_commits
            .iter()
            .any(|c| c.sha() == &commit_response.sha)
        {
            persistent.try_commits.push(try_commit.clone());
        }
        persistent.write().expect("successful encode");
    }
    let msg = format!(
        "Queued {} with parent {}, future [comparison URL]({}).",
        commit_response.sha,
        commit_response.parents[0].sha,
        try_commit.comparison_url(),
    );
    post_comment(&data.config, &request.issue, msg).await;
    Ok(github::Response)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum HomuComment {
    TryBuildCompleted { merge_sha: String },
}

async fn handle_homu_res(request: &github::Request, data: &InputData) -> Option<String> {
    if !request.comment.body.contains("Try build successful") {
        return None;
    }

    {
        let mut persistent = data.persistent.lock();
        if persistent.pending_try_builds.remove(&request.issue.number) {
            persistent.write().expect("successful encode");
        } else {
            log::debug!(
                "Skipping successful try build for pr {}, not in pending",
                request.issue.number
            );
            return None;
        }
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

pub async fn post_comment<B>(cfg: &Config, issue: &github::Issue, body: B)
where
    B: Into<String>,
{
    let body = body.into();
    let timer_token = cfg.keys.github.clone().expect("needs rust-timer token");
    let client = reqwest::r#async::Client::new();
    let req = client
        .post(&issue.comments_url)
        .json(&github::PostComment {
            body: body.to_owned(),
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token));

    if let Err(e) = req.send().compat().await {
        eprintln!("failed to post comment: {:?}", e);
    }
}
