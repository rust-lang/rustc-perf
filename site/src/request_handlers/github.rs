use crate::api::{github, ServerResult};
use crate::github::{
    branch_for_rollup, enqueue_sha, get_authorized_users, parse_homu_comment, post_comment,
    pr_and_try_for_rollup,
};
use crate::load::SiteCtxt;

use std::sync::Arc;

use regex::Regex;

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

pub async fn handle_github(
    request: github::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<github::Response> {
    log::info!("handle_github({:?})", request);
    if request.comment.body.contains(" homu: ") {
        if let Some(sha) = parse_homu_comment(&request.comment.body).await {
            enqueue_sha(request.issue, &ctxt, sha).await?;
            return Ok(github::Response);
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
            enqueue_sha(request.issue, &ctxt, commit.to_owned()).await?;
            return Ok(github::Response);
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
