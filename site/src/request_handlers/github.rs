use crate::api::{github, ServerResult};
use crate::github::{
    client, enqueue_shas, parse_homu_comment, rollup_pr_number, unroll_rollup,
    COMMENT_MARK_TEMPORARY, RUST_REPO_GITHUB_API_URL,
};
use crate::load::SiteCtxt;

use database::{
    parse_backends, parse_profiles, parse_targets, BenchmarkRequest, BenchmarkRequestInsertResult,
    CodegenBackend, Profile, Target,
};
use futures::stream::{FuturesUnordered, StreamExt};
use hashbrown::HashMap;
use std::sync::Arc;

pub async fn handle_github_webhook(
    request: github::Request,
    ctxt: Arc<SiteCtxt>,
) -> ServerResult<github::Response> {
    log::info!("handle_github({:?})", request);
    match request {
        github::Request::Issue {
            action,
            issue,
            comment,
        } => {
            // Ignore edits and other comment actions
            if action != "created" {
                return Ok(github::Response);
            }
            handle_issue(ctxt, issue, comment).await
        }
        github::Request::Push(p) => handle_push(ctxt, p).await,
    }
}

async fn handle_push(ctxt: Arc<SiteCtxt>, push: github::Push) -> ServerResult<github::Response> {
    let gh_client = client::Client::from_ctxt(&ctxt, RUST_REPO_GITHUB_API_URL.to_owned());
    if push.r#ref != format!("refs/heads/{}", push.repository.default_branch) {
        return Ok(github::Response);
    }
    let rollup_pr_number = match rollup_pr_number(&gh_client, &push.head_commit.message).await? {
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
        let result =
            unroll_rollup(gh_client, rollup_merges, &previous_master, rollup_pr_number).await;
        log::info!("Processing of rollup merge finished: {:#?}", result);
    });
    Ok(github::Response)
}

async fn handle_issue(
    ctxt: Arc<SiteCtxt>,
    issue: github::Issue,
    comment: github::Comment,
) -> ServerResult<github::Response> {
    // Do not react to our own comments, to avoid funny loops :)
    if comment.user.login == "rust-timer" {
        return Ok(github::Response);
    }

    let gh_client = client::Client::from_ctxt(&ctxt, RUST_REPO_GITHUB_API_URL.to_owned());
    if comment.body.contains(" homu: ") {
        if let Some(sha) = parse_homu_comment(&comment.body).await {
            enqueue_shas(
                &ctxt,
                &gh_client,
                issue.number,
                std::iter::once(sha.as_str()),
            )
            .await?;
            return Ok(github::Response);
        }
    }

    if comment.body.contains("@rust-timer") {
        return handle_rust_timer(ctxt, &gh_client, comment, issue).await;
    }

    Ok(github::Response)
}

/// The try request does not have a `sha` or a `parent_sha` but we need to keep a record
/// of this commit existing. The DB ensures that there is only one non-completed
/// try benchmark request per `pr`.
async fn record_try_benchmark_request_without_artifacts(
    conn: &dyn database::pool::Connection,
    pr: u32,
    backends: &str,
    profiles: &str,
    targets: &str,
) -> String {
    let try_request =
        BenchmarkRequest::create_try_without_artifacts(pr, backends, profiles, targets);
    log::info!("Inserting try benchmark request {try_request:?}");

    match conn.insert_benchmark_request(&try_request).await {
        Ok(BenchmarkRequestInsertResult::NothingInserted) => {
            log::info!(
                "Failed to insert try benchmark request, a request for PR`#{pr}` already exists"
            );
            format!(
                "This pull request is already queued and waiting for a try build to finish.

{COMMENT_MARK_TEMPORARY}"
            )
        }
        Ok(BenchmarkRequestInsertResult::Inserted) => {
            format!(
                "Awaiting bors try build completion.

@rustbot label: +S-waiting-on-perf

{COMMENT_MARK_TEMPORARY}"
            )
        }
        Err(e) => {
            log::error!("Failed to insert try benchmark request: {e}");
            "Something went wrong! This is most likely an internal failure, please let us know on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra)".to_string()
        }
    }
}

