//! Allow users to add summary comments in Issues & Pull Requests.
//!
//! Users can make a new summary entry by commenting the following:
//!
//! ```md
//! @rustbot note Summary title
//! ```
//!
//! If this is the first summary entry, rustbot will amend the original post (the top-level comment) to add a "Notes" section. The section should **not** be edited by hand.
//!
//! ```md
//! <!-- TRIAGEBOT_SUMMARY_START -->
//!
//! ### Summary Notes
//!
//! - [Summary title](link-to-comment) by [username](https://github.com/<username>)
//!
//! *Managed by `@bot`—see [help](https://forge.rust-lang.org/triagebot/note.html) for details*
//! <!-- TRIAGEBOT_SUMMARY_END -->
//! ```
//!
//! If this is *not* the first summary entry, rustbot will simply append the new entry to the existing notes section:
//!
//! ```md
//! <!-- TRIAGEBOT_SUMMARY_START -->
//!
//! ### Summary Notes
//!
//! - [First note](link-to-comment) by [username](https://github.com/<username>)
//! - [Second note](link-to-comment) by [username](https://github.com/<username>)
//! - [Summary title](link-to-comment) by [username](https://github.com/<username>)
//!
//! <!-- TRIAGEBOT_SUMMARY_END -->
//! ```
//!

use crate::{config::NoteConfig, github::Event, handlers::Context, interactions::EditIssueBody};
use itertools::Itertools;
use parser::command::note::NoteCommand;
use std::fmt::Write;
use std::{cmp::Ordering, collections::HashMap};
use tracing as log;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
struct NoteDataEntry {
    title: String,
    comment_url: String,
    author: String,
}

impl NoteDataEntry {
    pub(crate) fn to_markdown(&self) -> String {
        format!(
            "\n- [{title}]({comment_url}) by [{author}](https://github.com/{author})",
            title = self.title,
            author = self.author,
            comment_url = self.comment_url
        )
    }
}
impl Ord for NoteDataEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.comment_url.cmp(&other.comment_url)
    }
}
impl PartialOrd for NoteDataEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
struct NoteData {
    entries_by_url: HashMap<String, NoteDataEntry>,
}

impl NoteData {
    pub(crate) fn get_url_from_title_prefix(&self, title: &str) -> Option<String> {
        let tmp = self.entries_by_url.clone();
        tmp.iter().sorted().find_map(|(key, val)| {
            if val.title.starts_with(title) {
                Some(key.to_owned())
            } else {
                None
            }
        })
    }

    pub(crate) fn remove_by_title(&mut self, title: &str) -> Option<NoteDataEntry> {
        if let Some(url_to_remove) = self.get_url_from_title_prefix(title) {
            if let Some(entry) = self.entries_by_url.remove(&url_to_remove) {
                log::debug!("SUCCESSFULLY REMOVED ENTRY: {:#?}", &entry);
                Some(entry)
            } else {
                log::debug!("UNABLE TO REMOVE ENTRY WITH URL: {:?}", &url_to_remove);
                None
            }
        } else {
            log::debug!("UNABLE TO REMOVE ENTRY WITH TITLE: {:?}", title);
            None
        }
    }

    pub(crate) fn to_markdown(&self, bot: &str) -> String {
        if self.entries_by_url.is_empty() {
            return String::new();
        }

        let mut text = String::from("\n### Summary Notes\n");
        for (_, entry) in self.entries_by_url.iter().sorted() {
            text.push_str(&entry.to_markdown());
        }
        let _ = writeln!(text, "\n\n*Managed by `@{bot}`—see [help](https://forge.rust-lang.org/triagebot/note.html) for details*");
        text
    }
}

pub(super) async fn handle_command(
    ctx: &Context,
    _config: &NoteConfig,
    event: &Event,
    cmd: NoteCommand,
) -> anyhow::Result<()> {
    let issue = event.issue().unwrap();
    let e = EditIssueBody::new(&issue, "SUMMARY");

    let mut current: NoteData = e.current_data().unwrap_or_default();

    let comment_url = String::from(event.html_url().unwrap());
    let author = event.user().login.to_owned();

    match &cmd {
        NoteCommand::Summary { title } => {
            let title = title.to_owned();
            if let Some(existing_entry) = current.entries_by_url.get_mut(&comment_url) {
                existing_entry.title = title;
                log::debug!("Updated existing entry: {:#?}", existing_entry);
            } else {
                let new_entry = NoteDataEntry {
                    title,
                    comment_url: comment_url.clone(),
                    author,
                };
                log::debug!("New Note Entry: {:#?}", new_entry);
                current.entries_by_url.insert(comment_url, new_entry);
                log::debug!("Entries by URL: {:#?}", current.entries_by_url);
            }
        }
        NoteCommand::Remove { title } => {
            if let Some(entry) = current.remove_by_title(title) {
                log::debug!("SUCCESSFULLY REMOVED ENTRY: {:#?}", entry);
            } else {
                log::debug!("UNABLE TO REMOVE ENTRY");
            }
        }
    }

    let new_markdown = current.to_markdown(&ctx.username);
    log::debug!("New MD: {:#?}", new_markdown);

    e.apply(&ctx.github, new_markdown, current).await?;

    Ok(())
}
