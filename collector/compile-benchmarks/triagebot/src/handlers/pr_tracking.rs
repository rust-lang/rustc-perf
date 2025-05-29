//! This module updates the PR workqueue of the Rust project contributors
//! Runs after a PR has been assigned or unassigned
//!
//! Purpose:
//!
//! - Adds the PR to the workqueue of one team member (after the PR has been assigned or reopened)
//! - Removes the PR from the workqueue of one team member (after the PR has been unassigned or closed)

use crate::github::{Label, PullRequestNumber};
use crate::github::{User, UserId};
use crate::{
    config::ReviewPrefsConfig,
    github::{IssuesAction, IssuesEvent},
    handlers::Context,
};
use futures::TryStreamExt;
use octocrab::models::IssueState;
use octocrab::params::pulls::Sort;
use octocrab::params::{Direction, State};
use octocrab::Octocrab;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLockWriteGuard;
use tracing as log;

/// Maps users to a set of currently assigned open non-draft pull requests.
/// We store this map in memory, rather than in the DB, because it can get desynced when webhooks
/// are missed.
/// It is thus reloaded when triagebot starts and also periodically, so it is not needed to store it
/// in the DB.
#[derive(Debug, Default)]
pub struct ReviewerWorkqueue {
    reviewers: HashMap<UserId, HashSet<PullRequestNumber>>,
}

impl ReviewerWorkqueue {
    pub fn new(reviewers: HashMap<UserId, HashSet<PullRequestNumber>>) -> Self {
        Self { reviewers }
    }

    pub fn assigned_pr_count(&self, user_id: UserId) -> u64 {
        self.reviewers
            .get(&user_id)
            .map(|prs| prs.len() as u64)
            .unwrap_or(0)
    }
}

pub(super) enum ReviewPrefsInput {
    Assigned { assignee: User },
    Unassigned { assignee: User },
    OtherChange,
}

pub(super) async fn parse_input(
    _ctx: &Context,
    event: &IssuesEvent,
    config: Option<&ReviewPrefsConfig>,
) -> Result<Option<ReviewPrefsInput>, String> {
    // NOTE: this config check MUST exist. Else, the triagebot will emit an error
    // about this feature not being enabled
    if config.is_none() {
        return Ok(None);
    };

    // Execute this handler only if this is a PR ...
    if !event.issue.is_pr() {
        return Ok(None);
    }

    // ... and if the action is an assignment or unassignment with an assignee
    match &event.action {
        IssuesAction::Assigned { assignee } => Ok(Some(ReviewPrefsInput::Assigned {
            assignee: assignee.clone(),
        })),
        IssuesAction::Unassigned { assignee } => Ok(Some(ReviewPrefsInput::Unassigned {
            assignee: assignee.clone(),
        })),
        // We don't need to handle Opened explicitly, because that will trigger the Assigned event
        IssuesAction::Reopened
        | IssuesAction::ReadyForReview
        | IssuesAction::ConvertedToDraft
        | IssuesAction::Closed
        | IssuesAction::Deleted
        | IssuesAction::Transferred
        | IssuesAction::Labeled { .. }
        | IssuesAction::Unlabeled { .. } => Ok(Some(ReviewPrefsInput::OtherChange)),
        _ => Ok(None),
    }
}

