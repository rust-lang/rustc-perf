//! Purpose: When opening a PR, or pushing new changes, check for merge commits
//! and notify the user of our no-merge policy.

use crate::{
    config::NoMergesConfig,
    github::{GithubCommit, Repository},
};
use std::collections::BTreeSet;
use std::fmt::Write;

pub(super) fn merges_in_commits(
    issue_title: &str,
    repository: &Repository,
    config: &NoMergesConfig,
    commits: &Vec<GithubCommit>,
) -> Option<(String, Vec<String>)> {
    // Don't trigger if the PR has any of the excluded title segments.
    if config
        .exclude_titles
        .iter()
        .any(|s| issue_title.contains(s))
    {
        return None;
    }

    let mut merge_commits = BTreeSet::new();
    for commit in commits {
        if commit.parents.len() > 1 {
            merge_commits.insert(&*commit.sha);
        }
    }

    // No merge commits.
    if merge_commits.is_empty() {
        return None;
    }

    let message = config
        .message
        .as_deref()
        .unwrap_or(&get_default_message(
            &repository.full_name,
            &repository.default_branch,
            merge_commits.into_iter(),
        ))
        .to_string();

    Some((message, config.labels.clone()))
}

fn get_default_message<'a>(
    repository_name: &str,
    default_branch: &str,
    commits: impl IntoIterator<Item = &'a str>,
) -> String {
    let mut message = format!(
        "The following commits have merge commits (commits with multiple parents) in your changes. \
We have a [no merge policy](https://rustc-dev-guide.rust-lang.org/git.html#no-merge-policy) \
so these commits will need to be removed for this pull request to be merged.
"
    );

    for commit in commits {
        writeln!(message, "- {commit}").unwrap();
    }

    writeln!(
        message,
        "
You can start a rebase with the following commands:
```shell-session
$ # rebase
$ git pull --rebase https://github.com/{repository_name}.git {default_branch}
$ git push --force-with-lease
```"
    )
    .unwrap();

    message
}

#[test]
fn end_to_end() {
    use super::dummy_commit_from_body;
    use crate::github::Parent;

    let config = NoMergesConfig {
        exclude_titles: vec!["Subtree update".to_string()],
        labels: vec!["merge-commits".to_string()],
        message: None,
    };

    let title = "This a PR!";
    let repository = Repository {
        full_name: "rust-lang/rust".to_string(),
        default_branch: "master".to_string(),
        fork: false,
        parent: None,
    };

    let commit_with_merge = || {
        let mut commit_with_merge =
            dummy_commit_from_body("9cc6dce67c917fe5937e984f58f5003ccbb5c37e", "+ main.rs");
        commit_with_merge.parents = vec![
            Parent {
                sha: "4fb1228e72cf76549e275c38c226c1c2326ca991".to_string(),
            },
            Parent {
                sha: "febd545030008f13541064895ae36e19d929a043".to_string(),
            },
        ];
        commit_with_merge
    };

    // contains merges
    {
        let Some((warning, labels)) = merges_in_commits(
            &title,
            &repository,
            &config,
            &vec![
                commit_with_merge(),
                dummy_commit_from_body("499bdd2d766f98420c66a80a02b7d3ceba4d06ba", "+ nothing.rs"),
            ],
        ) else {
            unreachable!()
        };
        assert_eq!(warning, "The following commits have merge commits (commits with multiple parents) in your changes. We have a [no merge policy](https://rustc-dev-guide.rust-lang.org/git.html#no-merge-policy) so these commits will need to be removed for this pull request to be merged.
- 9cc6dce67c917fe5937e984f58f5003ccbb5c37e

You can start a rebase with the following commands:
```shell-session
$ # rebase
$ git pull --rebase https://github.com/rust-lang/rust.git master
$ git push --force-with-lease
```
");
        assert_eq!(labels, vec!["merge-commits"]);
    }

    // doesn't contains merges
    {
        let commit = dummy_commit_from_body("67c917fe5937e984f58f5003ccbb5c37e", "+ main.rs");

        assert_eq!(
            merges_in_commits(&title, &repository, &config, &vec![commit]),
            None
        );
    }

    // contains merges, but excluded by title
    {
        assert_eq!(
            merges_in_commits(
                "Subtree update of rustc_custom_codegen",
                &repository,
                &config,
                &vec![commit_with_merge()]
            ),
            None
        );
    }
}
