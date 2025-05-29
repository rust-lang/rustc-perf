//! Purpose: Allow any user to ping a pre-selected group of people on GitHub via comments.
//!
//! The set of "teams" which can be pinged is intentionally restricted via configuration.
//!
//! Parsing is done in the `parser::command::ping` module.

use crate::{
    config::PingConfig,
    github::{self, Event},
    handlers::Context,
    interactions::ErrorComment,
};
use parser::command::ping::PingCommand;

pub(super) async fn handle_command(
    ctx: &Context,
    config: &PingConfig,
    event: &Event,
    team_name: PingCommand,
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
            format!("Only Rust team members can ping teams."),
        );
        cmnt.post(&ctx.github).await?;
        return Ok(());
    }

    let (gh_team, config) = match config.get_by_name(&team_name.team) {
        Some(v) => v,
        None => {
            let cmnt = ErrorComment::new(
                &event.issue().unwrap(),
                format!(
                    "This team (`{}`) cannot be pinged via this command; \
                    it may need to be added to `triagebot.toml` on the default branch.",
                    team_name.team,
                ),
            );
            cmnt.post(&ctx.github).await?;
            return Ok(());
        }
    };
    let team = github::get_team(&ctx.github, &gh_team).await?;
    let team = match team {
        Some(team) => team,
        None => {
            let cmnt = ErrorComment::new(
                &event.issue().unwrap(),
                format!(
                    "This team (`{}`) does not exist in the team repository.",
                    team_name.team,
                ),
            );
            cmnt.post(&ctx.github).await?;
            return Ok(());
        }
    };

    if let Some(label) = &config.label {
        if let Err(err) = event
            .issue()
            .unwrap()
            .add_labels(
                &ctx.github,
                vec![github::Label {
                    name: label.clone(),
                }],
            )
            .await
        {
            let cmnt = ErrorComment::new(
                &event.issue().unwrap(),
                format!("Error adding team label (`{}`): {:?}.", label, err),
            );
            cmnt.post(&ctx.github).await?;
        }
    }

    let mut users = Vec::new();

    if let Some(gh) = team.github {
        let repo = event.issue().expect("has issue").repository();
        // Ping all github teams associated with this team repo team that are in this organization.
        // We cannot ping across organizations, but this should not matter, as teams should be
        // sync'd to the org for which triagebot is configured.
        for gh_team in gh.teams.iter().filter(|t| t.org == repo.organization) {
            users.push(format!("@{}/{}", gh_team.org, gh_team.name));
        }
    } else {
        for member in &team.members {
            users.push(format!("@{}", member.github));
        }
    }

    let ping_msg = if users.is_empty() {
        format!("no known users to ping?")
    } else {
        format!("cc {}", users.join(" "))
    };
    let comment = format!("{}\n\n{}", config.message, ping_msg);
    event
        .issue()
        .expect("issue")
        .post_comment(&ctx.github, &comment)
        .await?;

    Ok(())
}