pub(super) async fn handle_input<'a>(
    ctx: &Context,
    _config: &ReviewPrefsConfig,
    event: &IssuesEvent,
    input: ReviewPrefsInput,
) -> anyhow::Result<()> {
    log::info!("Handling event action {:?} in PR tracking", event.action);

    let pr = &event.issue;
    let pr_number = event.issue.number;

    let mut workqueue = ctx.workqueue.write().await;

    // If the PR doesn't wait for a review, remove it from the workqueue completely.
    // This handles situations such as labels being modified, which make the PR no longer to be
    // in the "waiting for a review" state, or the PR being closed/merged.
    if !waits_for_a_review(&pr.labels, &pr.assignees, &pr.user, pr.is_open(), pr.draft) {
        log::info!(
            "Removing PR {pr_number} from workqueue, because it is not waiting for a review.",
        );
        delete_pr_from_all_queues(&mut workqueue, pr_number);
        return Ok(());
    }

    match input {
        // The PR was assigned to a specific user, and it is waiting for a review.
        ReviewPrefsInput::Assigned { assignee } => {
            log::info!(
                "Adding PR {pr_number} to workqueue of {} because they were assigned.",
                assignee.login
            );

            upsert_pr_into_user_queue(&mut workqueue, assignee.id, pr_number);
        }
        ReviewPrefsInput::Unassigned { assignee } => {
            log::info!(
                "Removing PR {pr_number} from workqueue of {} because they were unassigned.",
                assignee.login
            );
            delete_pr_from_user_queue(&mut workqueue, assignee.id, pr_number);
        }
        // Some other change has happened (e.g. labels changed or the PR being reopened).
        // Make sure that all assigned users have the PR in their queue.
        // When a PR is opened, it might not yet contain all the information needed to determine
        // whether it waits for a reviewer or not. For example, when you open a PR,
        // triagebot might apply the "S-waiting-on-review" (or similar) label to it, which we
        // currently use to determine whether a PR is truly assigned to someone or not.
        // We thus need to refresh the queue state after every relevant state change that we
        // receive.
        ReviewPrefsInput::OtherChange => {
            for assignee in &event.issue.assignees {
                if upsert_pr_into_user_queue(&mut workqueue, assignee.id, pr_number) {
                    log::info!("Adding PR {pr_number} to workqueue of {}.", assignee.login);
                }
            }
        }
    }

    Ok(())
}

/// Loads the workqueue (mapping of open PRs assigned to users) from GitHub
pub async fn load_workqueue(client: &Octocrab) -> anyhow::Result<ReviewerWorkqueue> {
    tracing::debug!("Loading workqueue for rust-lang/rust");
    let prs = retrieve_pull_request_assignments("rust-lang", "rust", &client).await?;

    // Aggregate PRs by user
    let aggregated: HashMap<UserId, HashSet<PullRequestNumber>> =
        prs.into_iter().fold(HashMap::new(), |mut acc, (user, pr)| {
            let prs = acc.entry(user.id).or_default();
            prs.insert(pr);
            acc
        });
    tracing::debug!("PR assignments\n{aggregated:?}");
    Ok(ReviewerWorkqueue::new(aggregated))
}

/// Retrieve tuples of (user, PR number) where
/// the given user is assigned as a reviewer for that PR
/// and the PR is considered to be "waiting for a review", according to the semantics
/// of the reviewer workqueue.
/// See the [`waits_for_a_review`] function.
pub async fn retrieve_pull_request_assignments(
    owner: &str,
    repository: &str,
    client: &Octocrab,
) -> anyhow::Result<Vec<(User, PullRequestNumber)>> {
    let mut assignments = vec![];

    // We use the REST API to fetch open pull requests, as it is much (~5-10x)
    // faster than using GraphQL here.
    let stream = client
        .pulls(owner, repository)
        .list()
        .state(State::Open)
        .direction(Direction::Ascending)
        .sort(Sort::Created)
        .per_page(100)
        .send()
        .await?
        .into_stream(client);
    let mut stream = std::pin::pin!(stream);
    while let Some(pr) = stream.try_next().await? {
        let labels = pr
            .labels
            .unwrap_or_default()
            .into_iter()
            .map(|l| Label { name: l.name })
            .collect::<Vec<Label>>();
        let assignees = pr
            .assignees
            .as_ref()
            .map(|authors| authors.iter().map(User::from).collect::<Vec<_>>())
            .unwrap_or_default();
        let author = pr
            .user
            .as_ref()
            .map(|author| User::from(author.as_ref()))
            .unwrap_or_else(|| User {
                login: "ghost".to_string(),
                id: 0,
            });
        if waits_for_a_review(
            &labels,
            &assignees,
            &author,
            pr.state == Some(IssueState::Open),
            pr.draft.unwrap_or_default(),
        ) {
            for user in pr.assignees.unwrap_or_default() {
                assignments.push((
                    User {
                        login: user.login,
                        id: (*user.id).into(),
                    },
                    pr.number,
                ));
            }
        }
    }
    assignments.sort_by(|a, b| a.0.id.cmp(&b.0.id));

    Ok(assignments)
}

