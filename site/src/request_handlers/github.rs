use crate::api::{github, ServerResult};
use crate::github::{
    client, enqueue_shas, parse_homu_comment, rollup_pr_number, unroll_rollup,
    COMMENT_MARK_TEMPORARY, RUST_REPO_GITHUB_API_URL,
};
use crate::load::SiteCtxt;

use hashbrown::HashMap;
use regex::Regex;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref BODY_TIMER_BUILD: Regex =
        Regex::new(r"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)(?:include=(\S+))?\s*(?:exclude=(\S+))?\s*(?:runs=(\d+))?").unwrap();
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
    let main_repo_client = client::Client::from_ctxt(&ctxt, RUST_REPO_GITHUB_API_URL.to_owned());
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
            .filter(|c| c.message.starts_with("Rollup merge of #"));
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
    let main_client = client::Client::from_ctxt(&ctxt, RUST_REPO_GITHUB_API_URL.to_owned());
    let ci_client = client::Client::from_ctxt(
        &ctxt,
        "https://api.github.com/repos/rust-lang-ci/rust".to_owned(),
    );
    if comment.body.contains(" homu: ") {
        if let Some(sha) = parse_homu_comment(&comment.body).await {
            enqueue_shas(
                &ctxt,
                &main_client,
                &ci_client,
                issue.number,
                std::iter::once(sha.as_str()),
            )
            .await?;
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
                format!(
                    "Insufficient permissions to issue commands to rust-timer.
{COMMENT_MARK_TEMPORARY}"
                ),
            )
            .await;
        return Ok(github::Response);
    }

    if let Some(queue) = parse_queue_command(&comment.body) {
        let msg = match queue {
            Ok(cmd) => {
                let conn = ctxt.conn().await;
                conn.queue_pr(
                    issue.number,
                    cmd.params.include,
                    cmd.params.exclude,
                    cmd.params.runs,
                )
                .await;
                format!(
                    "Awaiting bors try build completion.

@rustbot label: +S-waiting-on-perf

{COMMENT_MARK_TEMPORARY}"
                )
            }
            Err(error) => {
                format!("Error occurred while parsing comment: {error}")
            }
        };
        main_client.post_comment(issue.number, msg).await;
        return Ok(github::Response);
    }

    for captures in build_captures(&comment.body).map(|(_, captures)| captures) {
        let include = captures.get(2).map(|v| v.as_str());
        let exclude = captures.get(3).map(|v| v.as_str());
        let runs = captures.get(4).and_then(|v| v.as_str().parse::<i32>().ok());
        {
            let conn = ctxt.conn().await;
            conn.queue_pr(issue.number, include, exclude, runs).await;
        }
    }

    enqueue_shas(
        &ctxt,
        main_client,
        ci_client,
        issue.number,
        build_captures(&comment.body).map(|(commit, _)| commit),
    )
    .await?;

    Ok(github::Response)
}

/// Parses the first occurrence of a `@rust-timer queue <shared-args>` command
/// in the input string.
fn parse_queue_command(body: &str) -> Option<Result<QueueCommand, String>> {
    let args = get_command_lines(body, "queue").next()?;
    let args = match parse_command_arguments(args) {
        Ok(args) => args,
        Err(error) => return Some(Err(error)),
    };
    let params = match parse_benchmark_parameters(args) {
        Ok(params) => params,
        Err(error) => return Some(Err(error)),
    };

    Some(Ok(QueueCommand { params }))
}

fn get_command_lines<'a: 'b, 'b>(
    body: &'a str,
    command: &'b str,
) -> impl Iterator<Item = &'a str> + 'b {
    let prefix = "@rust-timer";
    body.lines()
        .filter_map(move |line| {
            line.find(prefix)
                .map(|index| line[index + prefix.len()..].trim())
        })
        .filter_map(move |line| line.strip_prefix(command))
        .map(move |l| l.trim())
}

fn parse_benchmark_parameters<'a>(
    mut args: HashMap<&'a str, &'a str>,
) -> Result<BenchmarkParameters<'a>, String> {
    let mut params = BenchmarkParameters {
        include: args.remove("include"),
        exclude: args.remove("exclude"),
        runs: None,
    };
    if let Some(runs) = args.remove("runs") {
        let Ok(runs) = runs.parse::<u32>() else {
            return Err(format!("Cannot parse runs {runs} as a number"));
        };
        params.runs = Some(runs as i32);
    }

    if !args.is_empty() {
        Err(format!(
            "Unknown command argument(s) `{}`",
            args.into_keys().collect::<Vec<_>>().join(",")
        ))
    } else {
        Ok(params)
    }
}

