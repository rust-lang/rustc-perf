use crate::api::{github, ServerResult};
use crate::github::{
    client, enqueue_sha, parse_homu_comment, COMMENT_MARK_TEMPORARY, RUST_REPO_GITHUB_API_URL,
};
use crate::load::SiteCtxt;
use std::fmt::Write;

use crate::github::client::Client;
use crate::github::triage::{triage_body_end_marker, triage_body_start_marker, TRIAGE_MARKER};
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
    }
}

const RUST_TIMER_PREFIX: &str = "@rust-timer";

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
            match enqueue_sha(&ctxt, &gh_client, issue.number, &sha).await {
                Ok(Some(mut msg)) => {
                    msg.push_str(&format!("\n{COMMENT_MARK_TEMPORARY}"));
                    gh_client.post_comment(issue.number, msg).await;
                }
                Ok(None) => {
                    // A try build without @rust-timer queue finished
                }
                Err(err) => {
                    gh_client.post_comment(issue.number, err).await;
                }
            }
            return Ok(github::Response);
        }
    }

    // Do not react to @rust-timer commands sent by the bors GitHub App
    // https://api.github.com/users/rust-bors[bot]
    if comment.body.contains(RUST_TIMER_PREFIX) && comment.user.id != 122020455 {
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

async fn validate_build_command<'a>(cmd: &BuildCommand<'a>) -> Result<(), String> {
    const BASE_URL: &str = "https://ci-artifacts.rust-lang.org/rustc-builds";
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(3000))
        .build()
        .map_err(|e| format!("Failed to build request client {e}"))?;
    let mut futures = FuturesUnordered::new();

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

    match parse_command(&comment.body) {
        Ok(RustTimerCommand::Queue(cmd)) => {
            let conn = ctxt.conn().await;
            let comment = record_try_benchmark_request_without_artifacts(
                &*conn,
                issue.number,
                cmd.params.backends.unwrap_or(""),
                cmd.params.profiles.unwrap_or(""),
                cmd.params.targets.unwrap_or(""),
            )
            .await;
            main_client.post_comment(issue.number, comment).await;
        }
        Ok(RustTimerCommand::Build(cmd)) => {
            match enqueue_sha_build(&ctxt, main_client, issue.number, &cmd).await {
                Ok(mut msg) => {
                    msg.push_str(&format!("\n{COMMENT_MARK_TEMPORARY}"));
                    main_client.post_comment(issue.number, msg).await;
                }
                Err(error) => {
                    log::error!("Failed to enqueue SHA on {}: {error:?}", issue.number);
                    main_client.post_comment(issue.number, error).await;
                }
            };
        }
        Ok(RustTimerCommand::Triage(cmd)) => {
            let mut result = String::new();
            for (i, sha) in cmd.shas.iter().enumerate() {
                // Add separator between PRs
                if i != 0 {
                    writeln!(&mut result, "---").unwrap();
                }

                // Get unrolled commit
                let commit = match main_client.get_commit(sha).await {
                    Ok(commit) => commit,
                    Err(err) => {
                        writeln!(&mut result, "### {sha}").unwrap();
                        writeln!(&mut result, "Failed to get commit: {err}").unwrap();
                        continue;
                    }
                };

                // Find PR number from commit message
                let unrolled_build_message =
                    match parse_unrolled_build_message(&commit.commit.message) {
                        Ok(r) => r,
                        Err(err) => {
                            writeln!(&mut result, "### {sha}").unwrap();
                            writeln!(&mut result, "{err}").unwrap();
                            continue;
                        }
                    };

                // Write header
                let pr_title = &commit
                    .commit
                    .message
                    .lines()
                    .nth(3)
                    .unwrap_or("<FAILED TO GET PR TITLE>");
                writeln!(
                    &mut result,
                    "### #{} {sha} {pr_title}",
                    unrolled_build_message.member_pr_number
                )
                .unwrap();

                // Enqueue the sha build and write result
                write!(
                    &mut result,
                    "{}",
                    triage_body_start_marker(unrolled_build_message.member_pr_number)
                )
                .unwrap();
                let (Ok(msg) | Err(msg)) = enqueue_sha_build(
                    &ctxt,
                    main_client,
                    unrolled_build_message.member_pr_number,
                    &BuildCommand {
                        sha,
                        params: Default::default(),
                    },
                )
                .await;
                writeln!(&mut result, "{msg}\n").unwrap();
                write!(
                    &mut result,
                    "{}",
                    triage_body_end_marker(unrolled_build_message.member_pr_number)
                )
                .unwrap();
            }
            // Add a marker to the comment which should help to find it again later
            writeln!(&mut result, "{}", TRIAGE_MARKER).unwrap();
            main_client.post_comment(issue.number, result).await;
        }
        Err(e) => {
            main_client.post_comment(issue.number, e).await;
        }
    }

    Ok(github::Response)
}

