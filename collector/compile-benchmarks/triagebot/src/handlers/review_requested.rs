use crate::config::ReviewRequestedConfig;
use crate::github::{IssuesAction, IssuesEvent, Label};
use crate::handlers::Context;

pub(crate) struct ReviewRequestedInput {}

pub(crate) async fn parse_input(
    _ctx: &Context,
    event: &IssuesEvent,
    config: Option<&ReviewRequestedConfig>,
) -> Result<Option<ReviewRequestedInput>, String> {
    // PR author requests a review from one of the assignees

    if config.is_none() {
        return Ok(None);
    }

    let IssuesAction::ReviewRequested {
        requested_reviewer: Some(requested_reviewer),
    } = &event.action
    else {
        return Ok(None);
    };

    if event.sender != event.issue.user {
        return Ok(None);
    }

    if !event.issue.assignees.contains(requested_reviewer) {
        return Ok(None);
    }

    Ok(Some(ReviewRequestedInput {}))
}

pub(crate) async fn handle_input(
    ctx: &Context,
    config: &ReviewRequestedConfig,
    event: &IssuesEvent,
    ReviewRequestedInput {}: ReviewRequestedInput,
) -> anyhow::Result<()> {
    event
        .issue
        .add_labels(
            &ctx.github,
            config
                .add_labels
                .iter()
                .cloned()
                .map(|name| Label { name })
                .collect(),
        )
        .await?;

    for label in &config.remove_labels {
        event.issue.remove_label(&ctx.github, label).await?;
    }

    Ok(())
}
