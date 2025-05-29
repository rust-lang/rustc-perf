//! Handles PR and issue assignment.
//!
//! This supports several ways for setting issue/PR assignment:
//!
//! * `@rustbot assign @gh-user`: Assigns to the given user.
//! * `@rustbot claim`: Assigns to the comment author.
//! * `@rustbot release-assignment`: Removes the commenter's assignment.
//! * `r? @user`: Assigns to the given user (PRs only).
//!
//! Note: this module does not handle review assignments issued from the
//! GitHub "Assignees" dropdown menu
//!
//! This is capable of assigning to any user, even if they do not have write
//! access to the repo. It does this by fake-assigning the bot and adding a
//! "claimed by" section to the top-level comment.
//!
//! Configuration is done with the `[assign]` table.
//!
//! This also supports auto-assignment of new PRs. Based on rules in the
//! `assign.owners` config, it will auto-select an assignee based on the files
//! the PR modifies.

use crate::db::issue_data::IssueData;
use crate::db::review_prefs::{get_review_prefs_batch, RotationMode};
use crate::github::UserId;
use crate::handlers::pr_tracking::ReviewerWorkqueue;
use crate::{
    config::AssignConfig,
    github::{self, Event, FileDiff, Issue, IssuesAction, Selection},
    handlers::{Context, GithubClient, IssuesEvent},
    interactions::EditIssueBody,
};
use anyhow::{bail, Context as _};
use parser::command::assign::AssignCommand;
use parser::command::{Command, Input};
use rand::seq::IteratorRandom;
use rust_team_data::v1::Teams;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_postgres::Client as DbClient;
use tracing as log;

#[cfg(test)]
mod tests {
    mod tests_candidates;
    mod tests_from_diff;
}

const NEW_USER_WELCOME_MESSAGE: &str = "Thanks for the pull request, and welcome! \
The Rust team is excited to review your changes, and you should hear from {who} \
some time within the next two weeks.";

const CONTRIBUTION_MESSAGE: &str = "Please see [the contribution \
instructions]({contributing_url}) for more information. Namely, in order to ensure the \
minimum review times lag, PR authors and assigned reviewers should ensure that the review \
label (`S-waiting-on-review` and `S-waiting-on-author`) stays updated, invoking these commands \
when appropriate:

- `@{bot} author`: the review is finished, PR author should check the comments and take action accordingly
- `@{bot} review`: the author is ready for a review, this PR will be queued again in the reviewer's queue";

const WELCOME_WITH_REVIEWER: &str = "@{assignee} (or someone else)";

const WELCOME_WITHOUT_REVIEWER: &str = "@Mark-Simulacrum (NB. this repo may be misconfigured)";

const RETURNING_USER_WELCOME_MESSAGE: &str = "r? @{assignee}

{bot} has assigned @{assignee}.
They will have a look at your PR within the next two weeks and either review your PR or \
reassign to another reviewer.

Use `r?` to explicitly pick a reviewer";

const RETURNING_USER_WELCOME_MESSAGE_NO_REVIEWER: &str =
    "@{author}: no appropriate reviewer found, use `r?` to override";

fn reviewer_off_rotation_message(username: &str) -> String {
    format!(
        r"`{username}` is not available for reviewing at the moment.

Please choose another assignee."
    )
}

const REVIEWER_IS_PR_AUTHOR: &str = "Pull request author cannot be assigned as reviewer.

Please choose another assignee.";

const REVIEWER_ALREADY_ASSIGNED: &str =
    "Requested reviewer is already assigned to this pull request.

Please choose another assignee.";

const REVIEWER_ASSIGNED_BEFORE: &str = "Requested reviewer @{username} was already assigned before.

Please choose another assignee by using `r? @reviewer`.";

// Special account that we use to prevent assignment.
const GHOST_ACCOUNT: &str = "ghost";

/// Key for the state in the database
const PREVIOUS_REVIEWERS_KEY: &str = "previous-reviewers";

/// State stored in the database
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize, serde::Serialize)]
struct Reviewers {
    // names are stored in lowercase
    names: HashSet<String>,
}

/// Assignment data stored in the issue/PR body.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct AssignData {
    user: Option<String>,
}

/// Input for auto-assignment when a PR is created or converted from draft.
#[derive(Debug)]
pub(super) enum AssignInput {
    Opened { draft: bool },
    ReadyForReview,
}

/// Prepares the input when a new PR is opened.
pub(super) async fn parse_input(
    _ctx: &Context,
    event: &IssuesEvent,
    config: Option<&AssignConfig>,
) -> Result<Option<AssignInput>, String> {
    let config = match config {
        Some(config) => config,
        None => return Ok(None),
    };
    if config.owners.is_empty() || !event.issue.is_pr() {
        return Ok(None);
    }

    match event.action {
        IssuesAction::Opened => Ok(Some(AssignInput::Opened {
            draft: event.issue.draft,
        })),
        IssuesAction::ReadyForReview => Ok(Some(AssignInput::ReadyForReview)),
        _ => Ok(None),
    }
}

