use std::borrow::Cow;

use anyhow::bail;

use crate::{
    config::RenderedLinkConfig,
    github::{Event, IssuesAction, IssuesEvent},
    handlers::Context,
};

pub(super) async fn handle(
    ctx: &Context,
    event: &Event,
    config: &RenderedLinkConfig,
) -> anyhow::Result<()> {
    let Event::Issue(e) = event else {
        return Ok(());
    };

    if !e.issue.is_pr() {
        return Ok(());
    }

    if let Err(e) = add_rendered_link(&ctx, &e, config).await {
        tracing::error!("Error adding rendered link: {:?}", e);
    }

    Ok(())
}

async fn add_rendered_link(
    ctx: &Context,
    e: &IssuesEvent,
    config: &RenderedLinkConfig,
) -> anyhow::Result<()> {
    if e.action == IssuesAction::Opened
        || e.action == IssuesAction::Closed
        || e.action == IssuesAction::Reopened
        || e.action == IssuesAction::Synchronize
    {
        let files = e.issue.files(&ctx.github).await?;

        let rendered_link = files
            .iter()
            .find(|f| {
                config
                    .trigger_files
                    .iter()
                    .any(|tf| f.filename.starts_with(tf))
            })
            .and_then(|file| {
                let head = e.issue.head.as_ref()?;
                let base = e.issue.base.as_ref()?;

                // This URL should be stable while the PR is open, even if the
                // user pushes new commits.
                //
                // It will go away if the user deletes their branch, or if
                // they reset it (such as if they created a PR from master).
                // That should usually only happen after the PR is closed
                // a which point we switch to a SHA-based url.
                //
                // If the PR is merged we use a URL that points to the actual
                // repository, as to be resilient to branch deletion, as well
                // be in sync with current "master" branch.
                //
                // For a PR "octocat:master" <- "Bob:patch-1", we generate,
                //  - if merged: `https://github.com/octocat/REPO/blob/master/FILEPATH`
                //  - if open: `https://github.com/Bob/REPO/blob/patch-1/FILEPATH`
                //  - if closed: `https://github.com/octocat/REPO/blob/SHA/FILEPATH`
                Some(format!(
                    "[Rendered](https://github.com/{}/blob/{}/{})",
                    if e.issue.merged || e.action == IssuesAction::Closed {
                        &e.repository.full_name
                    } else {
                        &head.repo.as_ref()?.full_name
                    },
                    if e.issue.merged {
                        &base.git_ref
                    } else if e.action == IssuesAction::Closed {
                        &head.sha
                    } else {
                        &head.git_ref
                    },
                    file.filename
                ))
            });

        let new_body: Cow<'_, str> = if !e.issue.body.contains("[Rendered]") {
            if let Some(rendered_link) = rendered_link {
                // add rendered link to the end of the body
                format!("{}\n\n{rendered_link}", e.issue.body).into()
            } else {
                // or return the original body since we don't have
                // a rendered link to add
                e.issue.body.as_str().into()
            }
        } else if let Some(start_pos) = e.issue.body.find("[Rendered](") {
            let Some(end_offset) = &e.issue.body[start_pos..].find(')') else {
                bail!("no `)` after `[Rendered]` found")
            };

            // replace the current rendered link with the new one or replace
            // it with an empty string if we don't have one
            e.issue
                .body
                .replace(
                    &e.issue.body[start_pos..=(start_pos + end_offset)],
                    rendered_link.as_deref().unwrap_or(""),
                )
                .into()
        } else {
            bail!(
                "found `[Rendered]` but not it's associated link, can't replace it or remove it, bailing out"
            )
        };

        // avoid an expensive GitHub api call by first checking if we actually
        // edited the pull request body
        if e.issue.body != new_body {
            e.issue.edit_body(&ctx.github, &new_body).await?;
        }
    }

    Ok(())
}
