use std::fmt::Write;

use anyhow::{bail, Context as _};
use octocrab::models::AuthorAssociation;

use crate::{
    config::ConcernConfig,
    github::{Event, Label},
    handlers::Context,
    interactions::EditIssueBody,
};
use parser::command::concern::ConcernCommand;

const CONCERN_ISSUE_KEY: &str = "CONCERN-ISSUE";

#[derive(Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
struct ConcernData {
    concerns: Vec<Concern>,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Concern {
    title: String,
    author: String,
    comment_url: String,
    status: ConcernStatus,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum ConcernStatus {
    Active,
    Resolved { comment_url: String },
}

pub(super) async fn handle_command(
    ctx: &Context,
    config: &ConcernConfig,
    event: &Event,
    cmd: ConcernCommand,
) -> anyhow::Result<()> {
    let Event::IssueComment(issue_comment) = event else {
        bail!("concern issued on something other than a issue")
    };
    let Some(comment_url) = event.html_url() else {
        bail!("unable to retrieve the comment url")
    };
    let author = event.user().login.to_owned();
    let issue = &issue_comment.issue;

    // Verify that this issue isn't a rfcbot FCP, skip if it is
    match crate::rfcbot::get_all_fcps().await {
        Ok(fcps) => {
            if fcps.iter().any(|(_, fcp)| {
                fcp.issue.number as u64 == issue.number
                    && fcp.issue.repository == issue_comment.repository.full_name
            }) {
                tracing::info!(
                    "{}#{} tried to register a concern, blocked by our rfcbot FCP check",
                    issue_comment.repository.full_name,
                    issue.number,
                );
                return Ok(());
            }
        }
        Err(err) => {
            tracing::warn!(
                "unable to fetch rfcbot active FCPs: {:?}, skipping check",
                err
            );
        }
    }

    // Verify that the comment author is at least a member of the org, error if it's not
    if !matches!(
        issue_comment.comment.author_association,
        AuthorAssociation::Member | AuthorAssociation::Owner
    ) {
        issue
            .post_comment(
                &ctx.github,
                "Only organization members can add or resolve concerns.",
            )
            .await?;
        return Ok(());
    }

    let edit = EditIssueBody::new(&issue, CONCERN_ISSUE_KEY);
    let mut concern_data: ConcernData = edit.current_data().unwrap_or_default();

    // Process the command by either adding a new comment or "deactivating" the old one
    match cmd {
        ConcernCommand::Concern { title } => concern_data.concerns.push(Concern {
            title,
            author,
            status: ConcernStatus::Active,
            comment_url: comment_url.to_string(),
        }),
        ConcernCommand::Resolve { title } => concern_data
            .concerns
            .iter_mut()
            .filter(|c| c.title == title)
            .for_each(|c| {
                c.status = ConcernStatus::Resolved {
                    comment_url: comment_url.to_string(),
                }
            }),
    }

    // Create the new markdown content listing all the concerns
    let new_content = markdown_content(&concern_data.concerns, &ctx.username);

    // Add or remove the labels
    if concern_data
        .concerns
        .iter()
        .any(|c| matches!(c.status, ConcernStatus::Active))
    {
        if let Err(err) = issue
            .add_labels(
                &ctx.github,
                config
                    .labels
                    .iter()
                    .map(|l| Label {
                        name: l.to_string(),
                    })
                    .collect(),
            )
            .await
        {
            tracing::error!("unable to add concern labels: {:?}", err);
            let labels = config.labels.join(", ");
            issue.post_comment(
                &ctx.github,
                &format!("*Psst, I was unable to add the labels ({labels}), could someone do it for me.*"),
            ).await?;
        }
    } else {
        for l in &config.labels {
            issue.remove_label(&ctx.github, &l).await?;
        }
    }

    // Apply the new markdown concerns list to the issue
    edit.apply(&ctx.github, new_content, concern_data)
        .await
        .context("failed to apply the new concerns section markdown")?;

    Ok(())
}

fn markdown_content(concerns: &[Concern], bot: &str) -> String {
    if concerns.is_empty() {
        return "".to_string();
    }

    let mut md = String::new();

    let _ = writeln!(md, "\n# Concerns");
    let _ = writeln!(md, "");

    for &Concern {
        ref title,
        ref author,
        ref status,
        ref comment_url,
    } in concerns
    {
        let _ = match status {
            ConcernStatus::Active => {
                writeln!(
                    md,
                    " - [{title}]({comment_url}) by [{author}](https://github.com/{author})"
                )
            }
            ConcernStatus::Resolved {
                comment_url: resolved_comment_url,
            } => {
                writeln!(
                    md,
                    " - ~~[{title}]({comment_url}) by [{author}](https://github.com/{author})~~ resolved [in this comment]({resolved_comment_url})"
                )
            }
        };
    }

    let _ = writeln!(md, "");
    let _ = writeln!(md, "*Managed by `@{bot}`—see [help](https://forge.rust-lang.org/triagebot/concern.html) for details.*");

    md
}

#[test]
fn simple_markdown_content() {
    let concerns = &[
        Concern {
            title: "This is my concern about concern".to_string(),
            author: "Urgau".to_string(),
            status: ConcernStatus::Active,
            comment_url: "https://github.com/fake-comment-1234".to_string(),
        },
        Concern {
            title: "This is a resolved concern".to_string(),
            author: "Kobzol".to_string(),
            status: ConcernStatus::Resolved {
                comment_url: "https:://github.com/fake-comment-8888".to_string(),
            },
            comment_url: "https://github.com/fake-comment-4561".to_string(),
        },
    ];

    assert_eq!(
        markdown_content(concerns, "rustbot"),
        r#"
# Concerns

 - [This is my concern about concern](https://github.com/fake-comment-1234) by [Urgau](https://github.com/Urgau)
 - ~~[This is a resolved concern](https://github.com/fake-comment-4561) by [Kobzol](https://github.com/Kobzol)~~ resolved [in this comment](https:://github.com/fake-comment-8888)

*Managed by `@rustbot`—see [help](https://forge.rust-lang.org/triagebot/concern.html) for details.*
"#
    );
}