/// Handles the work of setting an assignment for a new PR and posting a
/// welcome message.
pub(super) async fn handle_input(
    ctx: &Context,
    config: &AssignConfig,
    event: &IssuesEvent,
    input: AssignInput,
) -> anyhow::Result<()> {
    let assign_command = find_assign_command(ctx, event);

    // Perform assignment when:
    // - PR was opened normally
    // - PR was opened as a draft with an explicit r? (but not r? ghost)
    // - PR was converted from a draft and there are no current assignees
    let should_assign = match input {
        AssignInput::Opened { draft: false } => true,
        AssignInput::Opened { draft: true } => {
            // Even if the PR is opened as a draft, we still want to perform assignment if r?
            // was used. However, historically, `r? ghost` was supposed to mean "do not
            // perform assignment". So in that case, we skip the assignment and only perform it once
            // the PR has been marked as being ready for review.
            assign_command.as_ref().is_some_and(|a| a != GHOST_ACCOUNT)
        }
        AssignInput::ReadyForReview => event.issue.assignees.is_empty(),
    };

    if !should_assign {
        log::info!("Skipping PR assignment, input: {input:?}, assign_command: {assign_command:?}");
        return Ok(());
    }

    let Some(diff) = event.issue.diff(&ctx.github).await? else {
        bail!(
            "expected issue {} to be a PR, but the diff could not be determined",
            event.issue.number
        )
    };

    // Don't auto-assign or welcome if the user manually set the assignee when opening.
    if event.issue.assignees.is_empty() {
        let (assignee, from_comment) =
            determine_assignee(ctx, assign_command, event, config, &diff).await?;
        if assignee.as_ref().map(|r| r.name.as_str()) == Some(GHOST_ACCOUNT) {
            // "ghost" is GitHub's placeholder account for deleted accounts.
            // It is used here as a convenient way to prevent assignment. This
            // is typically used for rollups or experiments where you don't
            // want any assignments or noise.
            return Ok(());
        }
        // This is temporarily disabled until we come up with a better
        // solution, or decide to remove this. The `is_new_contributor` query
        // is too expensive and takes too long to process.
        let welcome = if false
            && ctx
                .github
                .is_new_contributor(&event.repository, &event.issue.user.login)
                .await
        {
            let who_text = match &assignee {
                Some(assignee) => WELCOME_WITH_REVIEWER.replace("{assignee}", &assignee.name),
                None => WELCOME_WITHOUT_REVIEWER.to_string(),
            };
            let mut welcome = NEW_USER_WELCOME_MESSAGE.replace("{who}", &who_text);
            if let Some(contrib) = &config.contributing_url {
                welcome.push_str("\n\n");
                welcome.push_str(
                    &CONTRIBUTION_MESSAGE
                        .replace("{contributing_url}", contrib)
                        .replace("{bot}", &ctx.username),
                );
            }
            Some(welcome)
        } else if !from_comment {
            let welcome = match &assignee {
                Some(assignee) => RETURNING_USER_WELCOME_MESSAGE
                    .replace("{assignee}", &assignee.name)
                    .replace("{bot}", &ctx.username),
                None => RETURNING_USER_WELCOME_MESSAGE_NO_REVIEWER
                    .replace("{author}", &event.issue.user.login),
            };
            Some(welcome)
        } else {
            // No welcome is posted if they are not new and they used `r?` in the opening body.
            None
        };
        if let Some(assignee) = assignee {
            set_assignee(&ctx, &event.issue, &ctx.github, &assignee).await?;
        }

        if let Some(welcome) = welcome {
            if let Err(e) = event.issue.post_comment(&ctx.github, &welcome).await {
                log::warn!(
                    "failed to post welcome comment to {}: {e}",
                    event.issue.global_id()
                );
            }
        }
    }

    Ok(())
}

/// Finds the `r?` command in the PR body.
///
/// Returns the name after the `r?` command, or None if not found.
fn find_assign_command(ctx: &Context, event: &IssuesEvent) -> Option<String> {
    let mut input = Input::new(&event.issue.body, vec![&ctx.username]);
    input.find_map(|command| match command {
        Command::Assign(Ok(AssignCommand::RequestReview { name })) => Some(name),
        _ => None,
    })
}

fn is_self_assign(assignee: &str, pr_author: &str) -> bool {
    assignee.to_lowercase() == pr_author.to_lowercase()
}