async fn validate_build_commands<'a>(build_cmds: &[BuildCommand<'a>]) -> Result<(), String> {
    const BASE_URL: &str = "https://ci-artifacts.rust-lang.org/rustc-builds";
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(3000))
        .build()
        .map_err(|e| format!("Failed to build request client {e}"))?;
    let mut futures = FuturesUnordered::new();

    // Many commands within one build command
    for cmd in build_cmds {
        let sha = cmd.sha;
        // Though presently very unlikely, there could be `N` targets
        let targets = cmd
            .params
            .targets
            .map(|targets| {
                targets
                    .split(',')
                    .map(str::trim)
                    .filter(|t| !t.is_empty())
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| {
                Target::default_targets()
                    .into_iter()
                    .map(|t| t.to_string())
                    .collect()
            });

        for target in targets {
            let url = format!("{BASE_URL}/{sha}/rustc-nightly-{target}.tar.xz");
            let client = client.clone();

            futures.push(async move {
                let status = client.head(&url).send().await.map(|r| r.status());
                (sha, url, status)
            });
        }
    }

    let mut errors = String::new();
    while let Some((sha, url, status)) = futures.next().await {
        match status {
            Ok(reqwest::StatusCode::NOT_FOUND) => {
                errors += format!(
                    "Missing artifact for sha `{sha}` ({url}); not built yet, try again later.\n"
                )
                .as_str();
            }
            Ok(_) => {}
            Err(e) => {
                errors += format!("Failed to check sha `{sha}` ({url}): {e}. Try again later.\n")
                    .as_str();
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

async fn handle_rust_timer(
    ctxt: Arc<SiteCtxt>,
    main_client: &client::Client,
    comment: github::Comment,
    issue: github::Issue,
) -> ServerResult<github::Response> {
    // Avoid reacting to the bot's comments
    if comment.user.login == "rust-timer" {
        return Ok(github::Response);
    }

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

                record_try_benchmark_request_without_artifacts(
                    &*conn,
                    issue.number,
                    cmd.params.backends.unwrap_or(""),
                    cmd.params.profiles.unwrap_or(""),
                    cmd.params.targets.unwrap_or(""),
                )
                .await
            }
            Err(error) => {
                format!("Error occurred while parsing comment: {error}")
            }
        };
        main_client.post_comment(issue.number, msg).await;
        return Ok(github::Response);
    }

    let mut valid_build_cmds = vec![];
    let mut errors = String::new();
    for cmd in parse_build_commands(&comment.body) {
        match cmd {
            Ok(cmd) => valid_build_cmds.push(cmd),
            Err(error) => errors.push_str(&format!("Cannot parse build command: {error}\n")),
        }
    }

    // parser errors
    if valid_build_cmds.is_empty() && errors.is_empty() {
        errors.push_str("Command cannot be empty\n");
    }

    // requested artifacts do not exist errors
    if let Err(error) = validate_build_commands(&valid_build_cmds).await {
        errors.push_str(&error);
    }

    if !errors.is_empty() {
        main_client.post_comment(issue.number, errors).await;
        return Ok(github::Response);
    }

    {
        let conn = ctxt.conn().await;
        for command in &valid_build_cmds {
            record_try_benchmark_request_without_artifacts(
                &*conn,
                issue.number,
                command.params.backends.unwrap_or(""),
                command.params.profiles.unwrap_or(""),
                command.params.targets.unwrap_or(""),
            )
            .await;
        }
    }

    let enqueued = match enqueue_shas(
        &ctxt,
        main_client,
        issue.number,
        valid_build_cmds.iter().map(|c| c.sha),
    )
    .await
    {
        Ok(enqueued) => enqueued,
        Err(error) => {
            log::error!("Failed to enqueue SHAs on {}: {error:?}", issue.number);
            main_client
                .post_comment(
                    issue.number,
                    "Failed to enqueue some commit SHAs. Maybe they were already benchmarked?"
                        .to_string(),
                )
                .await;
            return Ok(github::Response);
        }
    };
    if enqueued.len() < valid_build_cmds.len() {
        use std::fmt::Write;

        let mut msg =
            "The following SHAs were not enqueued, as they were probably already benchmarked:\n"
                .to_string();
        for cmd in valid_build_cmds {
            if !enqueued.contains(&cmd.sha) {
                writeln!(msg, "- {}", cmd.sha).unwrap();
            }
        }
        main_client.post_comment(issue.number, msg).await;
    }

    Ok(github::Response)
}

/// Parses the first occurrence of a `@rust-timer queue <shared-args>` command
/// in the input string.
fn parse_queue_command(body: &str) -> Option<Result<QueueCommand<'_>, String>> {
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

/// Parses all occurrences of a `@rust-timer build <shared-args>` command in the input string.
fn parse_build_commands(body: &str) -> impl Iterator<Item = Result<BuildCommand<'_>, String>> {
    get_command_lines(body, "build").map(|line| {
        let mut iter = line.splitn(2, ' ');
        let Some(sha) = iter.next().filter(|s| !s.is_empty() && !s.contains('=')) else {
            return Err("Missing SHA in build command".to_string());
        };

        let sha = sha.trim_start_matches("https://github.com/rust-lang/rust/commit/");
        let args = iter.next().unwrap_or("");
        let args = parse_command_arguments(args)?;
        let params = parse_benchmark_parameters(args)?;
        Ok(BuildCommand { sha, params })
    })
}

