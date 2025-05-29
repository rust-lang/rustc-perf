#![allow(clippy::new_without_default)]

use crate::github::PullRequestDetails;

use anyhow::Context;
use handlers::HandlerError;
use interactions::ErrorComment;
use std::fmt;
use tracing as log;

pub mod actions;
pub mod agenda;
mod changelogs;
pub mod config;
pub mod db;
pub mod github;
pub mod handlers;
pub mod interactions;
pub mod jobs;
pub mod notification_listing;
pub mod payload;
pub mod rfcbot;
pub mod team;
mod team_data;
pub mod triage;
mod utils;
pub mod zulip;

#[cfg(test)]
mod tests;

/// The name of a webhook event.
#[derive(Debug)]
pub enum EventName {
    /// Pull request activity.
    ///
    /// This gets translated to [`github::Event::Issue`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request>
    PullRequest,
    /// Pull request review activity.
    ///
    /// This gets translated to [`github::Event::IssueComment`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request_review>
    PullRequestReview,
    /// A comment on a pull request review.
    ///
    /// This gets translated to [`github::Event::IssueComment`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#pull_request_review_comment>
    PullRequestReviewComment,
    /// An issue or PR comment.
    ///
    /// This gets translated to [`github::Event::IssueComment`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issue_comment>
    IssueComment,
    /// Issue activity.
    ///
    /// This gets translated to [`github::Event::Issue`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#issues>
    Issue,
    /// One or more commits are pushed to a repository branch or tag.
    ///
    /// This gets translated to [`github::Event::Push`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#push>
    Push,
    /// A Git branch or tag is created.
    ///
    /// This gets translated to [`github::Event::Create`] when sent to a handler.
    ///
    /// <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads#create>
    Create,
    /// All other unhandled webhooks.
    Other,
}

impl std::str::FromStr for EventName {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<EventName, Self::Err> {
        Ok(match s {
            "pull_request_review" => EventName::PullRequestReview,
            "pull_request_review_comment" => EventName::PullRequestReviewComment,
            "issue_comment" => EventName::IssueComment,
            "pull_request" => EventName::PullRequest,
            "issues" => EventName::Issue,
            "push" => EventName::Push,
            "create" => EventName::Create,
            _ => EventName::Other,
        })
    }
}

impl fmt::Display for EventName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EventName::PullRequestReview => "pull_request_review",
                EventName::PullRequestReviewComment => "pull_request_review_comment",
                EventName::IssueComment => "issue_comment",
                EventName::Issue => "issues",
                EventName::PullRequest => "pull_request",
                EventName::Push => "push",
                EventName::Create => "create",
                EventName::Other => "other",
            }
        )
    }
}

#[derive(Debug)]
pub struct WebhookError(
    #[allow(dead_code)] // Used in debug display
    anyhow::Error,
);

impl From<anyhow::Error> for WebhookError {
    fn from(e: anyhow::Error) -> WebhookError {
        WebhookError(e)
    }
}

pub fn deserialize_payload<T: serde::de::DeserializeOwned>(v: &str) -> anyhow::Result<T> {
    let mut deserializer = serde_json::Deserializer::from_str(&v);
    let res: Result<T, _> = serde_path_to_error::deserialize(&mut deserializer);
    match res {
        Ok(r) => Ok(r),
        Err(e) => {
            log::error!("failed to deserialize webhook payload: {v}");
            let ctx = format!("at {:?}", e.path());
            Err(e.into_inner()).context(ctx)
        }
    }
}

pub async fn webhook(
    event: EventName,
    payload: String,
    ctx: &handlers::Context,
) -> Result<bool, WebhookError> {
    let event = match event {
        EventName::PullRequestReview => {
            let mut payload = deserialize_payload::<github::PullRequestReviewEvent>(&payload)
                .context("PullRequestReview failed to deserialize")
                .map_err(anyhow::Error::from)?;

            log::info!("handling pull request review comment {:?}", payload);
            payload.pull_request.pull_request = Some(PullRequestDetails::new());

            // Treat pull request review comments exactly like pull request
            // comments.
            github::Event::IssueComment(github::IssueCommentEvent {
                action: match payload.action {
                    github::PullRequestReviewAction::Submitted => {
                        github::IssueCommentAction::Created
                    }
                    github::PullRequestReviewAction::Edited => github::IssueCommentAction::Edited,
                    github::PullRequestReviewAction::Dismissed => {
                        github::IssueCommentAction::Deleted
                    }
                },
                changes: payload.changes,
                issue: payload.pull_request,
                comment: payload.review,
                repository: payload.repository,
            })
        }
        EventName::PullRequestReviewComment => {
            let mut payload = deserialize_payload::<github::PullRequestReviewComment>(&payload)
                .context("PullRequestReview(Comment) failed to deserialize")
                .map_err(anyhow::Error::from)?;

            payload.issue.pull_request = Some(PullRequestDetails::new());

            log::info!("handling pull request review comment {:?}", payload);

            // Treat pull request review comments exactly like pull request
            // review comments.
            github::Event::IssueComment(github::IssueCommentEvent {
                action: payload.action,
                changes: payload.changes,
                issue: payload.issue,
                comment: payload.comment,
                repository: payload.repository,
            })
        }
        EventName::IssueComment => {
            let payload = deserialize_payload::<github::IssueCommentEvent>(&payload)
                .context("IssueCommentEvent failed to deserialize")
                .map_err(anyhow::Error::from)?;

            log::info!("handling issue comment {:?}", payload);

            github::Event::IssueComment(payload)
        }
        EventName::Issue | EventName::PullRequest => {
            let mut payload = deserialize_payload::<github::IssuesEvent>(&payload)
                .context(format!("{:?} failed to deserialize", event))
                .map_err(anyhow::Error::from)?;

            if matches!(event, EventName::PullRequest) {
                payload.issue.pull_request = Some(PullRequestDetails::new());
            }

            log::info!("handling issue event {:?}", payload);

            github::Event::Issue(payload)
        }
        EventName::Push => {
            let payload = deserialize_payload::<github::PushEvent>(&payload)
                .with_context(|| format!("{:?} failed to deserialize", event))
                .map_err(anyhow::Error::from)?;

            log::info!("handling push event {:?}", payload);

            github::Event::Push(payload)
        }
        EventName::Create => {
            let payload = deserialize_payload::<github::CreateEvent>(&payload)
                .with_context(|| format!("{:?} failed to deserialize", event))
                .map_err(anyhow::Error::from)?;

            log::info!("handling create event {:?}", payload);

            github::Event::Create(payload)
        }
        // Other events need not be handled
        EventName::Other => {
            return Ok(false);
        }
    };
    let errors = handlers::handle(&ctx, &event).await;
    let mut other_error = false;
    let mut message = String::new();
    for err in errors {
        match err {
            HandlerError::Message(msg) => {
                if !message.is_empty() {
                    message.push_str("\n\n");
                }
                message.push_str(&msg);
            }
            HandlerError::Other(err) => {
                log::error!("handling event failed: {:?}", err);
                other_error = true;
            }
        }
    }
    if !message.is_empty() {
        if let Some(issue) = event.issue() {
            let cmnt = ErrorComment::new(issue, message);
            cmnt.post(&ctx.github).await?;
        }
    }
    if other_error {
        Err(WebhookError(anyhow::anyhow!(
            "handling failed, error logged",
        )))
    } else {
        Ok(true)
    }
}
