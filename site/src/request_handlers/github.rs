use crate::api::{github, ServerResult};
use crate::github::{
    client, enqueue_sha, enqueue_unrolled_try_builds, parse_homu_comment, rollup_pr_number,
};
use crate::load::SiteCtxt;

use std::sync::Arc;

use regex::Regex;

lazy_static::lazy_static! {
    static ref ROLLUP_PR_NUMBER: Regex =
        Regex::new(r#"^Auto merge of #(\d+)"#).unwrap();
    static ref ROLLEDUP_PR_NUMBER: Regex =
        Regex::new(r#"^Rollup merge of #(\d+)"#).unwrap();
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
        let result = handle_rollup_merge(
            ci_client,
            main_repo_client,
            commits,
            &previous_master,
            rollup_pr_number,
        )
        .await;
        log::info!("Processing of rollup merge finished: {:#?}", result);
    });
    Ok(github::Response)
}

/// Handler for when a rollup has been merged
async fn handle_rollup_merge(
    ci_client: client::Client,
    main_repo_client: client::Client,
    commits: Vec<github::Commit>,
    previous_master: &str,
    rollup_pr_number: u32,
) -> Result<(), String> {
    let rollup_merges = commits
        .iter()
        .rev()
        .skip(1) // skip the head commit
        .take_while(|c| c.message.starts_with("Rollup merge of "));
    let mapping = enqueue_unrolled_try_builds(ci_client, rollup_merges, previous_master).await?;
    let mapping = mapping
        .into_iter()
        .map(|(rollup_merge, sha)| {
            ROLLEDUP_PR_NUMBER
                .captures(&rollup_merge.message)
                .and_then(|c| c.get(1))
                .map(|m| (m.as_str(), sha))
                .ok_or_else(|| {
                    format!(
                        "Could not get PR number from message: '{}'",
                        rollup_merge.message
                    )
                })
        })
        .fold(ServerResult::Ok(String::new()), |string, n| {
            use std::fmt::Write;
            let (pr, commit) = n?;
            let mut string = string?;
            write!(
                &mut string,
                "|#{pr}|[{commit}](https://github.com/rust-lang-ci/rust/commit/{commit})|\n"
            )
            .unwrap();
            Ok(string)
        })?;
    let msg =
        format!("📌 Perf builds for each rolled up PR:\n\n\
        |PR# | Perf Build Sha|\n|----|-----|\n\
        {mapping}\nIn the case of a perf regression, \
        run the following command for each PR you suspect might be the cause: `@rust-timer build $SHA`");
    main_repo_client.post_comment(rollup_pr_number, msg).await;
    Ok(())
}

async fn handle_issue(
    ctxt: Arc<SiteCtxt>,
    issue: github::Issue,
    comment: github::Comment,
) -> ServerResult<github::Response> {
    if comment.body.contains(" homu: ") {
        if let Some(sha) = parse_homu_comment(&comment.body).await {
            enqueue_sha(issue, &ctxt, sha).await?;
            return Ok(github::Response);
        }
    }

    if comment.body.contains("@rust-timer ") {
        return handle_rust_timer(ctxt, comment, issue).await;
    }

    Ok(github::Response)
}

async fn handle_rust_timer(
    ctxt: Arc<SiteCtxt>,
    comment: github::Comment,
    issue: github::Issue,
) -> ServerResult<github::Response> {
    let main_repo_client = client::Client::from_ctxt(
        &ctxt,
        "https://api.github.com/repos/rust-lang/rust".to_owned(),
    );
    if comment.author_association != github::Association::Owner
        && !get_authorized_users().await?.contains(&comment.user.id)
    {
        main_repo_client
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
        main_repo_client
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
            enqueue_sha(issue, &ctxt, commit.to_owned()).await?;
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
