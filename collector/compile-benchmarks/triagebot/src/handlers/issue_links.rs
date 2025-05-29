//! This handler is used to canonicalize linked GitHub issues into their long form
//! so that when pulling subtree into the main repository we don't accidentaly
//! close issues in the wrong repository.
//!
//! Example: `Fixes #123` (in rust-lang/clippy) would now become `Fixes rust-lang/clippy#123`

use std::borrow::Cow;
use std::sync::LazyLock;

use regex::Regex;

use crate::{
    config::IssueLinksConfig,
    github::{IssuesAction, IssuesEvent},
    handlers::Context,
};

static LINKED_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\B(?P<issue>#[0-9]+)\b").unwrap());

pub(super) struct IssueLinksInput {}

pub(super) async fn parse_input(
    _ctx: &Context,
    event: &IssuesEvent,
    config: Option<&IssueLinksConfig>,
) -> Result<Option<IssueLinksInput>, String> {
    if !event.issue.is_pr() {
        return Ok(None);
    }

    if !matches!(
        event.action,
        IssuesAction::Opened | IssuesAction::Reopened | IssuesAction::Edited
    ) {
        return Ok(None);
    }

    // Require a `[issue-links]` (or it's alias `[canonicalize-issue-links]`)
    // configuration block to enable the handler.
    if config.is_none() {
        return Ok(None);
    };

    Ok(Some(IssueLinksInput {}))
}

pub(super) async fn handle_input(
    ctx: &Context,
    _config: &IssueLinksConfig,
    e: &IssuesEvent,
    _input: IssueLinksInput,
) -> anyhow::Result<()> {
    let full_repo_name = e.issue.repository().full_repo_name();

    let new_body = fix_linked_issues(&e.issue.body, full_repo_name.as_str());

    if e.issue.body != new_body {
        e.issue.edit_body(&ctx.github, &new_body).await?;
    }

    Ok(())
}

fn fix_linked_issues<'a>(body: &'a str, full_repo_name: &str) -> Cow<'a, str> {
    let replace_by = format!("{full_repo_name}${{issue}}");
    parser::replace_all_outside_ignore_blocks(&LINKED_RE, body, replace_by)
}

#[test]
fn fixed_body() {
    let full_repo_name = "rust-lang/rust";

    let body = r#"
This is a PR, which links to #123.

Fix #123
fixed #456
Fixes    #7895
Fixesd #7895
Closes: #987
resolves:   #655
Resolves #00000 Closes #888
    "#;

    let fixed_body = r#"
This is a PR, which links to rust-lang/rust#123.

Fix rust-lang/rust#123
fixed rust-lang/rust#456
Fixes    rust-lang/rust#7895
Fixesd rust-lang/rust#7895
Closes: rust-lang/rust#987
resolves:   rust-lang/rust#655
Resolves rust-lang/rust#00000 Closes rust-lang/rust#888
    "#;

    let new_body = fix_linked_issues(body, full_repo_name);
    assert_eq!(new_body, fixed_body);
}

#[test]
fn edge_case_body() {
    let full_repo_name = "rust-lang/rust";

    assert_eq!(
        fix_linked_issues("#132 with a end", full_repo_name),
        "rust-lang/rust#132 with a end"
    );
    assert_eq!(
        fix_linked_issues("with a start #132", full_repo_name),
        "with a start rust-lang/rust#132"
    );
    assert_eq!(
        fix_linked_issues("#132", full_repo_name),
        "rust-lang/rust#132"
    );
    assert_eq!(
        fix_linked_issues("(#132)", full_repo_name),
        "(rust-lang/rust#132)"
    );
}

#[test]
fn untouched_body() {
    let full_repo_name = "rust-lang/rust";

    let body = r#"
This is a PR.

Fix rust-lang#123
Resolves #abgt
Resolves: #abgt
Fixes #157a
Fixes#123
`Fixes #123`

```
Example: Fixes #123
```

<!-- Fixes #123 -->
    "#;

    let new_body = fix_linked_issues(body, full_repo_name);
    assert_eq!(new_body, body);
}
