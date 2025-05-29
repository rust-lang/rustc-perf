use crate::github::{GithubCompare, IssuesEvent};
use tracing as log;

/// Default threshold for parent commit age in days to trigger a warning
pub(super) const DEFAULT_DAYS_THRESHOLD: usize = 7;

/// Check if the PR is based on an old parent commit
pub(super) async fn behind_upstream(
    age_threshold: usize,
    event: &IssuesEvent,
    compare: &GithubCompare,
) -> Option<String> {
    log::debug!("Checking if PR #{} is behind upstream", event.issue.number);

    // Compute the number of days old the merge base commit is
    let commit_date = compare.merge_base_commit.commit.author.date;
    let now = chrono::Utc::now().with_timezone(&commit_date.timezone());
    let days_old = (now - commit_date).num_days() as usize;

    let upstream_commit_url = &compare.merge_base_commit.html_url;

    // First try the parent commit age check as it's more accurate
    if days_old > age_threshold {
        log::info!(
            "PR #{} has a parent commit that is {} days old",
            event.issue.number,
            days_old
        );

        Some(format!(
            r"This PR is based on an [upstream commit]({upstream_commit_url}) that is {days_old} days old.

*It's recommended to update your branch according to the [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/contributing.html#keeping-your-branch-up-to-date).*",
        ))
    } else {
        // Parent commit is not too old, log and do nothing
        log::debug!("PR #{} parent commit is not too old", event.issue.number);
        None
    }
}
