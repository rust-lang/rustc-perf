use crate::github::{Issue, IssueState, Label, PullRequestDetails, User};
use bon::builder;
use chrono::Utc;

pub fn default_test_user() -> User {
    User {
        login: "triagebot-tester".to_string(),
        id: 1,
    }
}

pub fn user(login: &str, id: u64) -> User {
    User {
        login: login.to_string(),
        id,
    }
}

#[builder]
pub fn issue(
    state: Option<IssueState>,
    number: Option<u64>,
    author: Option<User>,
    body: Option<&str>,
    assignees: Option<Vec<User>>,
    pr: Option<bool>,
    org: Option<&str>,
    repo: Option<&str>,
    labels: Option<Vec<&str>>,
) -> Issue {
    let number = number.unwrap_or(1);
    let state = state.unwrap_or(IssueState::Open);
    let author = author.unwrap_or(default_test_user());
    let body = body.unwrap_or("").to_string();
    let assignees = assignees.unwrap_or_default();
    let pull_request = if pr.unwrap_or(false) {
        Some(PullRequestDetails::new())
    } else {
        None
    };
    let org = org.unwrap_or("rust-lang");
    let repo = repo.unwrap_or("rust");
    let labels = labels
        .unwrap_or_default()
        .into_iter()
        .map(|l| Label {
            name: l.to_string(),
        })
        .collect();

    Issue {
        number,
        body,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        merge_commit_sha: None,
        title: format!("Issue #{number}"),
        html_url: format!("https://github.com/{org}/{repo}/pull/{number}"),
        user: author,
        labels,
        assignees,
        pull_request,
        merged: false,
        draft: false,
        comments: None,
        // The repository is parsed from comments_url, so this field is important
        comments_url: format!("https://api.github.com/repos/{org}/{repo}/issues/{number}/comments"),
        repository: Default::default(),
        base: None,
        head: None,
        state,
        milestone: None,
        mergeable: None,
        author_association: octocrab::models::AuthorAssociation::None,
    }
}

#[builder]
pub fn pull_request(
    state: Option<IssueState>,
    number: Option<u64>,
    author: Option<User>,
    body: Option<&str>,
    assignees: Option<Vec<User>>,
    labels: Option<Vec<&str>>,
) -> Issue {
    issue()
        .maybe_state(state)
        .maybe_number(number)
        .maybe_author(author)
        .maybe_body(body)
        .maybe_labels(labels)
        .maybe_assignees(assignees)
        .pr(true)
        .call()
}
