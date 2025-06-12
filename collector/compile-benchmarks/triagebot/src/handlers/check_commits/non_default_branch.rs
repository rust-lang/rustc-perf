use crate::{config::WarnNonDefaultBranchException, github::IssuesEvent};

/// Returns a message if the PR is opened against the non-default branch (or the
/// exception branch if it's an exception).
pub(super) fn non_default_branch(
    exceptions: &[WarnNonDefaultBranchException],
    event: &IssuesEvent,
) -> Option<String> {
    let target_branch = &event.issue.base.as_ref().unwrap().git_ref;

    if let Some(exception) = exceptions
        .iter()
        .find(|e| event.issue.title.contains(&e.title))
    {
        if &exception.branch != target_branch {
            return Some(not_default_exception_branch_warn(
                &exception.branch,
                target_branch,
            ));
        }
    } else if &event.repository.default_branch != target_branch {
        return Some(not_default_branch_warn(
            &event.repository.default_branch,
            target_branch,
        ));
    }
    None
}

fn not_default_branch_warn(default: &str, target: &str) -> String {
    format!(
        "Pull requests are usually filed against the {default} branch for this repo, \
         but this one is against {target}. \
         Please double check that you specified the right target!"
    )
}

fn not_default_exception_branch_warn(default: &str, target: &str) -> String {
    format!(
        "Pull requests targetting the {default} branch are usually filed against the {default} \
         branch, but this one is against {target}. \
         Please double check that you specified the right target!"
    )
}
