use crate::api::{github, ServerResult};
use crate::github::{client, enqueue_sha, parse_homu_comment, rollup_pr_number, unroll_rollup};
use crate::load::SiteCtxt;

use std::sync::Arc;

use regex::Regex;

lazy_static::lazy_static! {
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?"#).unwrap();
    static ref BODY_QUEUE: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+queue(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?"#).unwrap();
}

pub async fn handle_github(
    request: github::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<github::Response> {
    log::info!("handle_github({:?})", request);
    match request {
        github::Request::Issue { issue, comment } => handle_issue(ctxt, issue, comment).await,
        github::Request::Push(p) => handle_push(ctxt, p).await,
    }
}

async fn handle_push(ctxt: Arc<SiteCtxt>, push: github::Push) -> ServerResult<github::Response> {
    let ci_client = client::Client::from_ctxt(
        &ctxt,
        "https://api.github.com/repos/rust-lang-ci/rust".to_owned(),
    );
    let main_repo_client = client::Client::from_ctxt(
        &ctxt,
        "https://api.github.com/repos/rust-lang/rust".to_owned(),
    );
    if push.r#ref != "refs/heads/master" || push.sender.login != "bors" {
        return Ok(github::Response);
    }
    let rollup_pr_number =
        match rollup_pr_number(&main_repo_client, &push.head_commit.message).await? {
            Some(pr) => pr,
            None => return Ok(github::Response),
        };

    let previous_master = push.before;
    let commits = push.commits;

    // GitHub webhooks have a timeout of 10 seconds, so we process this
    // in the background.
    tokio::spawn(async move {
        let rollup_merges = commits
            .iter()
            .rev()
            .skip(1) // skip the head commit
            .take_while(|c| c.message.starts_with("Rollup merge of "));
        let result = unroll_rollup(
            ci_client,
            main_repo_client,
            rollup_merges,
            &previous_master,
            rollup_pr_number,
        )
        .await;
        log::info!("Processing of rollup merge finished: {:#?}", result);
    });
    Ok(github::Response)
}

async fn handle_issue(
    ctxt: Arc<SiteCtxt>,
    issue: github::Issue,
    comment: github::Comment,
) -> ServerResult<github::Response> {
    let main_client = client::Client::from_ctxt(
        &ctxt,
        "https://api.github.com/repos/rust-lang/rust".to_owned(),
    );
    let ci_client = client::Client::from_ctxt(
        &ctxt,
        "https://api.github.com/repos/rust-lang-ci/rust".to_owned(),
    );
    if comment.body.contains(" homu: ") {
        if let Some(sha) = parse_homu_comment(&comment.body).await {
            enqueue_sha(&ctxt, &main_client, &ci_client, issue.number, sha).await?;
            return Ok(github::Response);
        }
    }

    if comment.body.contains("@rust-timer ") {
        return handle_rust_timer(ctxt, &main_client, &ci_client, comment, issue).await;
    }

    Ok(github::Response)
}

async fn handle_rust_timer(
    ctxt: Arc<SiteCtxt>,
    main_client: &client::Client,
    ci_client: &client::Client,
    comment: github::Comment,
    issue: github::Issue,
) -> ServerResult<github::Response> {
    if comment.author_association != github::Association::Owner
        && !get_authorized_users().await?.contains(&comment.user.id)
    {
        main_client
            .post_comment(
                issue.number,
                "Insufficient permissions to issue commands to rust-timer.",
            )
            .await;
        return Ok(github::Response);
    }

    if let Some(captures) = BODY_QUEUE.captures(&comment.body) {
        let include = captures.get(1).map(|v| v.as_str());
        let exclude = captures.get(2).map(|v| v.as_str());
        let runs = captures.get(3).and_then(|v| v.as_str().parse::<i32>().ok());
        {
            let conn = ctxt.conn().await;
            conn.queue_pr(issue.number, include, exclude, runs).await;
        }
        main_client
            .post_comment(
                issue.number,
                "Awaiting bors try build completion.

@rustbot label: +S-waiting-on-perf",
            )
            .await;
        return Ok(github::Response);
    }
    if let Some(captures) = BODY_TRY_COMMIT.captures(&comment.body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str().to_owned()) {
            let include = captures.get(2).map(|v| v.as_str());
            let exclude = captures.get(3).map(|v| v.as_str());
            let runs = captures.get(4).and_then(|v| v.as_str().parse::<i32>().ok());
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            {
                let conn = ctxt.conn().await;
                conn.queue_pr(issue.number, include, exclude, runs).await;
            }
            enqueue_sha(
                &ctxt,
                &main_client,
                &ci_client,
                issue.number,
                commit.to_owned(),
            )
            .await?;
            return Ok(github::Response);
        }
    }
    Ok(github::Response)
}

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
