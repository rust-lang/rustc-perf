//! Purpose: Allow team members to nominate issues or PRs.

use crate::{
    config::NominateConfig,
    github::{self, Event},
    handlers::Context,
    interactions::ErrorComment,
};
use parser::command::nominate::{NominateCommand, Style};

pub(super) async fn handle_command(
    ctx: &Context,
    config: &NominateConfig,
    event: &Event,
    cmd: NominateCommand,
) -> anyhow::Result<()> {
    let is_team_member = if let Err(_) | Ok(false) = event.user().is_team_member(&ctx.github).await
    {
        false
    } else {
        true
    };

    if !is_team_member {
        let cmnt = ErrorComment::new(
            &event.issue().unwrap(),
            format!(
                "Nominating and approving issues and pull requests is restricted to members of\
                 the Rust teams."
            ),
        );
        cmnt.post(&ctx.github).await?;
        return Ok(());
    }

    let issue_labels = event.issue().unwrap().labels();
    let mut labels_to_add = vec![];
    if cmd.style == Style::BetaApprove {
        if !issue_labels.iter().any(|l| l.name == "beta-nominated") {
            let cmnt = ErrorComment::new(
                &event.issue().unwrap(),
                format!(
                    "This pull request is not beta-nominated, so it cannot be approved yet.\
                     Perhaps try to beta-nominate it by using `@{} beta-nominate <team>`?",
                    ctx.username,
                ),
            );
            cmnt.post(&ctx.github).await?;
            return Ok(());
        }

        // Add the beta-accepted label, but don't attempt to remove beta-nominated or the team
        // label.
        labels_to_add.push(github::Label {
            name: "beta-accepted".into(),
        });
    } else {
        if !config.teams.contains_key(&cmd.team) {
            let cmnt = ErrorComment::new(
                &event.issue().unwrap(),
                format!(
                    "This team (`{}`) cannot be nominated for via this command;\
                     it may need to be added to `triagebot.toml` on the default branch.",
                    cmd.team,
                ),
            );
            cmnt.post(&ctx.github).await?;
            return Ok(());
        }

        let label = config.teams[&cmd.team].clone();
        labels_to_add.push(github::Label { name: label });

        let style_label = match cmd.style {
            Style::Decision => "I-nominated",
            Style::Beta => "beta-nominated",
            Style::BetaApprove => unreachable!(),
        };
        labels_to_add.push(github::Label {
            name: style_label.into(),
        });
    }

    event
        .issue()
        .unwrap()
        .add_labels(&ctx.github, labels_to_add)
        .await?;

    Ok(())
}
