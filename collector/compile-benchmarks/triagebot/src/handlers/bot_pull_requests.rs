use crate::github::{IssuesAction, PrState};
use crate::{github::Event, handlers::Context};

pub(crate) async fn handle(ctx: &Context, event: &Event) -> anyhow::Result<()> {
    let Event::Issue(event) = event else {
        return Ok(());
    };
    // Note that this filters out reopened too, which is what we'd expect when we set the state
    // back to opened after closing.
    if event.action != IssuesAction::Opened {
        return Ok(());
    }
    if !event.issue.is_pr() {
        return Ok(());
    }

    // avoid acting on our own open events, otherwise we'll infinitely loop
    if event.sender.login == ctx.username {
        return Ok(());
    }

    // If it's not the github-actions bot, we don't expect this handler to be needed. Skip the
    // event.
    if event.sender.login != "github-actions[bot]" {
        return Ok(());
    }

    ctx.github
        .set_pr_state(
            event.issue.repository(),
            event.issue.number,
            PrState::Closed,
        )
        .await?;
    ctx.github
        .set_pr_state(event.issue.repository(), event.issue.number, PrState::Open)
        .await?;

    Ok(())
}
