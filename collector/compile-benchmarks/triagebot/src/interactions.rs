use crate::github::{GithubClient, Issue};
use std::fmt::Write;

pub struct ErrorComment<'a> {
    issue: &'a Issue,
    message: String,
}

impl<'a> ErrorComment<'a> {
    pub fn new<T>(issue: &'a Issue, message: T) -> ErrorComment<'a>
    where
        T: Into<String>,
    {
        ErrorComment {
            issue,
            message: message.into(),
        }
    }

    pub async fn post(&self, client: &GithubClient) -> anyhow::Result<()> {
        let mut body = String::new();
        writeln!(body, "**Error**: {}", self.message)?;
        writeln!(body)?;
        writeln!(
            body,
            "Please file an issue on GitHub at [triagebot](https://github.com/rust-lang/triagebot) if there's \
            a problem with this bot, or reach out on [#t-infra](https://rust-lang.zulipchat.com/#narrow/stream/242791-t-infra) on Zulip."
        )?;
        self.issue.post_comment(client, &body).await?;
        Ok(())
    }
}

pub struct PingComment<'a> {
    issue: &'a Issue,
    users: &'a [&'a str],
}

impl<'a> PingComment<'a> {
    pub fn new(issue: &'a Issue, users: &'a [&str]) -> PingComment<'a> {
        PingComment { issue, users }
    }

    pub async fn post(&self, client: &GithubClient) -> anyhow::Result<()> {
        let mut body = String::new();
        for user in self.users {
            write!(body, "@{} ", user)?;
        }
        self.issue.post_comment(client, &body).await?;
        Ok(())
    }
}

pub struct EditIssueBody<'a> {
    issue: &'a Issue,
    id: &'static str,
}

static START_BOT: &str = "<!-- TRIAGEBOT_START -->\n\n";
static END_BOT: &str = "<!-- TRIAGEBOT_END -->";

fn normalize_body(body: &str) -> String {
    str::replace(body, "\r\n", "\n")
}

impl<'a> EditIssueBody<'a> {
    pub fn new(issue: &'a Issue, id: &'static str) -> EditIssueBody<'a> {
        EditIssueBody { issue, id }
    }

    fn get_current(&self) -> Option<String> {
        let self_issue_body = normalize_body(&self.issue.body);
        let start_section = self.start_section();
        let end_section = self.end_section();
        if self_issue_body.contains(START_BOT) {
            if self_issue_body.contains(&start_section) {
                let start_idx = self_issue_body.find(&start_section).unwrap();
                let end_idx = self_issue_body.find(&end_section).unwrap();
                let current =
                    String::from(&self_issue_body[start_idx..(end_idx + end_section.len())]);
                Some(current)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current_data<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
        let all = self.get_current()?;
        let start = self.data_section_start();
        let end = self.data_section_end();
        let start_idx = all.find(&start).unwrap();
        let end_idx = all.find(&end).unwrap();
        let text = &all[(start_idx + start.len())..end_idx];
        Some(serde_json::from_str(text).unwrap_or_else(|e| {
            panic!("deserializing data {:?} failed: {:?}", text, e);
        }))
    }

    fn start_section(&self) -> String {
        format!("<!-- TRIAGEBOT_{}_START -->\n", self.id)
    }

    fn end_section(&self) -> String {
        format!("\n<!-- TRIAGEBOT_{}_END -->\n", self.id)
    }

    fn data_section_start(&self) -> String {
        format!("\n<!-- TRIAGEBOT_{}_DATA_START$$", self.id)
    }

    fn data_section_end(&self) -> String {
        format!("$$TRIAGEBOT_{}_DATA_END -->\n", self.id)
    }

    fn data_section<T>(&self, data: T) -> String
    where
        T: serde::Serialize,
    {
        format!(
            "{}{}{}",
            self.data_section_start(),
            serde_json::to_string(&data).unwrap(),
            self.data_section_end()
        )
    }

    pub async fn apply<T>(&self, client: &GithubClient, text: String, data: T) -> anyhow::Result<()>
    where
        T: serde::Serialize,
    {
        let mut current_body = normalize_body(&self.issue.body.clone());
        let start_section = self.start_section();
        let end_section = self.end_section();

        let bot_section = format!(
            "{}{}{}{}",
            start_section,
            text,
            self.data_section(data),
            end_section
        );
        let empty_bot_section = format!("{}{}", start_section, end_section);

        let all_new = format!("\n\n{}{}{}", START_BOT, bot_section, END_BOT);
        if current_body.contains(START_BOT) {
            if current_body.contains(&start_section) {
                let start_idx = current_body.find(&start_section).unwrap();
                let end_idx = current_body.find(&end_section).unwrap();
                current_body.replace_range(start_idx..(end_idx + end_section.len()), &bot_section);
                if current_body.contains(&all_new) && bot_section == empty_bot_section {
                    let start_idx = current_body.find(&all_new).unwrap();
                    let end_idx = start_idx + all_new.len();
                    current_body.replace_range(start_idx..end_idx, "");
                }
                self.issue.edit_body(&client, &current_body).await?;
            } else {
                let end_idx = current_body.find(&END_BOT).unwrap();
                current_body.insert_str(end_idx, &bot_section);
                self.issue.edit_body(&client, &current_body).await?;
            }
        } else {
            let new_body = format!("{}{}", current_body, all_new);

            self.issue.edit_body(&client, &new_body).await?;
        }
        Ok(())
    }
}
