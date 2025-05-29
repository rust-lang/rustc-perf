//! Tests for `candidate_reviewers_from_names`

use super::super::*;
use crate::db::review_prefs::{upsert_review_prefs, RotationMode};
use crate::github::{PullRequestNumber, User};
use crate::handlers::pr_tracking::ReviewerWorkqueue;
use crate::tests::github::{issue, user};
use crate::tests::{run_db_test, TestContext};

#[must_use]
struct AssignCtx {
    test_ctx: TestContext,
    teams: Teams,
    config: AssignConfig,
    issue: Issue,
    reviewer_workqueue: HashMap<UserId, HashSet<PullRequestNumber>>,
}

impl AssignCtx {
    fn new(test_ctx: TestContext, config: toml::Table, issue: Issue) -> Self {
        Self {
            test_ctx,
            teams: Teams {
                teams: Default::default(),
            },
            config: config.try_into().unwrap(),
            issue,
            reviewer_workqueue: Default::default(),
        }
    }

    fn teams(mut self, table: &toml::Table) -> Self {
        let teams: serde_json::Value = table.clone().try_into().unwrap();
        let mut teams_config = serde_json::json!({});
        for (team_name, members) in teams.as_object().unwrap() {
            let members: Vec<_> = members.as_array().unwrap().iter().map(|member| {
                serde_json::json!({"name": member, "github": member, "github_id": 100, "is_lead": false})
            }).collect();
            teams_config[team_name] = serde_json::json!({
                "name": team_name,
                "kind": "team",
                "members": serde_json::Value::Array(members),
                "alumni": [],
                "discord": [],
                "roles": [],
            });
        }
        self.teams = serde_json::value::from_value(teams_config).unwrap();
        self
    }

    fn assign_prs(mut self, user_id: UserId, count: u64) -> Self {
        let prs: HashSet<PullRequestNumber> = (0..count).collect();
        self.reviewer_workqueue.insert(user_id, prs);
        self
    }

    async fn set_review_prefs(
        self,
        user: &User,
        capacity: Option<u32>,
        rotation_mode: RotationMode,
    ) -> Self {
        upsert_review_prefs(
            self.test_ctx.db_client(),
            user.clone(),
            capacity,
            rotation_mode,
        )
        .await
        .unwrap();
        self
    }

    async fn set_previous_reviewers(mut self, users: HashSet<&User>) -> Self {
        let mut db = self.test_ctx.db_client_mut();
        let mut state: IssueData<'_, Reviewers> =
            IssueData::load(&mut db, &self.issue, PREVIOUS_REVIEWERS_KEY)
                .await
                .unwrap();

        // Create a new set with all user names (overwrite existing data)
        state.data.names = users.iter().map(|user| user.login.to_lowercase()).collect();
        state.save().await.unwrap();
        self
    }

    async fn check(
        mut self,
        names: &[&str],
        expected: Result<&[ReviewerSelection], FindReviewerError>,
    ) -> anyhow::Result<Self> {
        let names: Vec<_> = names.iter().map(|n| n.to_string()).collect();

        let workqueue = ReviewerWorkqueue::new(self.reviewer_workqueue.clone());
        let reviewers = candidate_reviewers_from_names(
            self.test_ctx.db_client_mut(),
            Arc::new(RwLock::new(workqueue)),
            &self.teams,
            &self.config,
            &self.issue,
            &names,
        )
        .await;
        match (reviewers, expected) {
            (Ok(candidates), Ok(expected)) => {
                let mut candidates: Vec<&ReviewerSelection> = candidates.iter().collect();
                candidates.sort();
                let expected: Vec<&ReviewerSelection> = expected.iter().collect();
                assert_eq!(candidates, expected);
            }
            (Err(actual), Err(expected)) => {
                assert_eq!(actual, expected)
            }
            (Ok(candidates), Err(_)) => panic!("expected Err, got Ok: {candidates:?}"),
            (Err(e), Ok(_)) => panic!("expected Ok, got Err: {e}"),
        };
        Ok(self)
    }
}

