use crate::api::{github, ServerResult};
use crate::github::{
    branch_for_rollup, client, enqueue_sha, get_authorized_users, parse_homu_comment,
    pr_and_try_for_rollup,
};
use crate::load::SiteCtxt;

use std::sync::Arc;

use regex::{Captures, Regex};

lazy_static::lazy_static! {
    static ref ROLLUP_PR_NUMBER: Regex =
        Regex::new(r#"^Auto merge of #(\d+)"#).unwrap();
    static ref ROLLUPED_PR_NUMBER: Regex =
        Regex::new(r#"^Rollup merge of #(\d+)"#).unwrap();
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?"#).unwrap();
    static ref BODY_QUEUE: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+queue(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?"#).unwrap();
    static ref BODY_MAKE_PR_FOR: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+make-pr-for\s+([\w:/\.\-]+)(?:\W|$)"#).unwrap();
    static ref BODY_UDPATE_PR_FOR: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+update-branch-for\s+([\w:/\.\-]+)(?:\W|$)"#).unwrap();
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
    let client = reqwest::Client::new();
    let repository_url = "https://github.com/rust-lang-ci/rust";
    if push.r#ref != "refs/heads/master" {
        return Ok(github::Response);
    }
    let pr = rollup_pr(&client, &ctxt, repository_url, &push).await?;
    let pr = match pr {
        Some(pr) => pr,
        None => return Ok(github::Response),
    };

    let previous_master = &push.before;
    let rollup_merges = push
        .commits
        .iter()
        .rev()
        .skip(1) // skip the head commit
        .take_while(|c| c.message.starts_with("Rollup merge of "));

    let mut prs = Vec::new();
    for rollup_merge in rollup_merges {
        let pr_num = ROLLUPED_PR_NUMBER
            .captures(&rollup_merge.message)
            .and_then(|c| c.get(0))
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
        let commit = client::get_commit(&client, &ctxt, repository_url, &rollup_merge.sha)
            .await
            .map_err(|e| {
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
        client::update_branch(&client, &ctxt, repository_url, "perf-tmp", previous_master)
            .await
            .map_err(|e| format!("Error updating perf-tmp with previous master: {e:?}"))?;

        // Merge in the rolled up PR's head commit into the previous master
        let sha = client::merge_branch(
            &client,
            &ctxt,
            repository_url,
            "perf-tmp",
            rolled_up_head,
            "merge",
        )
        .await
        .map_err(|e| format!("Error merging commit into perf-tmp: {e:?}"))?;

        // Force the `try-perf` branch to point to what the perf-tmp branch points to
        client::update_branch(&client, &ctxt, repository_url, "try-perf", &sha)
            .await
            .map_err(|e| format!("Error updating the try-perf branch: {e:?}"))?;

        prs.push((pr_num, sha));
        // Wait to ensure there's enough time for GitHub to checkout these changes before they are overwritten
        tokio::time::sleep(std::time::Duration::from_secs(15)).await
    }

    // Post comment to the rollup PR with the mapping between individual PRs and the new try commits
    let mapping = prs
        .into_iter()
        .fold(String::new(), |mut string, (pr, commit)| {
            use std::fmt::Write;
            write!(&mut string, "#{pr}: {commit}\n").unwrap();
            string
        });
    let msg =
        format!("Try perf builds for each individual rolled up PR have been enqueued:\n{mapping}");
    client::post_comment(&ctxt.config, pr, msg).await;
    Ok(github::Response)
}

// Gets the pr number for the associated rollup PR. Returns None if this is not a rollup PR
async fn rollup_pr(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    push: &github::Push,
) -> ServerResult<Option<u32>> {
    macro_rules! get {
        ($x:expr) => {
            match $x {
                Some(x) => x,
                None => return Ok(None),
            }
        };
    }
    let is_bors =
        push.sender.login == "bors" && push.head_commit.message.starts_with("Auto merge of");

    if !is_bors {
        return Ok(None);
    }
    let captures = get!(ROLLUP_PR_NUMBER.captures(&push.head_commit.message));
    let number = get!(get!(captures.get(0)).as_str().parse::<u64>().ok());

    let issue = client::get_issue(client, ctxt, repository_url, number)
        .await
        .map_err(|e| format!("Error fetching PR #{number} {e:?}"))?;

    Ok(issue
        .labels
        .iter()
        .any(|l| l.name == "rollup")
        .then(|| issue.number))
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
    if comment.author_association != github::Association::Owner
        && !get_authorized_users().await?.contains(&comment.user.id)
    {
        client::post_comment(
            &ctxt.config,
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
        client::post_comment(
            &ctxt.config,
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
    for rollup_merge in extract_make_pr_for(&comment.body) {
        let client = reqwest::Client::new();
        pr_and_try_for_rollup(
            &client,
            ctxt.clone(),
            &issue.repository_url,
            &rollup_merge,
            &comment.html_url,
        )
        .await
        .map_err(|e| format!("{:?}", e))?;
    }
    for rollup_merge in extract_update_pr_for(&comment.body) {
        // This just creates or updates the branch for this merge commit.
        // Intended for resolving the race condition of master merging in
        // between us updating the commit and merging things.
        let client = reqwest::Client::new();
        let branch = branch_for_rollup(&client, &ctxt, &issue.repository_url, rollup_merge)
            .await
            .map_err(|e| e.to_string())?;
        client::post_comment(
            &ctxt.config,
            issue.number,
            &format!("Master base SHA: {}", branch.master_base_sha),
        )
        .await;
    }
    Ok(github::Response)
}

fn extract_make_pr_for(body: &str) -> impl Iterator<Item = &str> + '_ {
    BODY_MAKE_PR_FOR
        .captures_iter(body)
        .filter_map(|c| extract_rollup_merge(c))
}

fn extract_update_pr_for(body: &str) -> impl Iterator<Item = &str> + '_ {
    BODY_UDPATE_PR_FOR
        .captures_iter(body)
        .filter_map(|c| extract_rollup_merge(c))
}

fn extract_rollup_merge(capture: Captures) -> Option<&str> {
    capture.get(1).map(|c| {
        c.as_str()
            .trim_start_matches("https://github.com/rust-lang/rust/commit/")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_the_right_sha() {
        let message = r#"This is a message.

        @rust-timer make-pr-for https://github.com/rust-lang/rust/commit/857afc75e6ca69cc7dcae36a6fac8c093ee6fa31
        @rust-timer make-pr-for https://github.com/rust-lang/rust/commit/857afc75e6ca69cc7dcae36a6fac8c093ee6fa31
        "#;

        let mut iter = extract_make_pr_for(message);
        assert_eq!(
            iter.next().unwrap(),
            "857afc75e6ca69cc7dcae36a6fac8c093ee6fa31",
            "sha did not match"
        );
        assert_eq!(
            iter.next().unwrap(),
            "857afc75e6ca69cc7dcae36a6fac8c093ee6fa31",
            "sha did not match"
        );
        assert!(iter.next().is_none(), "there were more rollup merges");
        let message = r#"This is a message.

        @rust-timer update-branch-for https://github.com/rust-lang/rust/commit/857afc75e6ca69cc7dcae36a6fac8c093ee6fa31"#;

        let mut iter = extract_update_pr_for(message);
        let sha = iter.next().unwrap();
        println!("{sha}");
        assert_eq!(
            sha, "857afc75e6ca69cc7dcae36a6fac8c093ee6fa31",
            "sha did not match"
        );
        assert!(iter.next().is_none(), "there were more rollup merges");
    }
}
