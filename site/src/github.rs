use crate::api::{github, ServerResult};
use crate::load::{Config, InputData, TryCommit};
use futures::compat::Future01CompatExt;

use regex::Regex;
use reqwest::header::USER_AGENT;

lazy_static::lazy_static! {
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)"#).unwrap();
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

    let body = &request.comment.body;

    if let Some(captures) = BODY_TRY_COMMIT.captures(&body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str()) {
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            let client = reqwest::r#async::Client::new();
            let mut commit_response = client
                .get(&format!(
                    "{}/commits/{}",
                    request.issue.repository_url, commit
                ))
                .send()
                .compat()
                .await
                .map_err(|_| String::from("cannot get commit"))?;
            let commit_response = commit_response
                .json::<github::Commit>()
                .compat()
                .await
                .map_err(|e| format!("cannot deserialize commit: {:?}", e))?;
            if commit_response.parents.len() != 2 {
                post_comment(
                    &data.config,
                    &request.issue,
                    format!(
                        "Bors try commit {} unexpectedly has {} parents.",
                        commit_response.sha,
                        commit_response.parents.len()
                    ),
                )
                .await;
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
            post_comment(
                &data.config,
                &request.issue,
                format!(
                    "Success: Queued {} with parent {}, [comparison URL]({}).",
                    commit_response.sha,
                    commit_response.parents[0].sha,
                    try_commit.comparison_url(),
                ),
            )
            .await;
        }
    }

    Ok(github::Response)
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