impl From<AssignCtx> for TestContext {
    fn from(value: AssignCtx) -> Self {
        value.test_ctx
    }
}

impl From<&str> for ReviewerSelection {
    fn from(value: &str) -> Self {
        ReviewerSelection::from_name(value.to_string())
    }
}

/// Basic test function for testing `candidate_reviewers_from_names`.
fn basic_test(ctx: TestContext, config: toml::Table, issue: Issue) -> AssignCtx {
    AssignCtx::new(ctx, config, issue)
}

fn review_prefs_test(ctx: TestContext) -> AssignCtx {
    let config = toml::toml!([review_prefs]);
    basic_test(ctx, config, issue().call())
}

#[tokio::test]
async fn no_assigned_prs() {
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .set_review_prefs(&user, Some(3), RotationMode::OnRotation)
            .await
            .check(&["martin"], Ok(&["martin".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn no_review_prefs() {
    run_db_test(|ctx| async move {
        ctx.add_user("martin", 1).await;
        review_prefs_test(ctx)
            .assign_prs(1, 3)
            .check(&["martin"], Ok(&["martin".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn at_max_capacity() {
    let teams = toml::toml!(compiler = ["martin", "diana"]);
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .teams(&teams)
            .set_review_prefs(&user, Some(3), RotationMode::OnRotation)
            .await
            .assign_prs(user.id, 3)
            .check(
                &["martin"],
                Ok(&[ReviewerSelection {
                    name: "martin".to_string(),
                    suppressed_error: Some(FindReviewerError::ReviewerAtMaxCapacity {
                        username: "martin".to_string(),
                    }),
                }]),
            )
            .await?
            .check(&["compiler"], Ok(&["diana".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn below_max_capacity() {
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .set_review_prefs(&user, Some(3), RotationMode::OnRotation)
            .await
            .assign_prs(user.id, 2)
            .check(&["martin"], Ok(&["martin".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn above_max_capacity() {
    let teams = toml::toml!(compiler = ["martin", "diana"]);
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .teams(&teams)
            .set_review_prefs(&user, Some(3), RotationMode::OnRotation)
            .await
            .assign_prs(user.id, 10)
            .check(
                &["martin"],
                Ok(&[ReviewerSelection {
                    name: "martin".to_string(),
                    suppressed_error: Some(FindReviewerError::ReviewerAtMaxCapacity {
                        username: "martin".to_string(),
                    }),
                }]),
            )
            .await?
            .check(&["compiler"], Ok(&["diana".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn max_capacity_zero() {
    let teams = toml::toml!(compiler = ["martin", "diana"]);
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .teams(&teams)
            .set_review_prefs(&user, Some(0), RotationMode::OnRotation)
            .await
            .assign_prs(user.id, 0)
            .check(
                &["martin"],
                Ok(&[ReviewerSelection {
                    name: "martin".to_string(),
                    suppressed_error: Some(FindReviewerError::ReviewerAtMaxCapacity {
                        username: "martin".to_string(),
                    }),
                }]),
            )
            .await?
            .check(&["compiler"], Ok(&["diana".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn ignore_username_case() {
    let teams = toml::toml!(compiler = ["martin", "diana"]);
    run_db_test(|ctx| async move {
        let user = user("MARtin", 1);
        review_prefs_test(ctx)
            .teams(&teams)
            .set_review_prefs(&user, Some(3), RotationMode::OnRotation)
            .await
            .assign_prs(user.id, 3)
            .check(
                &["MARTIN"],
                Ok(&[ReviewerSelection {
                    name: "MARTIN".to_string(),
                    suppressed_error: Some(FindReviewerError::ReviewerAtMaxCapacity {
                        username: "MARTIN".to_string(),
                    }),
                }]),
            )
            .await?
            .check(&["compiler"], Ok(&["diana".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn unlimited_capacity() {
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .set_review_prefs(&user, None, RotationMode::OnRotation)
            .await
            .assign_prs(user.id, 10)
            .check(&["martin"], Ok(&["martin".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn user_off_rotation() {
    run_db_test(|ctx| async move {
        let teams = toml::toml!(compiler = ["martin", "diana"]);
        let user = user("martin", 1);
        review_prefs_test(ctx)
            .teams(&teams)
            .set_review_prefs(&user, None, RotationMode::OffRotation)
            .await
            .check(
                &["martin"],
                Ok(&[ReviewerSelection {
                    name: "martin".to_string(),
                    suppressed_error: Some(FindReviewerError::ReviewerOffRotation {
                        username: "martin".to_string(),
                    }),
                }]),
            )
            .await?
            .check(&["compiler"], Ok(&["diana".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn multiple_reviewers() {
    run_db_test(|ctx| async move {
        let users = &[user("martin", 1), user("jana", 2), user("mark", 3)];
        let teams = toml::toml!(team = ["martin", "jana", "mark", "diana"]);
        review_prefs_test(ctx)
            .teams(&teams)
            .set_review_prefs(&users[0], Some(3), RotationMode::OnRotation)
            .await
            .set_review_prefs(&users[1], Some(4), RotationMode::OnRotation)
            .await
            .set_review_prefs(&users[2], Some(2), RotationMode::OnRotation)
            .await
            .assign_prs(users[0].id, 4)
            .assign_prs(users[1].id, 2)
            .assign_prs(users[2].id, 2)
            .check(&["team"], Ok(&["diana".into(), "jana".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn circular_groups() {
    // A cycle in the groups map.
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["other"]
        other = ["compiler"]
    );

    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().call())
            .check(
                &["compiler"],
                Err(FindReviewerError::NoReviewer {
                    initial: vec!["compiler".to_string()],
                }),
            )
            .await
    })
    .await;
}

#[tokio::test]
async fn nested_groups() {
    // Test choosing a reviewer from group with nested groups.
    let config = toml::toml!(
        [adhoc_groups]
        a = ["@pnkfelix"]
        b = ["@nrc"]
        c = ["a", "b"]
    );
    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().call())
            .check(&["c"], Ok(&["nrc".into(), "pnkfelix".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn candidate_filtered_author_only_candidate() {
    // When the author is the only candidate.
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["nikomatsakis"]
    );
    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().author(user("nikomatsakis", 1)).call())
            .check(
                &["compiler"],
                Err(FindReviewerError::NoReviewer {
                    initial: vec!["compiler".to_string()],
                }),
            )
            .await
    })
    .await;
}

#[tokio::test]
async fn candidate_filtered_author() {
    // Filter out the author from the candidates.
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["user1", "user2", "user3", "group2"]
        group2 = ["user2", "user4"]
    );
    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().author(user("user2", 1)).call())
            .check(
                &["compiler"],
                Ok(&["user1".into(), "user3".into(), "user4".into()]),
            )
            .await
    })
    .await;
}

#[tokio::test]
async fn candidate_filtered_assignee() {
    // Filter out an existing assignee from the candidates.
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["user1", "user2", "user3", "user4"]
    );
    let issue = issue()
        .author(user("user2", 2))
        .assignees(vec![user("user1", 1), user("user3", 3)])
        .call();
    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue)
            .check(&["compiler"], Ok(&["user4".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn groups_teams_users() {
    // Assortment of groups, teams, and users all selected at once.
    let teams = toml::toml!(
        team1 = ["t-user1"]
        team2 = ["t-user2"]
    );
    let config = toml::toml!(
        [adhoc_groups]
        group1 = ["user1", "rust-lang/team2"]
    );
    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().call())
            .teams(&teams)
            .check(
                &["team1", "group1", "user3"],
                Ok(&[
                    "t-user1".into(),
                    "t-user2".into(),
                    "user1".into(),
                    "user3".into(),
                ]),
            )
            .await
    })
    .await;
}

#[tokio::test]
async fn group_team_user_precedence() {
    // How it handles ambiguity when names overlap.
    let teams = toml::toml!(compiler = ["t-user1"]);
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["user2"]
    );
    run_db_test(|ctx| async move {
        let ctx = basic_test(ctx, config.clone(), issue().call())
            .teams(&teams)
            .check(&["compiler"], Ok(&["user2".into()]))
            .await?;

        basic_test(ctx.into(), config, issue().call())
            .teams(&teams)
            .check(&["rust-lang/compiler"], Ok(&["user2".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn what_do_slashes_mean() {
    // How slashed names are handled.
    let teams = toml::toml!(compiler = ["t-user1"]);
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["user2"]
        "foo/bar" = ["foo-user"]
    );
    let issue = || issue().org("rust-lang-nursery").call();

    run_db_test(|ctx| async move {
        // Random slash names should work from groups.
        basic_test(ctx, config, issue())
            .teams(&teams)
            .check(&["foo/bar"], Ok(&["foo-user".into()]))
            .await?
            .check(&["rust-lang-nursery/compiler"], Ok(&["user2".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn invalid_org_doesnt_match() {
    let teams = toml::toml!(compiler = ["t-user1"]);
    let config = toml::toml!(
        [adhoc_groups]
        compiler = ["user2"]
    );
    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().call())
            .teams(&teams)
            .check(
                &["github/compiler"],
                Err(FindReviewerError::TeamNotFound(
                    "github/compiler".to_string(),
                )),
            )
            .await
    })
    .await;
}

#[tokio::test]
async fn users_on_vacation() {
    let teams = toml::toml!(bootstrap = ["jyn514", "Mark-Simulacrum"]);
    let config = toml::toml!(users_on_vacation = ["jyn514"]);

    run_db_test(|ctx| async move {
        basic_test(ctx, config, issue().call())
            .teams(&teams)
            // Allow direct assignment
            .check(
                &["jyn514"],
                Ok(&[ReviewerSelection {
                    name: "jyn514".to_string(),
                    suppressed_error: Some(FindReviewerError::ReviewerOffRotation {
                        username: "jyn514".to_string(),
                    }),
                }]),
            )
            .await?
            // But ignore the user when requesting a team
            .check(&["bootstrap"], Ok(&["Mark-Simulacrum".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn previous_reviewers_ignore_in_team_success() {
    let teams = toml::toml!(compiler = ["martin", "jyn514"]);
    let config = toml::Table::new();
    run_db_test(|ctx| async move {
        let user = user("martin", 1);
        basic_test(ctx, config, issue().call())
            .teams(&teams)
            .set_previous_reviewers(HashSet::from([&user]))
            .await
            .check(&["compiler"], Ok(&["jyn514".into()]))
            .await
    })
    .await;
}

#[tokio::test]
async fn previous_reviewers_ignore_in_team_failed() {
    let teams = toml::toml!(compiler = ["martin", "jyn514"]);
    let config = toml::Table::new();
    run_db_test(|ctx| async move {
        let user1 = user("martin", 1);
        let user2 = user("jyn514", 2);
        basic_test(ctx, config, issue().call())
            .teams(&teams)
            .set_previous_reviewers(HashSet::from([&user1, &user2]))
            .await
            .check(
                &["compiler"],
                Err(FindReviewerError::NoReviewer {
                    initial: vec!["compiler".to_string()],
                }),
            )
            .await
    })
    .await
}

#[tokio::test]
async fn previous_reviewers_direct_assignee() {
    let teams = toml::toml!(compiler = ["martin", "jyn514"]);
    let config = toml::Table::new();
    run_db_test(|ctx| async move {
        let user1 = user("martin", 1);
        let user2 = user("jyn514", 2);
        basic_test(ctx, config, issue().call())
            .teams(&teams)
            .set_previous_reviewers(HashSet::from([&user1, &user2]))
            .await
            .check(&["jyn514"], Ok(&["jyn514".into()]))
            .await
    })
    .await
}