/// Sets the assignee of a PR, alerting any errors.
async fn set_assignee(
    ctx: &Context,
    issue: &Issue,
    github: &GithubClient,
    reviewer: &ReviewerSelection,
) -> anyhow::Result<()> {
    let mut db = ctx.db.get().await;
    let mut state: IssueData<'_, Reviewers> =
        IssueData::load(&mut db, &issue, PREVIOUS_REVIEWERS_KEY).await?;

    // Don't re-assign if already assigned, e.g. on comment edit
    if issue.contain_assignee(&reviewer.name) {
        log::trace!(
            "ignoring assign PR {} to {}, already assigned",
            issue.global_id(),
            reviewer.name,
        );
        return Ok(());
    }
    if let Err(err) = issue.set_assignee(github, &reviewer.name).await {
        log::warn!(
            "failed to set assignee of PR {} to {}: {:?}",
            issue.global_id(),
            reviewer.name,
            err
        );
        if let Err(e) = issue
            .post_comment(
                github,
                &format!(
                    "Failed to set assignee to `{}`: {err}\n\
                     \n\
                     > **Note**: Only org members with at least the repository \"read\" role, \
                       users with write permissions, or people who have commented on the PR may \
                       be assigned.",
                    reviewer.name
                ),
            )
            .await
        {
            log::warn!("failed to post error comment: {e}");
            return Err(e);
        }
    } else {
        // If an error was suppressed, post a warning on the PR.
        if let Some(suppressed_error) = &reviewer.suppressed_error {
            let warning = match suppressed_error {
                FindReviewerError::ReviewerOffRotation { username } => Some(format!(
                    r"`{username}` is not on the review rotation at the moment.
They may take a while to respond.
"
                )),
                FindReviewerError::ReviewerAtMaxCapacity { username } => Some(format!(
                    "`{username}` is currently at their maximum review capacity.
They may take a while to respond."
                )),
                _ => None,
            };
            if let Some(warning) = warning {
                if let Err(err) = issue.post_comment(&ctx.github, &warning).await {
                    // This is a best-effort warning, do not do anything apart from logging if it fails
                    log::warn!("failed to post reviewer warning comment: {err}");
                }
            }
        }
    }

    // Record the reviewer in the database
    state.data.names.insert(reviewer.name.to_lowercase());
    state.save().await?;
    Ok(())
}

/// Determines who to assign the PR to based on either an `r?` command, or
/// based on which files were modified.
///
/// Will also check if candidates have capacity in their work queue.
///
/// Returns `(assignee, from_comment)` where `assignee` is who to assign to
/// (or None if no assignee could be found). `from_comment` is a boolean
/// indicating if the assignee came from an `r?` command (it is false if
/// determined from the diff).
async fn determine_assignee(
    ctx: &Context,
    assign_command: Option<String>,
    event: &IssuesEvent,
    config: &AssignConfig,
    diff: &[FileDiff],
) -> anyhow::Result<(Option<ReviewerSelection>, bool)> {
    let mut db_client = ctx.db.get().await;
    let teams = crate::team_data::teams(&ctx.github).await?;
    if let Some(name) = assign_command {
        // User included `r?` in the opening PR body.
        match find_reviewer_from_names(
            &mut db_client,
            ctx.workqueue.clone(),
            &teams,
            config,
            &event.issue,
            &[name],
        )
        .await
        {
            Ok(assignee) => return Ok((Some(assignee), true)),
            Err(e) => {
                event
                    .issue
                    .post_comment(&ctx.github, &e.to_string())
                    .await?;
                // Fall through below for normal diff detection.
            }
        }
    }
    // Errors fall-through to try fallback group.
    match find_reviewers_from_diff(config, diff) {
        Ok(candidates) if !candidates.is_empty() => {
            match find_reviewer_from_names(
                &mut db_client,
                ctx.workqueue.clone(),
                &teams,
                config,
                &event.issue,
                &candidates,
            )
            .await
            {
                Ok(assignee) => return Ok((Some(assignee), false)),
                Err(FindReviewerError::TeamNotFound(team)) => log::warn!(
                    "team {team} not found via diff from PR {}, \
                    is there maybe a misconfigured group?",
                    event.issue.global_id()
                ),
                Err(
                    e @ FindReviewerError::NoReviewer { .. }
                    | e @ FindReviewerError::ReviewerIsPrAuthor { .. }
                    | e @ FindReviewerError::ReviewerAlreadyAssigned { .. }
                    | e @ FindReviewerError::ReviewerPreviouslyAssigned { .. }
                    | e @ FindReviewerError::ReviewerOffRotation { .. }
                    | e @ FindReviewerError::DatabaseError(_)
                    | e @ FindReviewerError::ReviewerAtMaxCapacity { .. },
                ) => log::trace!(
                    "no reviewer could be determined for PR {}: {e}",
                    event.issue.global_id()
                ),
            }
        }
        // If no owners matched the diff, fall-through.
        Ok(_) => {}
        Err(e) => {
            log::warn!(
                "failed to find candidate reviewer from diff due to error: {e}\n\
                 Is the triagebot.toml misconfigured?"
            );
        }
    }

    if let Some(fallback) = config.adhoc_groups.get("fallback") {
        match find_reviewer_from_names(
            &mut db_client,
            ctx.workqueue.clone(),
            &teams,
            config,
            &event.issue,
            fallback,
        )
        .await
        {
            Ok(assignee) => return Ok((Some(assignee), false)),
            Err(e) => {
                log::trace!(
                    "failed to select from fallback group for PR {}: {e}",
                    event.issue.global_id()
                );
            }
        }
    }
    Ok((None, false))
}

