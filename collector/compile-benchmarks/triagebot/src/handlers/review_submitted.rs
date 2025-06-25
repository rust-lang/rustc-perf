use crate::github::{Issue, IssueCommentAction, IssueCommentEvent, Label, PullRequestReviewState};
use crate::{config::ReviewSubmittedConfig, github::Event, handlers::Context};

pub(crate) async fn handle(
    ctx: &Context,
    event: &Event,
    config: &ReviewSubmittedConfig,
) -> anyhow::Result<()> {
    if let Event::IssueComment(
        event @ IssueCommentEvent {
            action: IssueCommentAction::Created,
            issue: Issue {
                pull_request: Some(_),
                ..
            },
            ..
        },
    ) = event
    {
        if event.comment.pr_review_state != Some(PullRequestReviewState::ChangesRequested) {
            return Ok(());
        }

        if event.issue.assignees.contains(&event.comment.user) {
            // Remove review labels
            for label in &config.review_labels {
                event.issue.remove_label(&ctx.github, &label).await?;
            }
            // Add waiting on author
            event
                .issue
                .add_labels(
                    &ctx.github,
                    vec![Label {
                        name: config.reviewed_label.clone(),
                    }],
                )
                .await?;
        }
    }

    Ok(())
}