/// Get pull request assignments for a team member
pub async fn get_assigned_prs(ctx: &Context, user_id: UserId) -> HashSet<PullRequestNumber> {
    ctx.workqueue
        .read()
        .await
        .reviewers
        .get(&user_id)
        .cloned()
        .unwrap_or_default()
}

/// Add a PR to the workqueue of a team member.
/// Ensures no accidental PR duplicates.
///
/// Returns true if the PR was actually inserted.
fn upsert_pr_into_user_queue(
    workqueue: &mut RwLockWriteGuard<ReviewerWorkqueue>,
    user_id: UserId,
    pr: PullRequestNumber,
) -> bool {
    workqueue.reviewers.entry(user_id).or_default().insert(pr)
}

/// Delete a PR from the workqueue of a team member.
fn delete_pr_from_user_queue(
    workqueue: &mut ReviewerWorkqueue,
    user_id: UserId,
    pr: PullRequestNumber,
) {
    if let Some(queue) = workqueue.reviewers.get_mut(&user_id) {
        queue.remove(&pr);
    }
}

/// Delete a PR from the workqueue completely.
fn delete_pr_from_all_queues(workqueue: &mut ReviewerWorkqueue, pr: PullRequestNumber) {
    for queue in workqueue.reviewers.values_mut() {
        queue.retain(|pr_number| *pr_number != pr);
    }
}