fn get_command_lines<'a>(body: &'a str, command: &'a str) -> impl Iterator<Item = &'a str> {
    let prefix = "@rust-timer";
    body.lines()
        .filter_map(move |line| {
            line.find(prefix)
                .map(|index| line[index + prefix.len()..].trim())
        })
        .filter_map(move |line| line.strip_prefix(command))
        .map(|l| l.trim_start())
}

fn parse_benchmark_parameters<'a>(
    mut args: HashMap<&'a str, &'a str>,
) -> Result<BenchmarkParameters<'a>, String> {
    let params = BenchmarkParameters {
        backends: args.remove("backends").filter(|s| !s.is_empty()),
        profiles: args.remove("profiles").filter(|s| !s.is_empty()),
        targets: args.remove("targets").filter(|s| !s.is_empty()),
    };

    if let Some(backends) = &params.backends {
        // Make sure that the backends are correct
        parse_backends(backends).map_err(|e| {
            format!(
                "Cannot parse backends: {e}. Valid values are: {}",
                CodegenBackend::all_values()
                    .iter()
                    .map(|b| b.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
    }

    if let Some(profiles) = &params.profiles {
        // Make sure that the profiles are correct
        parse_profiles(profiles).map_err(|e| {
            format!(
                "Cannot parse profiles: {e}. Valid values are: {}",
                Profile::all_values()
                    .iter()
                    .map(|b| b.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
    }

    if let Some(targets) = &params.targets {
        // Make sure that the targets are correct
        parse_targets(targets).map_err(|e| {
            format!(
                "Cannot parse targets: {e}. Valid values are: {}",
                Target::primary_targets()
                    .iter()
                    .map(|b| b.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
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
struct BuildCommand<'a> {
    sha: &'a str,
    params: BenchmarkParameters<'a>,
}

#[derive(Debug)]
struct BenchmarkParameters<'a> {
    backends: Option<&'a str>,
    profiles: Option<&'a str>,
    targets: Option<&'a str>,
}

pub async fn get_authorized_users() -> Result<Vec<u64>, String> {
    let url = format!("{}/permissions/perf.json", ::rust_team_data::v1::BASE_URL);
    let client = reqwest::Client::new();
    client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("failed to fetch authorized users: {err}"))?
        .error_for_status()
        .map_err(|err| format!("failed to fetch authorized users: {err}"))?
        .json::<rust_team_data::v1::Permission>()
        .await
        .map_err(|err| format!("failed to fetch authorized users: {err}"))
        .map(|perms| perms.github_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_missing() {
        assert!(get_build_commands("").is_empty());
    }

    #[test]
    fn build_unknown_command() {
        assert!(get_build_commands("@rust-timer foo").is_empty());
    }

    #[test]
    fn build_command_missing_sha() {
        insta::assert_compact_debug_snapshot!(get_build_commands("@rust-timer build"),
            @r###"[Err("Missing SHA in build command")]"###);
    }

    #[test]
    fn build_command() {
        insta::assert_compact_debug_snapshot!(get_build_commands("@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af9952"),
            @r#"[Ok(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af9952", params: BenchmarkParameters { backends: None, profiles: None, targets: None } })]"#);
    }

    #[test]
    fn build_command_multiple() {
        insta::assert_compact_debug_snapshot!(get_build_commands(r#"
@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af9952
@rust-timer build 23936af287657fa4148aeab40cc2a780810fae52
"#),
            @r#"[Ok(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af9952", params: BenchmarkParameters { backends: None, profiles: None, targets: None } }), Ok(BuildCommand { sha: "23936af287657fa4148aeab40cc2a780810fae52", params: BenchmarkParameters { backends: None, profiles: None, targets: None } })]"#);
    }

    #[test]
    fn build_command_unknown_arg() {
        insta::assert_compact_debug_snapshot!(get_build_commands("@rust-timer build foo=bar"),
            @r###"[Err("Missing SHA in build command")]"###);
    }

    #[test]
    fn build_command_link() {
        insta::assert_compact_debug_snapshot!(get_build_commands(r#"
@rust-timer build https://github.com/rust-lang/rust/commit/323f521bc6d8f2b966ba7838a3f3ee364e760b7e"#),
            @r#"[Ok(BuildCommand { sha: "323f521bc6d8f2b966ba7838a3f3ee364e760b7e", params: BenchmarkParameters { backends: None, profiles: None, targets: None } })]"#);
    }

    #[test]
    fn queue_command_missing() {
        assert!(parse_queue_command("").is_none());
    }

    #[test]
    fn queue_unknown_command() {
        assert!(parse_queue_command("@rust-timer foo").is_none());
    }

    #[test]
    fn queue_command() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue"),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    #[test]
    fn queue_command_unknown_arg() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue foo=bar"),
            @r###"Some(Err("Unknown command argument(s) `foo`"))"###);
    }

    #[test]
    fn queue_command_duplicate_arg() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue backends=a targets=c backends=b"),
            @r#"Some(Err("Duplicate command argument `backends`"))"#);
    }

    #[test]
    fn queue_command_argument_spaces() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue backends  =  llvm"),
            @r#"Some(Err("Invalid command argument `backends` (there may be no spaces around the `=` character)"))"#);
    }

    #[test]
    fn queue_command_spaces() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer     queue     backends=llvm   "),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("llvm"), profiles: None, targets: None } }))"#);
    }

    #[test]
    fn queue_command_with_bors() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@bors try @rust-timer queue backends=llvm"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("llvm"), profiles: None, targets: None } }))"#);
    }

    #[test]
    fn queue_command_parameter_order() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles=Doc backends=llvm"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("llvm"), profiles: Some("Doc"), targets: None } }))"#);
    }

    #[test]
    fn queue_command_multiline() {
        insta::assert_compact_debug_snapshot!(parse_queue_command(r#"Ok, this looks good now.
Let's do a perf run quickly and then we can merge it.

@bors try @rust-timer queue

Otherwise LGTM."#),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    fn get_build_commands(body: &str) -> Vec<Result<BuildCommand<'_>, String>> {
        parse_build_commands(body).collect()
    }

    #[test]
    fn build_command_with_backends() {
        insta::assert_compact_debug_snapshot!(get_build_commands(r#"@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af995G"#),
            @r#"[Ok(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af995G", params: BenchmarkParameters { backends: None, profiles: None, targets: None } })]"#);
        insta::assert_compact_debug_snapshot!(get_build_commands(r#"@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af995A backends=Llvm"#),
            @r#"[Ok(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af995A", params: BenchmarkParameters { backends: Some("Llvm"), profiles: None, targets: None } })]"#);
        insta::assert_compact_debug_snapshot!(get_build_commands(r#"@rust-timer build 23936af287657fa4148aeab40cc2a780810fae5B backends=Cranelift"#),
            @r#"[Ok(BuildCommand { sha: "23936af287657fa4148aeab40cc2a780810fae5B", params: BenchmarkParameters { backends: Some("Cranelift"), profiles: None, targets: None } })]"#);
        insta::assert_compact_debug_snapshot!(get_build_commands(r#"@rust-timer build 23936af287657fa4148aeab40cc2a780810fae5C backends=Cranelift,Llvm"#),
            @r#"[Ok(BuildCommand { sha: "23936af287657fa4148aeab40cc2a780810fae5C", params: BenchmarkParameters { backends: Some("Cranelift,Llvm"), profiles: None, targets: None } })]"#);
    }

    #[test]
    fn queue_command_with_backends() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue backends=Llvm"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("Llvm"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue backends=Cranelift"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("Cranelift"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue backends=Cranelift,Llvm"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("Cranelift,Llvm"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue"),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    #[test]
    fn queue_command_with_profiles() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles=Doc"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: Some("Doc"), targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles=Check,Clippy"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: Some("Check,Clippy"), targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles=Doc,Clippy,Opt backends=Cranelift,Llvm"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: Some("Cranelift,Llvm"), profiles: Some("Doc,Clippy,Opt"), targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles=Foo"),
            @r#"Some(Err("Cannot parse profiles: Invalid profile: Foo. Valid values are: check, debug, opt, doc, doc-json, clippy"))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles=check"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: Some("check"), targets: None } }))"#);
    }

    #[test]
    fn queue_command_with_targets() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue targets=x86_64-unknown-linux-gnu"),
            @r#"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: Some("x86_64-unknown-linux-gnu") } }))"#);
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue targets=x86_64-unknown-linux-gnu,67-unknown-none"),
            @r#"Some(Err("Cannot parse targets: Only primary targets can be specified. Valid values are: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu"))"#);
    }

    #[test]
    fn no_empty_arguments_thank_you() {
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue backends="),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue targets="),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
        insta::assert_compact_debug_snapshot!(parse_queue_command("@rust-timer queue profiles="),
            @"Some(Ok(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }
}
