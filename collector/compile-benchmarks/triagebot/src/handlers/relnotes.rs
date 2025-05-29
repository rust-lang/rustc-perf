//! This handler implements collecting release notes from issues and PRs that are tagged with
//! `relnotes`. Any such tagging will open a new issue in rust-lang/rust responsible for tracking
//! the inclusion in releases notes.
//!
//! The new issue will be closed when T-release has added the text proposed (tracked in the issue
//! description) into the final release notes PR.
//!
//! The issue description will be edited manually by teams through the GitHub UI -- in the future,
//! we might add triagebot support for maintaining that text via commands or similar.
//!
//! These issues will also be automatically milestoned when their corresponding PR or issue is. In
//! the absence of a milestone, T-release is responsible for ascertaining which release is
//! associated with the issue.

use serde::{Deserialize, Serialize};

use crate::{
    db::issue_data::IssueData,
    github::{Event, IssuesAction},
    handlers::Context,
};

const RELNOTES_KEY: &str = "relnotes";

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
struct RelnotesState {
    relnotes_issue: Option<u64>,
}

const TITLE_PREFIX: &str = "Tracking issue for release notes";

pub(super) async fn handle(ctx: &Context, event: &Event) -> anyhow::Result<()> {
    let Event::Issue(e) = event else {
        return Ok(());
    };

    let repo = e.issue.repository();
    if !(repo.organization == "rust-lang" && repo.repository == "rust") {
        return Ok(());
    }

    if e.issue.title.starts_with(TITLE_PREFIX) {
        // Ignore these issues -- they're otherwise potentially self-recursive.
        return Ok(());
    }

    let mut client = ctx.db.get().await;
    let mut state: IssueData<'_, RelnotesState> =
        IssueData::load(&mut client, &e.issue, RELNOTES_KEY).await?;

    if let Some(paired) = state.data.relnotes_issue {
        // Already has a paired release notes issue.

        if let IssuesAction::Milestoned = &e.action {
            if let Some(milestone) = &e.issue.milestone {
                ctx.github
                    .set_milestone(&e.issue.repository().to_string(), &milestone, paired)
                    .await?;
            }
        }

        return Ok(());
    }

    if let IssuesAction::Labeled { label } = &e.action {
        let is_fcp_merge = label.name == "finished-final-comment-period"
            && e.issue
                .labels
                .iter()
                .any(|label| label.name == "disposition-merge");

        if label.name == "relnotes" || label.name == "relnotes-perf" || is_fcp_merge {
            let title = format!("{TITLE_PREFIX} of #{}: {}", e.issue.number, e.issue.title);
            let body = format!(
                "
This issue tracks the release notes text for #{pr_number}.

cc {people} -- original issue/PR authors and assignees for drafting text

See the forge.rust-lang.org chapter about [release notes](https://forge.rust-lang.org/release/release-notes.html#preparing-release-notes) for an overview of how the release team makes use of these tracking issues.

### Release notes text

This section should be edited to specify the correct category(s) for the change, with succinct description(s) of what changed. Some things worth considering:
- Does this need an additional compat notes section?
- Was this a libs stabilization that should have additional headers to list new APIs under `Stabilized APIs` and `Const Stabilized APIs`?


````markdown
# Language/Compiler/Libraries/Stabilized APIs/Const Stabilized APIs/Rustdoc/Compatibility Notes/Internal Changes/Other
- [{pr_title}]({pr_url})
````

> [!TIP]
> Use the [previous releases](https://doc.rust-lang.org/nightly/releases.html) for inspiration on how to write the release notes text and which categories to pick.

### Release blog section

If this change is notable enough for inclusion in the blog post then this section should be edited to contain a draft for the blog post. *Otherwise leave it empty.*


````markdown
````

> [!NOTE]
>
> If a blog post section is required the `release-blog-post` label should be added (`@rustbot label +release-blog-post`) to this issue as otherwise it may be missed by the release team.
",
                pr_number = e.issue.number,
                people = [&e.issue.user].into_iter().chain(e.issue.assignees.iter())
                    .map(|v| format!("@{}", v.login)).collect::<Vec<_>>().join(", "),
                pr_title = e.issue.title,
                pr_url = e.issue.html_url,
            );
            let resp = ctx
                .github
                .new_issue(
                    &e.issue.repository(),
                    &title,
                    &body,
                    ["relnotes", "relnotes-tracking-issue"]
                        .into_iter()
                        .chain(e.issue.labels.iter().map(|l| &*l.name).filter(|l| {
                            l.starts_with("A-") // A-* (area)
                            || l.starts_with("F-") // F-* (feature)
                            || l.starts_with("L-") // L-* (lint)
                            || l.starts_with("O-") // O-* (OS)
                            || l.starts_with("T-") // T-* (team)
                            || l.starts_with("WG-") // WG-* (working group)
                        }))
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>(),
                )
                .await?;
            if let Some(milestone) = &e.issue.milestone {
                ctx.github
                    .set_milestone(&e.issue.repository().to_string(), &milestone, resp.number)
                    .await?;
            }
            state.data.relnotes_issue = Some(resp.number);
            state.save().await?;
        }
    }

    Ok(())
}