#[derive(Debug)]
pub struct UnrolledBuildMessage {
    pub member_pr_number: u32,
    pub rollup_pr_number: u32,
}

pub fn parse_unrolled_build_message(commit_message: &str) -> Result<UnrolledBuildMessage, String> {
    let first_line = commit_message.lines().next().unwrap_or("");

    // The first line of the commit message will look like
    // `Unrolled build for #123 in rollup 123`
    let words = first_line.split(" ").collect::<Vec<_>>();
    let ["Unrolled", "build", "for", member_pr_number, "in", "rollup", rollup_pr_number] =
        words[..]
    else {
        return Err(format!(
            "Unexpected commit name `{first_line}`, could not parse commit title. Is the commit an unrolled build?"
        ));
    };

    let Some(member_pr_number) = member_pr_number
        .strip_prefix("#")
        .and_then(|num| num.parse::<u32>().ok())
    else {
        return Err(format!(
            "Unexpected commit name `{first_line}`, could not parse member pr number. Is the commit an unrolled build?"
        ));
    };
    let Ok(rollup_pr_number) = rollup_pr_number.parse::<u32>() else {
        return Err(format!(
            "Unexpected commit name `{first_line}`, could not parse rollup pr number. Is the commit an unrolled build?"
        ));
    };

    Ok(UnrolledBuildMessage {
        member_pr_number,
        rollup_pr_number,
    })
}

async fn enqueue_sha_build(
    ctxt: &Arc<SiteCtxt>,
    main_client: &Client,
    issue_number: u32,
    cmd: &BuildCommand<'_>,
) -> Result<String, String> {
    // requested artifacts do not exist errors
    validate_build_command(cmd).await?;

    {
        let conn = ctxt.conn().await;
        record_try_benchmark_request_without_artifacts(
            &*conn,
            issue_number,
            cmd.params.backends.unwrap_or(""),
            cmd.params.profiles.unwrap_or(""),
            cmd.params.targets.unwrap_or(""),
        )
        .await;
    }

    match enqueue_sha(ctxt, main_client, issue_number, cmd.sha).await {
        Ok(Some(msg)) => Ok(msg),
        Ok(None) => Err(
            "Commit was not enqueued, since no previous benchmark request was found".to_string(),
        ),
        Err(err) => Err(err),
    }
}

fn parse_command(body: &str) -> Result<RustTimerCommand<'_>, String> {
    let mut cmds = body.lines().filter_map(move |line| {
        line.find(RUST_TIMER_PREFIX)
            .map(|index| line[index + RUST_TIMER_PREFIX.len()..].trim())
    });
    let Some(cmd) = cmds.next() else {
        return Err(
            "Cannot find @rust-timer command even though `@rust-timer` is tagged".to_string(),
        );
    };
    if cmds.next().is_some() {
        return Err("Rust-timer does not support multiple concurrent perf runs on the same PR. Please submit one perf run at a time, and wait until it is finished before submitting the next".to_string());
    }
    let (cmd, args) = cmd.split_once(" ").unwrap_or((cmd, ""));
    let args = args.trim();

    Ok(match cmd {
        "queue" => RustTimerCommand::Queue(parse_queue_command_args(args)?),
        "build" => RustTimerCommand::Build(parse_build_command_args(args)?),
        "triage" => RustTimerCommand::Triage(parse_triage_command_args(args)?),
        _ => return Err(format!("Unknown rust-timer command: {cmd}")),
    })
}

/// Parses the arguments of `<params>`
fn parse_queue_command_args(args: &str) -> Result<QueueCommand<'_>, String> {
    let args = parse_command_arguments(args)?;
    let params = parse_benchmark_parameters(args)?;
    Ok(QueueCommand { params })
}

/// Parses the arguments of `<sha> <params>`
fn parse_build_command_args(args: &str) -> Result<BuildCommand<'_>, String> {
    let mut iter = args.splitn(2, ' ');
    let Some(sha) = iter.next().filter(|s| !s.is_empty() && !s.contains('=')) else {
        return Err("Missing SHA in build command".to_string());
    };

    let sha = parse_sha(sha)?;
    let args = iter.next().unwrap_or("");
    let args = parse_command_arguments(args)?;
    let params = parse_benchmark_parameters(args)?;
    Ok(BuildCommand { sha, params })
}

/// Parses the arguments of `@rust-timer triage <sha>+`
fn parse_triage_command_args(args: &str) -> Result<TriageCommand<'_>, String> {
    let shas = args
        .split_whitespace()
        .map(parse_sha)
        .collect::<Result<Vec<_>, _>>()?;
    if shas.is_empty() {
        return Err(
            "The triage comment requires a space-separated list of SHAs as an argument."
                .to_string(),
        );
    }
    Ok(TriageCommand { shas })
}

