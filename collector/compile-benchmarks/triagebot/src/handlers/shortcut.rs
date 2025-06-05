//! Purpose: Allow the use of single words shortcut to do specific actions on GitHub via comments.
//!
//! Parsing is done in the `parser::command::shortcut` module.

use crate::{
    config::ShortcutConfig,
    db::issue_data::IssueData,
    github::{Event, Label},
    handlers::Context,
    interactions::ErrorComment,
};
use octocrab::models::AuthorAssociation;
use parser::command::shortcut::ShortcutCommand;

/// Key for the state in the database
const AUTHOR_REMINDER_KEY: &str = "author-reminder";

/// State stored in the database for a PR.
#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
struct AuthorReminderState {
    /// ID of the reminder comment.
    reminder_comment: Option<String>,
}

pub(super) async fn handle_command(
    ctx: &Context,
    _config: &ShortcutConfig,
    event: &Event,
    input: ShortcutCommand,
) -> anyhow::Result<()> {
    let issue = event.issue().unwrap();
    // NOTE: if shortcuts available to issues are created, they need to be allowed here
    if !issue.is_pr() {
        let msg = format!("The \"{:?}\" shortcut only works on pull requests.", input);
        let cmnt = ErrorComment::new(&issue, msg);
        cmnt.post(&ctx.github).await?;
        return Ok(());
    }

    let issue_labels = issue.labels();
    let waiting_on_review = "S-waiting-on-review";
    let waiting_on_author = "S-waiting-on-author";
    let blocked = "S-blocked";
    let status_labels = [waiting_on_review, waiting_on_author, blocked, "S-inactive"];

    let add = match input {
        ShortcutCommand::Ready => waiting_on_review,
        ShortcutCommand::Author => waiting_on_author,
        ShortcutCommand::Blocked => blocked,
    };

    if !issue_labels.iter().any(|l| l.name == add) {
        for remove in status_labels {
            if remove != add {
                issue.remove_label(&ctx.github, remove).await?;
            }
        }
        issue
            .add_labels(
                &ctx.github,
                vec![Label {
                    name: add.to_owned(),
                }],
            )
            .await?;
    }

    // We add a small reminder for the author to use `@bot ready` when ready
    //
    // Except if the author is a member (or the owner) of the repository, as
    // the author should already know about the `ready` command and already
    // have the required permissions to update the labels manually anyway.
    if matches!(input, ShortcutCommand::Author)
        && !matches!(
            issue.author_association,
            AuthorAssociation::Member | AuthorAssociation::Owner
        )
    {
        // Get the state of the author reminder for this PR
        let mut db = ctx.db.get().await;
        let mut state: IssueData<'_, AuthorReminderState> =
            IssueData::load(&mut db, &issue, AUTHOR_REMINDER_KEY).await?;

        if state.data.reminder_comment.is_none() {
            let comment_body = format!(
                "Reminder, once the PR becomes ready for a review, use `@{bot} ready`.",
                bot = &ctx.username,
            );
            let comment = issue
                .post_comment(&ctx.github, comment_body.as_str())
                .await?;

            state.data.reminder_comment = Some(comment.node_id);
            state.save().await?;
        }
    }

    Ok(())
}
