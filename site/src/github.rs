use crate::api::{github, ServerResult};
use crate::load::{Config, InputData, TryCommit};
use futures::future::Future;
use regex::Regex;
use reqwest::header::USER_AGENT;

lazy_static::lazy_static! {
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)"#).unwrap();
}

fn get_authorized_users() -> ServerResult<Vec<usize>> {
    let url = format!("{}/permissions/perf.json", ::rust_team_data::v1::BASE_URL);
    let client = reqwest::r#async::Client::new();
    let perms: ::rust_team_data::v1::Permission = wait_for_future(client.get(&url).send())
        .and_then(|resp| resp.error_for_status())
        .and_then(|mut resp| wait_for_future(resp.json()))
        .map_err(|err| format!("failed to fetch authorized users: {}", err))?;
    Ok(perms.github_ids)
}

pub fn handle_github(request: github::Request, data: &InputData) -> ServerResult<github::Response> {
    if !request.comment.body.contains("@rust-timer ") {
        return Ok(github::Response);
    }

    if request.comment.author_association != github::Association::Owner
        && !get_authorized_users()?.contains(&request.comment.user.id)
    {
        post_comment(
            &data.config,
            &request.issue,
            "Insufficient permissions to issue commands to rust-timer.",
        )?;
        return Ok(github::Response);
    }

    let body = &request.comment.body;

    if let Some(captures) = BODY_TRY_COMMIT.captures(&body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str()) {
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            let client = reqwest::r#async::Client::new();
            let commit_response: github::Commit = wait_for_future(
                wait_for_future(
                    client
                        .get(&format!(
                            "{}/commits/{}",
                            request.issue.repository_url, commit
                        ))
                        .send()
                        .map_err(|_| String::from("cannot get commit")),
                )?
                .json()
                .map_err(|e| format!("cannot deserialize commit: {:?}", e)),
            )?;
            if commit_response.parents.len() != 2 {
                post_comment(
                    &data.config,
                    &request.issue,
                    &format!(
                        "Bors try commit {} unexpectedly has {} parents.",
                        commit_response.sha,
                        commit_response.parents.len()
                    ),
                )?;
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
                &format!(
                    "Success: Queued {} with parent {}, [comparison URL]({}).",
                    commit_response.sha,
                    commit_response.parents[0].sha,
                    try_commit.comparison_url(),
                ),
            )?;
        }
    }

    Ok(github::Response)
}

fn wait_for_future<
    T: 'static + Send,
    E: 'static + Send,
    F: 'static + Send + Future<Item = T, Error = E>,
>(
    f: F,
) -> Result<T, E> {
    std::thread::spawn(move || f.wait()).join().unwrap()
}

pub fn post_comment(cfg: &Config, issue: &github::Issue, body: &str) -> ServerResult<()> {
    let timer_token = cfg.keys.github.clone().expect("needs rust-timer token");
    let client = reqwest::r#async::Client::new();
    let req = client
        .post(&issue.comments_url)
        .json(&github::PostComment {
            body: body.to_owned(),
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token));

    let res = wait_for_future(req.send());
    match res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to post comment: {:?}", err);
        }
    }
    Ok(())
}