/// Returns a list of candidate reviewers to use based on which files were changed.
///
/// May return an error if the owners map is misconfigured.
///
/// Beware this may return an empty list if nothing matches.
fn find_reviewers_from_diff(
    config: &AssignConfig,
    diff: &[FileDiff],
) -> anyhow::Result<Vec<String>> {
    // Map of `owners` path to the number of changes found in that path.
    // This weights the reviewer choice towards places where the most edits are done.
    let mut counts: HashMap<&str, u32> = HashMap::new();
    // Iterate over the diff, counting the number of modified lines in each
    // file, and tracks those in the `counts` map.
    for file_diff in diff {
        // List of the longest `owners` patterns that match the current path. This
        // prefers choosing reviewers from deeply nested paths over those defined
        // for top-level paths, under the assumption that they are more
        // specialized.
        //
        // This is a list to handle the situation if multiple paths of the same
        // length match.
        let mut longest_owner_patterns = Vec::new();

        // Find the longest `owners` entries that match this path.
        let mut longest = HashMap::new();
        for owner_pattern in config.owners.keys() {
            let ignore = ignore::gitignore::GitignoreBuilder::new("/")
                .add_line(None, owner_pattern)
                .with_context(|| format!("owner file pattern `{owner_pattern}` is not valid"))?
                .build()?;
            if ignore
                .matched_path_or_any_parents(&file_diff.filename, false)
                .is_ignore()
            {
                let owner_len = owner_pattern.split('/').count();
                longest.insert(owner_pattern, owner_len);
            }
        }
        let max_count = longest.values().copied().max().unwrap_or(0);
        longest_owner_patterns.extend(
            longest
                .iter()
                .filter(|(_, count)| **count == max_count)
                .map(|x| *x.0),
        );
        // Give some weight to these patterns to start. This helps with
        // files modified without any lines changed.
        for owner_pattern in &longest_owner_patterns {
            *counts.entry(owner_pattern).or_default() += 1;
        }

        // Count the modified lines.
        for line in file_diff.patch.lines() {
            if (!line.starts_with("+++") && line.starts_with('+'))
                || (!line.starts_with("---") && line.starts_with('-'))
            {
                for owner_path in &longest_owner_patterns {
                    *counts.entry(owner_path).or_default() += 1;
                }
            }
        }
    }
    // Use the `owners` entry with the most number of modifications.
    let max_count = counts.values().copied().max().unwrap_or(0);
    let max_paths = counts
        .iter()
        .filter(|(_, count)| **count == max_count)
        .map(|(path, _)| path);
    let mut potential: Vec<_> = max_paths
        .flat_map(|owner_path| &config.owners[*owner_path])
        .map(|owner| owner.to_string())
        .collect();
    // Dedupe. This isn't strictly necessary, as `find_reviewer_from_names` will deduplicate.
    // However, this helps with testing.
    potential.sort();
    potential.dedup();
    Ok(potential)
}

