//! Handles the `@rustbot transfer reponame` command to transfer an issue to
//! another repository.

use crate::{config::TransferConfig, github::Event, handlers::Context};
use parser::command::transfer::TransferCommand;

pub(super) async fn handle_command(
    ctx: &Context,
    _config: &TransferConfig,
    event: &Event,
    input: TransferCommand,
) -> anyhow::Result<()> {
    let issue = event.issue().unwrap();
    if issue.is_pr() {
        issue
            .post_comment(&ctx.github, "Only issues can be transferred.")
            .await?;
        return Ok(());
    }
    if !event
        .user()
        .is_team_member(&ctx.github)
        .await
        .ok()
        .unwrap_or(false)
    {
        issue
            .post_comment(
                &ctx.github,
                "Only team members may use the `transfer` command.",
            )
            .await?;
        return Ok(());
    }

    let repo = input.0;
    let repo = repo.strip_prefix("rust-lang/").unwrap_or(&repo);
    if repo.contains('/') {
        issue
            .post_comment(&ctx.github, "Cross-organization transfers are not allowed.")
            .await?;
        return Ok(());
    }

    if let Err(e) = issue.transfer(&ctx.github, "rust-lang", &repo).await {
        issue
            .post_comment(&ctx.github, &format!("Failed to transfer issue:\n{e:?}"))
            .await?;
        return Ok(());
    }

    Ok(())
}