/// Returns true if the workqueue should assume that this PR is actually waiting for a reviewer.
/// The function receives atomic attributes so that it is compatible both with triagebot's
/// `Issue` struct (used for incremental updates) and octocrab's `PullRequest` struct (used for
/// batch PR loads).
///
/// Note: this functionality is currently hardcoded for rust-lang/rust, other repos might use
/// different labels.
fn waits_for_a_review(
    labels: &[Label],
    assignees: &[User],
    author: &User,
    is_open: bool,
    is_draft: bool,
) -> bool {
    let is_blocked = labels
        .iter()
        .any(|l| l.name == "S-blocked" || l.name == "S-inactive");
    let is_rollup = labels.iter().any(|l| l.name == "rollup");
    let is_waiting_for_reviewer = labels.iter().any(|l| l.name == "S-waiting-on-review");
    let is_assigned_to_author = assignees.contains(author);

    is_open
        && !is_draft
        && !is_blocked
        && !is_rollup
        && is_waiting_for_reviewer
        && !is_assigned_to_author
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::github::{Issue, IssuesAction, IssuesEvent, Repository, User};
    use crate::github::{Label, PullRequestNumber};
    use crate::handlers::pr_tracking::{handle_input, parse_input, upsert_pr_into_user_queue};
    use crate::tests::github::{default_test_user, issue, pull_request, user};
    use crate::tests::{run_db_test, TestContext};

    #[tokio::test]
    async fn add_pr_to_workqueue_on_assign() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);

            run_handler(
                &ctx,
                IssuesAction::Assigned {
                    assignee: user.clone(),
                },
                pull_request()
                    .number(10)
                    .labels(vec!["S-waiting-on-review"])
                    .call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[10]).await;

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn ignore_blocked_pr() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);

            run_handler(
                &ctx,
                IssuesAction::Assigned {
                    assignee: user.clone(),
                },
                pull_request()
                    .labels(vec!["S-waiting-on-review", "S-blocked"])
                    .call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[]).await;

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn remove_pr_from_workqueue_on_unassign() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);
            set_assigned_prs(&ctx, &user, &[10]).await;

            run_handler(
                &ctx,
                IssuesAction::Unassigned {
                    assignee: user.clone(),
                },
                pull_request()
                    .number(10)
                    .labels(vec!["S-waiting-on-review"])
                    .call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[]).await;

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn add_pr_to_workqueue_on_label() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);

            run_handler(
                &ctx,
                IssuesAction::Assigned {
                    assignee: user.clone(),
                },
                pull_request().number(10).call(),
            )
            .await;
            check_assigned_prs(&ctx, &user, &[]).await;

            run_handler(
                &ctx,
                IssuesAction::Labeled {
                    label: Label {
                        name: "S-waiting-on-review".to_string(),
                    },
                },
                pull_request()
                    .number(10)
                    .labels(vec!["S-waiting-on-review"])
                    .assignees(vec![user.clone()])
                    .call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[10]).await;

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn remove_pr_from_workqueue_on_pr_closed() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);
            set_assigned_prs(&ctx, &user, &[10]).await;

            run_handler(
                &ctx,
                IssuesAction::Closed,
                pull_request()
                    .number(10)
                    .assignees(vec![user.clone()])
                    .call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[]).await;

            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn add_pr_to_workqueue_on_pr_reopen() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);
            set_assigned_prs(&ctx, &user, &[42]).await;

            run_handler(
                &ctx,
                IssuesAction::Reopened,
                pull_request()
                    .number(10)
                    .labels(vec!["S-waiting-on-review"])
                    .assignees(vec![user.clone()])
                    .call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[10, 42]).await;

            Ok(ctx)
        })
        .await;
    }

    // Make sure that we only consider pull requests, not issues.
    #[tokio::test]
    async fn ignore_issue_assignments() {
        run_db_test(|ctx| async move {
            let user = user("Martin", 2);

            run_handler(
                &ctx,
                IssuesAction::Assigned {
                    assignee: user.clone(),
                },
                issue().number(10).call(),
            )
            .await;

            check_assigned_prs(&ctx, &user, &[]).await;

            Ok(ctx)
        })
        .await;
    }

    async fn check_assigned_prs(
        ctx: &TestContext,
        user: &User,
        expected_prs: &[PullRequestNumber],
    ) {
        let mut assigned = ctx
            .handler_ctx()
            .workqueue
            .read()
            .await
            .reviewers
            .get(&user.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();
        assigned.sort();
        assert_eq!(assigned, expected_prs);
    }

    async fn set_assigned_prs(ctx: &TestContext, user: &User, prs: &[PullRequestNumber]) {
        {
            let mut workqueue = ctx.handler_ctx().workqueue.write().await;
            for &pr in prs {
                upsert_pr_into_user_queue(&mut workqueue, user.id, pr);
            }
        }
        check_assigned_prs(&ctx, user, prs).await;
    }

    async fn run_handler(ctx: &TestContext, action: IssuesAction, issue: Issue) {
        let handler_ctx = ctx.handler_ctx();
        let config = create_config().pr_tracking;

        let event = IssuesEvent {
            action,
            issue,
            changes: None,
            repository: Repository {
                full_name: "rust-lang-test/triagebot-test".to_string(),
                default_branch: "main".to_string(),
                fork: false,
                parent: None,
            },
            sender: default_test_user(),
        };

        let input = parse_input(&handler_ctx, &event, config.as_ref())
            .await
            .unwrap();
        if let Some(input) = input {
            handle_input(&handler_ctx, &config.unwrap(), &event, input)
                .await
                .unwrap()
        }
    }

    fn create_config() -> Config {
        toml::from_str::<Config>(
            r#"
[pr-tracking]
"#,
        )
        .unwrap()
    }
}