/// Handles a command posted in a comment.
pub(super) async fn handle_command(
    ctx: &Context,
    config: &AssignConfig,
    event: &Event,
    cmd: AssignCommand,
) -> anyhow::Result<()> {
    let is_team_member = if let Err(_) | Ok(false) = event.user().is_team_member(&ctx.github).await
    {
        false
    } else {
        true
    };

    // Don't handle commands in comments from the bot. Some of the comments it
    // posts contain commands to instruct the user, not things that the bot
    // should respond to.
    if event.user().login == ctx.username.as_str() {
        return Ok(());
    }

    let issue = event.issue().unwrap();
    if issue.is_pr() {
        if !issue.is_open() {
            issue
                .post_comment(&ctx.github, "Assignment is not allowed on a closed PR.")
                .await?;
            return Ok(());
        }
        if matches!(
            event,
            Event::Issue(IssuesEvent {
                action: IssuesAction::Opened,
                ..
            })
        ) {
            // Don't handle review request comments on new PRs. Those will be
            // handled by the new PR trigger (which also handles the
            // welcome message).
            return Ok(());
        }

        let teams = crate::team_data::teams(&ctx.github).await?;

        let assignee = match cmd {
            AssignCommand::Claim => event.user().login.clone(),
            AssignCommand::AssignUser { username } => username,
            AssignCommand::ReleaseAssignment => {
                log::trace!(
                    "ignoring release on PR {:?}, must always have assignee",
                    issue.global_id()
                );
                return Ok(());
            }
            AssignCommand::RequestReview { name } => {
                // Determine if assignee is a team. If yes, add the corresponding GH label.
                if let Some(team_name) = get_team_name(&teams, &issue, &name) {
                    let t_label = format!("T-{team_name}");
                    if let Err(err) = issue
                        .add_labels(&ctx.github, vec![github::Label { name: t_label }])
                        .await
                    {
                        if let Some(github::UnknownLabels { .. }) = err.downcast_ref() {
                            log::warn!("Error assigning label: {}", err);
                        } else {
                            return Err(err);
                        }
                    }
                }
                name
            }
        };

        // In the PR body, `r? ghost` means "do not assign anybody".
        // When you send `r? ghost` in a PR comment, it should mean "unassign the current assignee".
        // Only allow this for the PR author (usually when they forget to do `r? ghost` in the PR
        // body), otherwise anyone could remove assignees from any PR.
        if assignee == GHOST_ACCOUNT && issue.user.login == event.user().login {
            issue.remove_assignees(&ctx.github, Selection::All).await?;
            return Ok(());
        }
        let mut db_client = ctx.db.get().await;
        let assignee = match find_reviewer_from_names(
            &mut db_client,
            ctx.workqueue.clone(),
            &teams,
            config,
            issue,
            &[assignee.to_string()],
        )
        .await
        {
            Ok(assignee) => assignee,
            Err(e) => {
                issue.post_comment(&ctx.github, &e.to_string()).await?;
                return Ok(());
            }
        };

        set_assignee(ctx, issue, &ctx.github, &assignee).await?;
    } else {
        let e = EditIssueBody::new(&issue, "ASSIGN");

        let to_assign = match cmd {
            AssignCommand::Claim => event.user().login.clone(),
            AssignCommand::AssignUser { username } => {
                if !is_team_member && username != event.user().login {
                    bail!("Only Rust team members can assign other users");
                }
                username.clone()
            }
            AssignCommand::ReleaseAssignment => {
                if let Some(AssignData {
                    user: Some(current),
                }) = e.current_data()
                {
                    if current == event.user().login || is_team_member {
                        issue.remove_assignees(&ctx.github, Selection::All).await?;
                        e.apply(&ctx.github, String::new(), AssignData { user: None })
                            .await?;
                        return Ok(());
                    } else {
                        bail!("Cannot release another user's assignment");
                    }
                } else {
                    let current = &event.user().login;
                    if issue.contain_assignee(current) {
                        issue
                            .remove_assignees(&ctx.github, Selection::One(&current))
                            .await?;
                        e.apply(&ctx.github, String::new(), AssignData { user: None })
                            .await?;
                        return Ok(());
                    } else {
                        bail!("Cannot release unassigned issue");
                    }
                };
            }
            AssignCommand::RequestReview { .. } => bail!("r? is only allowed on PRs."),
        };
        // Don't re-assign if aleady assigned, e.g. on comment edit
        if issue.contain_assignee(&to_assign) {
            log::trace!(
                "ignoring assign issue {} to {}, already assigned",
                issue.global_id(),
                to_assign,
            );
            return Ok(());
        }
        let data = AssignData {
            user: Some(to_assign.clone()),
        };

        e.apply(&ctx.github, String::new(), &data).await?;

        match issue.set_assignee(&ctx.github, &to_assign).await {
            Ok(()) => return Ok(()), // we are done
            Err(github::AssignmentError::InvalidAssignee) => {
                issue
                    .set_assignee(&ctx.github, &ctx.username)
                    .await
                    .context("self-assignment failed")?;
                let cmt_body = format!(
                    "This issue has been assigned to @{} via [this comment]({}).",
                    to_assign,
                    event.html_url().unwrap()
                );
                e.apply(&ctx.github, cmt_body, &data).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

fn strip_organization_prefix<'a>(issue: &Issue, name: &'a str) -> &'a str {
    let repo = issue.repository();
    // @ is optional, so it is trimmed separately
    // both @rust-lang/compiler and rust-lang/compiler should work
    name.trim_start_matches("@")
        .trim_start_matches(&format!("{}/", repo.organization))
}

/// Returns `Some(team_name)` if `name` corresponds to a name of a team.
fn get_team_name<'a>(teams: &Teams, issue: &Issue, name: &'a str) -> Option<&'a str> {
    let team_name = strip_organization_prefix(issue, name);
    // Remove "t-" or "T-" prefixes before checking if it's a team name
    let team_name = team_name.trim_start_matches("t-").trim_start_matches("T-");
    teams.teams.get(team_name).map(|_| team_name)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum FindReviewerError {
    /// User specified something like `r? foo/bar` where that team name could
    /// not be found.
    TeamNotFound(String),
    /// No reviewer could be found.
    ///
    /// This could happen if there is a cyclical group or other misconfiguration.
    /// `initial` is the initial list of candidate names.
    NoReviewer { initial: Vec<String> },
    /// Requested reviewer is off the review rotation (e.g. on a vacation).
    /// Either the username is in [users_on_vacation] in `triagebot.toml` or the user has
    /// configured [RotationMode::OffRotation] in their reviewer preferences.
    ReviewerOffRotation { username: String },
    /// Requested reviewer is PR author
    ReviewerIsPrAuthor { username: String },
    /// Requested reviewer is already assigned to that PR
    ReviewerAlreadyAssigned { username: String },
    /// Requested reviewer was already assigned previously to that PR.
    ReviewerPreviouslyAssigned { username: String },
    /// Data required for assignment could not be loaded from the DB.
    DatabaseError(String),
    /// The reviewer has too many PRs already assigned.
    ReviewerAtMaxCapacity { username: String },
}

impl std::error::Error for FindReviewerError {}

impl fmt::Display for FindReviewerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            FindReviewerError::TeamNotFound(team) => {
                write!(
                    f,
                    "Team or group `{team}` not found.\n\
                    \n\
                    rust-lang team names can be found at https://github.com/rust-lang/team/tree/master/teams.\n\
                    Reviewer group names can be found in `triagebot.toml` in this repo."
                )
            }
            FindReviewerError::NoReviewer { initial } => {
                write!(
                    f,
                    "No reviewers could be found from initial request `{}`\n\
                     This repo may be misconfigured.\n\
                     Use `r?` to specify someone else to assign.",
                    initial.join(",")
                )
            }
            FindReviewerError::ReviewerOffRotation { username } => {
                write!(f, "{}", reviewer_off_rotation_message(username))
            }
            FindReviewerError::ReviewerIsPrAuthor { username } => {
                write!(
                    f,
                    "{}",
                    REVIEWER_IS_PR_AUTHOR.replace("{username}", username)
                )
            }
            FindReviewerError::ReviewerAlreadyAssigned { username } => {
                write!(
                    f,
                    "{}",
                    REVIEWER_ALREADY_ASSIGNED.replace("{username}", username)
                )
            }
            FindReviewerError::ReviewerPreviouslyAssigned { username } => {
                write!(
                    f,
                    "{}",
                    REVIEWER_ASSIGNED_BEFORE.replace("{username}", username)
                )
            }
            FindReviewerError::DatabaseError(error) => {
                write!(f, "Database error: {error}")
            }
            FindReviewerError::ReviewerAtMaxCapacity { username } => {
                write!(
                    f,
                    r"`{username}` has too many PRs assigned to them.

Please select a different reviewer.",
                )
            }
        }
    }
}