fn parse_sha(sha: &str) -> Result<&str, String> {
    let sha = sha.trim_start_matches("https://github.com/rust-lang/rust/commit/");
    if !sha.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!("Sha `{sha}` is not alphanumeric"));
    }
    Ok(sha)
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
enum RustTimerCommand<'a> {
    /// This command is usually invoked as `@bors try @rust-timer queue`, which starts a bors "try build".
    /// `@rust-timer` will wait for the try build to finish, and if it succeeds will then queue a perf run.
    Queue(QueueCommand<'a>),
    /// `@rust-timer build $commit` will queue a perf run for the given `$commit`.
    Build(BuildCommand<'a>),
    /// This command is meant to be executed on a rollup,
    /// to help identify the culprit of performance regressions/improvements of that rollup.
    /// It takes a space-separated list of `$commits` SHAs, and queues a perf run for each commit.
    Triage(TriageCommand<'a>),
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
struct TriageCommand<'a> {
    shas: Vec<&'a str>,
}

#[derive(Debug, Default)]
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
    fn command_missing() {
        insta::assert_compact_debug_snapshot!(parse_command(""),
            @r#"Err("Cannot find @rust-timer command even though `@rust-timer` is tagged")"#);
    }

    #[test]
    fn unknown_command() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer foo"),
            @r#"Err("Unknown rust-timer command: foo")"#);
    }

    #[test]
    fn build_command_missing_sha() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer build"),
            @r#"Err("Missing SHA in build command")"#);
    }

    #[test]
    fn build_command() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af9952"),
            @r#"Ok(Build(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af9952", params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))"#);
    }

    #[test]
    fn build_command_invalid_sha() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af9952/5"),
            @r#"Err("Sha `5832462aa1d9373b24ace96ad2c50b7a18af9952/5` is not alphanumeric")"#);
    }

    #[test]
    fn build_command_multiple() {
        insta::assert_compact_debug_snapshot!(parse_command(r#"
@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af9952
@rust-timer build 23936af287657fa4148aeab40cc2a780810fae52
"#),
            @r#"Err("Rust-timer does not support multiple concurrent perf runs on the same PR. Please submit one perf run at a time, and wait until it is finished before submitting the next")"#);
    }

    #[test]
    fn build_command_unknown_arg() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer build foo=bar"),
            @r#"Err("Missing SHA in build command")"#);
    }

    #[test]
    fn build_command_link() {
        insta::assert_compact_debug_snapshot!(parse_command(r#"
@rust-timer build https://github.com/rust-lang/rust/commit/323f521bc6d8f2b966ba7838a3f3ee364e760b7e"#),
            @r#"Ok(Build(BuildCommand { sha: "323f521bc6d8f2b966ba7838a3f3ee364e760b7e", params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))"#);
    }

    #[test]
    fn queue_command() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue"),
            @"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    #[test]
    fn queue_command_unknown_arg() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue foo=bar"),
            @r###"Err("Unknown command argument(s) `foo`")"###);
    }

    #[test]
    fn queue_command_duplicate_arg() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue backends=a targets=c backends=b"),
            @r#"Err("Duplicate command argument `backends`")"#);
    }

    #[test]
    fn queue_command_argument_spaces() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue backends  =  llvm"),
            @r#"Err("Invalid command argument `backends` (there may be no spaces around the `=` character)")"#);
    }

    #[test]
    fn queue_command_spaces() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer     queue     backends=llvm   "),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("llvm"), profiles: None, targets: None } }))"#);
    }

    #[test]
    fn queue_command_with_bors() {
        insta::assert_compact_debug_snapshot!(parse_command("@bors try @rust-timer queue backends=llvm"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("llvm"), profiles: None, targets: None } }))"#);
    }

    #[test]
    fn queue_command_parameter_order() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles=Doc backends=llvm"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("llvm"), profiles: Some("Doc"), targets: None } }))"#);
    }

    #[test]
    fn queue_command_multiline() {
        insta::assert_compact_debug_snapshot!(parse_command(r#"Ok, this looks good now.
Let's do a perf run quickly and then we can merge it.

@bors try @rust-timer queue

Otherwise LGTM."#),
            @"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    #[test]
    fn build_command_with_backends() {
        insta::assert_compact_debug_snapshot!(parse_command(r#"@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af995G"#),
            @r#"Ok(Build(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af995G", params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command(r#"@rust-timer build 5832462aa1d9373b24ace96ad2c50b7a18af995A backends=Llvm"#),
            @r#"Ok(Build(BuildCommand { sha: "5832462aa1d9373b24ace96ad2c50b7a18af995A", params: BenchmarkParameters { backends: Some("Llvm"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command(r#"@rust-timer build 23936af287657fa4148aeab40cc2a780810fae5B backends=Cranelift"#),
            @r#"Ok(Build(BuildCommand { sha: "23936af287657fa4148aeab40cc2a780810fae5B", params: BenchmarkParameters { backends: Some("Cranelift"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command(r#"@rust-timer build 23936af287657fa4148aeab40cc2a780810fae5C backends=Cranelift,Llvm"#),
            @r#"Ok(Build(BuildCommand { sha: "23936af287657fa4148aeab40cc2a780810fae5C", params: BenchmarkParameters { backends: Some("Cranelift,Llvm"), profiles: None, targets: None } }))"#);
    }

    #[test]
    fn queue_command_with_backends() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue backends=Llvm"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("Llvm"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue backends=Cranelift"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("Cranelift"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue backends=Cranelift,Llvm"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("Cranelift,Llvm"), profiles: None, targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue"),
            @"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    #[test]
    fn queue_command_with_profiles() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles=Doc"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: Some("Doc"), targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles=Check,Clippy"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: Some("Check,Clippy"), targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles=Doc,Clippy,Opt backends=Cranelift,Llvm"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: Some("Cranelift,Llvm"), profiles: Some("Doc,Clippy,Opt"), targets: None } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles=Foo"),
            @r#"Err("Cannot parse profiles: Invalid profile: Foo. Valid values are: check, debug, opt, doc, doc-json, clippy")"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles=check"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: Some("check"), targets: None } }))"#);
    }

    #[test]
    fn queue_command_with_targets() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue targets=x86_64-unknown-linux-gnu"),
            @r#"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: Some("x86_64-unknown-linux-gnu") } }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue targets=x86_64-unknown-linux-gnu,67-unknown-none"),
            @r#"Err("Cannot parse targets: Only primary targets can be specified. Valid values are: x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu")"#);
    }

    #[test]
    fn no_empty_arguments_thank_you() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue backends="),
            @"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue targets="),
            @"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer queue profiles="),
            @"Ok(Queue(QueueCommand { params: BenchmarkParameters { backends: None, profiles: None, targets: None } }))");
    }

    #[test]
    fn triage_command() {
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage"),
            @r#"Err("The triage comment requires a space-separated list of SHAs as an argument.")"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage    "),
            @r#"Err("The triage comment requires a space-separated list of SHAs as an argument.")"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage abcd"),
            @r#"Ok(Triage(TriageCommand { shas: ["abcd"] }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage abcd efgh"),
            @r#"Ok(Triage(TriageCommand { shas: ["abcd", "efgh"] }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage abcd efgh ijkl"),
            @r#"Ok(Triage(TriageCommand { shas: ["abcd", "efgh", "ijkl"] }))"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage abcd targets=Foo"),
            @r#"Err("Sha `targets=Foo` is not alphanumeric")"#);
        insta::assert_compact_debug_snapshot!(parse_command("@rust-timer triage abcd  efgh"),
            @r#"Ok(Triage(TriageCommand { shas: ["abcd", "efgh"] }))"#);
    }

    #[test]
    fn pr_number_from_unrolled_build() {
        const EXAMPLE: &str = "Unrolled build for #157428 in rollup 1234
Rollup merge of #157428 - nia-e:allocator-refactor, r=clarfonthey

allocator: refactor for stabilisation

Adds my current proposal per the doc in #156882 and follow-up Zulip conversations (notably for [dyn-compat](https://rust-lang.zulipchat.com/#narrow/channel/197181-t-libs.2Fwg-allocators/topic/Allocator.20dyn-safety/near/599555822)) unstably.

r? libs";
        insta::assert_compact_debug_snapshot!(parse_unrolled_build_message(EXAMPLE),
            @"Ok(UnrolledBuildMessage { member_pr_number: 157428, rollup_pr_number: 1234 })");
        insta::assert_compact_debug_snapshot!(parse_unrolled_build_message("Not a correct title"),
            @r#"Err("Unexpected commit name `Not a correct title`, could not parse commit title. Is the commit an unrolled build?")"#);
        insta::assert_compact_debug_snapshot!(parse_unrolled_build_message("Unrolled build for #123almost in rollup 1234"),
            @r#"Err("Unexpected commit name `Unrolled build for #123almost in rollup 1234`, could not parse member pr number. Is the commit an unrolled build?")"#);
        insta::assert_compact_debug_snapshot!(parse_unrolled_build_message("Unrolled build for #123 in rollup 1234almost"),
            @r#"Err("Unexpected commit name `Unrolled build for #123 in rollup 1234almost`, could not parse rollup pr number. Is the commit an unrolled build?")"#);
        insta::assert_compact_debug_snapshot!(parse_unrolled_build_message("Unrolled build for #123"),
            @r#"Err("Unexpected commit name `Unrolled build for #123`, could not parse commit title. Is the commit an unrolled build?")"#);
    }
}