/// Parses command arguments from a single line of text.
/// Expects that arguments are separated by whitespace, and each argument
/// has the format `<key>=<value>`.
fn parse_command_arguments(args: &str) -> Result<HashMap<&str, &str>, String> {
    let mut argmap = HashMap::new();
    for arg in args.split_whitespace() {
        let Some((key, value)) = arg.split_once('=') else {
            return Err(format!(
                "Invalid command argument `{arg}` (there may be no spaces around the `=` character)"
            ));
        };
        let key = key.trim();
        let value = value.trim();
        if argmap.insert(key, value).is_some() {
            return Err(format!("Duplicate command argument `{key}`"));
        }
    }

    Ok(argmap)
}

#[derive(Debug)]
struct QueueCommand<'a> {
    params: BenchmarkParameters<'a>,
}

#[derive(Debug)]
struct BenchmarkParameters<'a> {
    include: Option<&'a str>,
    exclude: Option<&'a str>,
    runs: Option<i32>,
}

/// Run the `@rust-timer build` regex over the comment message extracting the commit and the other captures
fn build_captures(comment_body: &str) -> impl Iterator<Item = (&str, regex::Captures)> {
    BODY_TIMER_BUILD
        .captures_iter(comment_body)
        .filter_map(|captures| {
            captures.get(1).map(|m| {
                let commit = m
                    .as_str()
                    .trim_start_matches("https://github.com/rust-lang/rust/commit/");
                (commit, captures)
            })
        })
}

pub async fn get_authorized_users() -> Result<Vec<u64>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_all_shas() {
        let comment_body = r#"
Going to do perf runs for a few of these:

@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af9952 (https://github.com/rust-lang/rust/pull/100307)
@rust-timer build 23936af287657fa4148aeab40cc2a780810fae52 (https://github.com/rust-lang/rust/pull/100392)
        "#;
        let shas = build_captures(comment_body)
            .map(|(c, _)| c)
            .collect::<Vec<_>>();
        assert_eq!(
            shas,
            &[
                "5832462aa1d9373b24ace96ad2c50b7a18af9952",
                "23936af287657fa4148aeab40cc2a780810fae52"
            ]
        );
    }

    #[test]
    fn command_missing() {
        assert!(parse_queue_command("").is_none());
    }

    #[test]
    fn unknown_command() {
        assert!(parse_queue_command("@rust-timer foo").is_none());
    }

    #[test]
    fn queue_command() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue"),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { include: None, exclude: None, runs: None } }))");
    }

    #[test]
    fn queue_command_unknown_arg() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue foo=bar"),
            @r###"Some(Err("Unknown command argument(s) `foo`"))"###);
    }

    #[test]
    fn queue_command_duplicate_arg() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue include=a exclude=c include=b"),
            @r###"Some(Err("Duplicate command argument `include`"))"###);
    }

    #[test]
    fn queue_command_include() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue include=abcd,feih"),
            @r###"Some(Ok(QueueCommand { params: BenchmarkParameters { include: Some("abcd,feih"), exclude: None, runs: None } }))"###);
    }

    #[test]
    fn queue_command_exclude() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue exclude=foo134,barzbaz41baf"),
            @r###"Some(Ok(QueueCommand { params: BenchmarkParameters { include: None, exclude: Some("foo134,barzbaz41baf"), runs: None } }))"###);
    }

    #[test]
    fn queue_command_runs() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue runs=5"),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { include: None, exclude: None, runs: Some(5) } }))");
    }

    #[test]
    fn queue_command_runs_nan() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue runs=xxx"),
            @r###"Some(Err("Cannot parse runs xxx as a number"))"###);
    }

    #[test]
    fn queue_command_combination() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue include=acda,13asd exclude=c13,DA runs=5"),
            @r###"Some(Ok(QueueCommand { params: BenchmarkParameters { include: Some("acda,13asd"), exclude: Some("c13,DA"), runs: Some(5) } }))"###);
    }

    #[test]
    fn queue_command_argument_spaces() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue include  =  abcd,das"),
            @r###"Some(Err("Invalid command argument `include` (there may be no spaces around the `=` character)"))"###);
    }

    #[test]
    fn queue_command_spaces() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer     queue     include=abcd,das   "),
            @r###"Some(Ok(QueueCommand { params: BenchmarkParameters { include: Some("abcd,das"), exclude: None, runs: None } }))"###);
    }

    #[test]
    fn queue_command_with_bors() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@bors try @rust-timer queue include=foo,bar"),
            @r###"Some(Ok(QueueCommand { params: BenchmarkParameters { include: Some("foo,bar"), exclude: None, runs: None } }))"###);
    }

    #[test]
    fn queue_command_parameter_order() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue runs=3 exclude=c,a include=b"),
        @r###"Some(Ok(QueueCommand { params: BenchmarkParameters { include: Some("b"), exclude: Some("c,a"), runs: Some(3) } }))"###);
    }
}