/// Reviewer that was found to be eligible as a result of `r? <...>`.
/// In some cases, a reviewer selection error might have been suppressed.
/// We store it here to allow sending a comment with a warning about the suppressed error.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct ReviewerSelection {
    name: String,
    suppressed_error: Option<FindReviewerError>,
}

impl ReviewerSelection {
    fn from_name(name: String) -> Self {
        Self {
            name,
            suppressed_error: None,
        }
    }
}

/// Finds a reviewer to assign to a PR.
///
/// The `names` is a list of candidate reviewers `r?`, such as `compiler` or
/// `@octocat`, or names from the owners map. It can contain GitHub usernames,
/// auto-assign groups, or rust-lang team names. It must have at least one
/// entry.
async fn find_reviewer_from_names(
    db: &mut DbClient,
    workqueue: Arc<RwLock<ReviewerWorkqueue>>,
    teams: &Teams,
    config: &AssignConfig,
    issue: &Issue,
    names: &[String],
) -> Result<ReviewerSelection, FindReviewerError> {
    // Fast path for self-assign, which is always allowed.
    if let [name] = names {
        if is_self_assign(&name, &issue.user.login) {
            return Ok(ReviewerSelection::from_name(name.clone()));
        }
    }

    let candidates =
        candidate_reviewers_from_names(db, workqueue, teams, config, issue, names).await?;
    assert!(!candidates.is_empty());

    // This uses a relatively primitive random choice algorithm.
    // GitHub's CODEOWNERS supports much more sophisticated options, such as:
    //
    // - Round robin: Chooses reviewers based on who's received the least
    //   recent review request, focusing on alternating between all members of
    //   the team regardless of the number of outstanding reviews they
    //   currently have.
    // - Load balance: Chooses reviewers based on each member's total number
    //   of recent review requests and considers the number of outstanding
    //   reviews for each member. The load balance algorithm tries to ensure
    //   that each team member reviews an equal number of pull requests in any
    //   30 day period.
    //
    // Additionally, with CODEOWNERS, users marked as "Busy" in the GitHub UI
    // will not be selected for reviewer. There are several other options for
    // configuring CODEOWNERS as well.
    //
    // These are all ideas for improving the selection here. However, I'm not
    // sure they are really worth the effort.

    log::info!(
        "[#{}] Filtered list of candidates: {:?}",
        issue.number,
        candidates
    );

    // Select a random reviewer from the filtered list
    Ok(candidates
        .into_iter()
        .choose(&mut rand::thread_rng())
        .expect("candidate_reviewers_from_names should return at least one entry"))
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct ReviewerCandidate {
    name: String,
    origin: ReviewerCandidateOrigin,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum ReviewerCandidateOrigin {
    /// This reviewer was directly requested for a review.
    Direct,
    /// This reviewer was expanded from a team or an assign group.
    Expanded,
}

/// Recursively expand all teams and adhoc groups found within `names`.
/// Returns a set of expanded usernames.
/// Also normalizes usernames from `@user` to `user`.
fn expand_teams_and_groups(
    teams: &Teams,
    issue: &Issue,
    config: &AssignConfig,
    names: &[String],
) -> Result<HashSet<ReviewerCandidate>, FindReviewerError> {
    let mut selected_candidates: HashSet<String> = HashSet::new();

    // Keep track of groups seen to avoid cycles and avoid expanding the same
    // team multiple times.
    let mut seen_names = HashSet::new();

    enum Candidate<'a> {
        Direct(&'a str),
        Expanded(&'a str),
    }

    // This is a queue of potential groups or usernames to expand. The loop
    // below will pop from this and then append the expanded results of teams.
    // Usernames will be added to `selected_candidates`.
    let mut to_be_expanded: Vec<Candidate> = names
        .iter()
        .map(|n| Candidate::Direct(n.as_str()))
        .collect();

    // We store the directly requested usernames (after normalization).
    // A username can be both directly requested and expanded from a group/team, the former
    // should have priority.
    let mut directly_requested: HashSet<&str> = HashSet::new();

    // Loop over names to recursively expand them.
    while let Some(candidate) = to_be_expanded.pop() {
        let name_to_expand = match &candidate {
            Candidate::Direct(name) => name,
            Candidate::Expanded(name) => name,
        };

        // `name_to_expand` could be a team name, an adhoc group name or a username.
        let maybe_team = get_team_name(teams, issue, name_to_expand);
        let maybe_group = strip_organization_prefix(issue, name_to_expand);
        let maybe_user = name_to_expand.strip_prefix('@').unwrap_or(name_to_expand);

        // Try ad-hoc groups first.
        if let Some(group_members) = config.adhoc_groups.get(maybe_group) {
            // If a group has already been expanded, don't expand it again.
            if seen_names.insert(maybe_group) {
                to_be_expanded.extend(
                    group_members
                        .iter()
                        .map(|s| Candidate::Expanded(s.as_str())),
                );
            }
            continue;
        }

        // Check for a team name.
        // Allow either a direct team name like `rustdoc` or a GitHub-style
        // team name of `rust-lang/rustdoc` (though this does not check if
        // that is a real GitHub team name).
        //
        // This ignores subteam relationships (it only uses direct members).
        if let Some(team) = maybe_team.and_then(|t| teams.teams.get(t)) {
            selected_candidates.extend(team.members.iter().map(|member| member.github.clone()));
            continue;
        }

        // Here we know it's not a known team nor a group.
        // If the username contains a slash, assume that it is an unknown team.
        if maybe_user.contains('/') {
            return Err(FindReviewerError::TeamNotFound(maybe_user.to_string()));
        }

        // Assume it is a user.
        let username = maybe_user.to_string();
        selected_candidates.insert(username);

        if let Candidate::Direct(_) = candidate {
            directly_requested.insert(maybe_user);
        }
    }

    // Now that we have a unique set of candidates, figure out which ones of them were requested
    // directly.
    Ok(selected_candidates
        .into_iter()
        .map(|name| {
            let origin = if directly_requested.contains(name.as_str()) {
                ReviewerCandidateOrigin::Direct
            } else {
                ReviewerCandidateOrigin::Expanded
            };
            ReviewerCandidate { name, origin }
        })
        .collect())
}

/// Returns a list of candidate usernames (from relevant teams) to choose as a reviewer.
/// If no reviewer is available, returns an error.
async fn candidate_reviewers_from_names<'a>(
    db: &mut DbClient,
    workqueue: Arc<RwLock<ReviewerWorkqueue>>,
    teams: &'a Teams,
    config: &'a AssignConfig,
    issue: &Issue,
    names: &'a [String],
) -> Result<HashSet<ReviewerSelection>, FindReviewerError> {
    // Step 1: expand teams and groups into candidate names
    let expanded = expand_teams_and_groups(teams, issue, config, names)?;
    let expanded_count = expanded.len();

    // Was it a request for a single user, i.e. `r? @username`?
    let is_single_user = expanded_count == 1
        && matches!(
            expanded.iter().next().map(|c| c.origin),
            Some(ReviewerCandidateOrigin::Direct)
        );

    // Set of candidate usernames to choose from.
    // We go through each expanded candidate and store either success or an error for them.
    let mut candidates: Vec<Result<ReviewerCandidate, FindReviewerError>> = Vec::new();
    let previous_reviewer_names = get_previous_reviewer_names(db, issue).await;

    // Step 2: pre-filter candidates based on checks that we can perform quickly
    for reviewer_candidate in expanded {
        let candidate = &reviewer_candidate.name;
        let name_lower = candidate.to_lowercase();
        let is_pr_author = name_lower == issue.user.login.to_lowercase();
        let is_on_vacation = config.is_on_vacation(&candidate);
        let is_already_assigned = issue
            .assignees
            .iter()
            .any(|assignee| name_lower == assignee.login.to_lowercase());

        let is_previously_assigned = previous_reviewer_names.contains(&name_lower);

        // Record the reason why the candidate was filtered out
        let reason = {
            if is_pr_author {
                Some(FindReviewerError::ReviewerIsPrAuthor {
                    username: candidate.clone(),
                })
            } else if is_on_vacation {
                Some(FindReviewerError::ReviewerOffRotation {
                    username: candidate.clone(),
                })
            } else if is_already_assigned {
                Some(FindReviewerError::ReviewerAlreadyAssigned {
                    username: candidate.clone(),
                })
            } else if reviewer_candidate.origin == ReviewerCandidateOrigin::Expanded
                && is_previously_assigned
            {
                // **Only** when r? group is expanded, we consider the reviewer previously assigned
                // `r? @reviewer` will not consider the reviewer previously assigned
                Some(FindReviewerError::ReviewerPreviouslyAssigned {
                    username: candidate.clone(),
                })
            } else {
                None
            }
        };

        if let Some(error_reason) = reason {
            candidates.push(Err(error_reason));
        } else {
            candidates.push(Ok(reviewer_candidate));
        }
    }
    assert_eq!(candidates.len(), expanded_count);

    if config.review_prefs.is_some() {
        // Step 3: gather potential usernames to form a DB query for review preferences
        let usernames: Vec<String> = candidates
            .iter()
            .filter_map(|res| res.as_ref().ok().map(|s| s.name.to_string()))
            .collect();
        let usernames: Vec<&str> = usernames.iter().map(|s| s.as_str()).collect();
        let review_prefs = get_review_prefs_batch(db, &usernames)
            .await
            .context("cannot fetch review preferences")
            .map_err(|e| FindReviewerError::DatabaseError(e.to_string()))?;

        let workqueue = workqueue.read().await;

        // Step 4: check review preferences
        candidates = candidates
            .into_iter()
            .map(|candidate| {
                // Only consider candidates that did not have an earlier error
                let candidate = candidate?;
                let username = &candidate.name;

                // If no review prefs were found, we assume the default unlimited
                // review capacity and being on rotation.
                let Some(review_prefs) = review_prefs.get(username.as_str()) else {
                    return Ok(candidate);
                };
                if let Some(capacity) = review_prefs.max_assigned_prs {
                    let assigned_prs = workqueue.assigned_pr_count(review_prefs.user_id as UserId);
                    // Is the reviewer at max capacity?
                    if (assigned_prs as i32) >= capacity {
                        return Err(FindReviewerError::ReviewerAtMaxCapacity {
                            username: username.clone(),
                        });
                    }
                }
                if review_prefs.rotation_mode == RotationMode::OffRotation {
                    return Err(FindReviewerError::ReviewerOffRotation {
                        username: username.clone(),
                    });
                }

                return Ok(candidate);
            })
            .collect();
    }
    assert_eq!(candidates.len(), expanded_count);

    let valid_candidates: HashSet<&str> = candidates
        .iter()
        .filter_map(|res| res.as_ref().ok().map(|c| c.name.as_str()))
        .collect();

    log::debug!(
        "Candidate reviewer results for review request `{}` on `{}`: {:?}",
        names.join(", "),
        issue.global_id(),
        candidates
    );

    if valid_candidates.is_empty() {
        if is_single_user {
            // If we requested a single user for a review, we may suppress some errors.
            // Check what error we got here.
            let error = candidates
                .pop()
                .unwrap()
                .expect_err("valid_candidates is empty, so this should be an error");
            let username = match &error {
                // If the reviewer is at capacity or off rotation, allow them to be requested,
                // but store the suppressed error.
                FindReviewerError::ReviewerOffRotation { username }
                | FindReviewerError::ReviewerAtMaxCapacity { username } => username,
                _ => return Err(error),
            };
            Ok(HashSet::from([ReviewerSelection {
                name: username.to_string(),
                suppressed_error: Some(error),
            }]))
        } else {
            // If it was a request for a team or a group, and no one is available, simply
            // return `NoReviewer`.
            log::warn!(
                "No valid candidates found for review request on {}. Reasons: {:?}",
                issue.global_id(),
                candidates
            );
            Err(FindReviewerError::NoReviewer {
                initial: names.to_vec(),
            })
        }
    } else {
        Ok(valid_candidates
            .into_iter()
            .map(|s| ReviewerSelection::from_name(s.to_string()))
            .collect())
    }
}

async fn get_previous_reviewer_names(db: &mut DbClient, issue: &Issue) -> HashSet<String> {
    let state: IssueData<'_, Reviewers> =
        match IssueData::load(db, &issue, PREVIOUS_REVIEWERS_KEY).await {
            Ok(state) => state,
            Err(_) => return HashSet::new(),
        };

    state.data.names
}
