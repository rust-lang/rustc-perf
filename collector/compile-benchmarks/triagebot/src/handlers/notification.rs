//! Purpose: Allow any user to ping a pre-selected group of people on GitHub via comments.
//!
//! The set of "teams" which can be pinged is intentionally restricted via configuration.
//!
//! Parsing is done in the `parser::command::ping` module.

use crate::db::{notifications, users};
use crate::github::get_id_for_username;
use crate::{
    github::{self, Event},
    handlers::Context,
};
use anyhow::Context as _;
use std::collections::HashSet;
use tracing as log;

pub(super) async fn handle(ctx: &Context, event: &Event) -> anyhow::Result<()> {
    let body = match event.comment_body() {
        Some(v) => v,
        // Skip events that don't have comment bodies associated
        None => return Ok(()),
    };

    if let Event::Issue(e) = event {
        if !matches!(
            e.action,
            github::IssuesAction::Opened | github::IssuesAction::Edited
        ) {
            // no change in issue's body for these events, so skip
            return Ok(());
        }
    }

    let short_description = match event {
        Event::Issue(e) => e.issue.title.clone(),
        Event::IssueComment(e) => format!("Comment on {}", e.issue.title),
        Event::Push(_) | Event::Create(_) => return Ok(()),
    };

    let mut caps = parser::get_mentions(body)
        .into_iter()
        .collect::<HashSet<_>>();

    // FIXME: Remove this hardcoding. Ideally we need organization-wide
    // configuration, but it's unclear where to put it.
    if event.issue().unwrap().repository().organization == "serde-rs" {
        // Only add dtolnay on new issues/PRs, not on comments to old PRs and
        // issues.
        if let Event::Issue(e) = event {
            if e.action == github::IssuesAction::Opened {
                caps.insert("dtolnay");
            }
        }
    }

    // Get the list of users already notified by a previous version of this
    // comment, so they don't get notified again
    let mut users_notified = HashSet::new();
    if let Some(from) = event.comment_from() {
        for login in parser::get_mentions(from).into_iter() {
            if let Some((users, _)) = id_from_user(ctx, login).await? {
                users_notified.extend(users.into_iter().map(|user| user.id));
            }
        }
    };

    // We've implicitly notified the user that is submitting the notification:
    // they already know that they left this comment.
    //
    // If the user intended to ping themselves, they can add the GitHub comment
    // via the Zulip interface.
    match get_id_for_username(&ctx.github, &event.user().login).await {
        Ok(Some(id)) => {
            users_notified.insert(id.try_into().unwrap());
        }
        Ok(None) => {}
        Err(err) => {
            log::error!("Failed to query ID for {:?}: {:?}", event.user(), err);
        }
    }
    log::trace!("Captured usernames in comment: {:?}", caps);
    for login in caps {
        let (users, team_name) = match id_from_user(ctx, login).await? {
            Some((users, team_name)) => (users, team_name),
            None => continue,
        };

        let client = ctx.db.get().await;
        for user in users {
            if !users_notified.insert(user.id) {
                // Skip users already associated with this event.
                continue;
            }

            if let Err(err) = users::record_username(&client, user.id, &user.login)
                .await
                .context("failed to record username")
            {
                log::error!("record username: {:?}", err);
            }

            if let Err(err) = notifications::record_ping(
                &client,
                &notifications::Notification {
                    user_id: user.id,
                    origin_url: event.html_url().unwrap().to_owned(),
                    origin_html: body.to_owned(),
                    time: event.time().unwrap(),
                    short_description: Some(short_description.clone()),
                    team_name: team_name.clone(),
                },
            )
            .await
            .context("failed to record ping")
            {
                log::error!("record ping: {:?}", err);
            }
        }
    }

    Ok(())
}

async fn id_from_user(
    ctx: &Context,
    login: &str,
) -> anyhow::Result<Option<(Vec<github::User>, Option<String>)>> {
    if let Some((org, team)) = login.split_once('/') {
        // This is a team ping. For now, just add it to everyone's agenda on
        // that team, but also mark it as such (i.e., a team ping) for
        // potentially different prioritization and so forth.
        //
        // In order to properly handle this down the road, we will want to
        // distinguish between "everyone must pay attention" and "someone
        // needs to take a look."
        //
        // We may also want to be able to categorize into these buckets
        // *after* the ping occurs and is initially processed.
        let team = match github::get_team_by_github_name(&ctx.github, org, team).await {
            Ok(Some(team)) => team,
            Ok(None) => {
                // If the team is in rust-lang*, then this is probably an error (potentially user
                // error, but should be investigated). Otherwise it's probably not going to be in
                // the team repository so isn't actually an error.
                if login.starts_with("rust") {
                    log::error!("team ping ({}) failed to resolve to a known team", login);
                } else {
                    log::info!("team ping ({}) failed to resolve to a known team", login);
                }
                return Ok(None);
            }
            Err(err) => {
                log::error!(
                    "team ping ({}) failed to resolve to a known team: {:?}",
                    login,
                    err
                );
                return Ok(None);
            }
        };

        Ok(Some((
            team.members
                .into_iter()
                .map(|member| github::User {
                    id: member.github_id,
                    login: member.github,
                })
                .collect::<Vec<github::User>>(),
            Some(team.name),
        )))
    } else {
        let id = get_id_for_username(&ctx.github, login)
            .await
            .with_context(|| format!("failed to get user {} ID", login))?;
        let Some(id) = id else {
            // If the user was not in the team(s) then just don't record it.
            log::trace!("Skipping {} because no id found", login);
            return Ok(None);
        };
        Ok(Some((
            vec![github::User {
                login: login.to_string(),
                id,
            }],
            None,
        )))
    }
}
