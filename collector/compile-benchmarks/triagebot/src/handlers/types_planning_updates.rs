use crate::db::schedule_job;
use crate::github;
use crate::jobs::Job;
use anyhow::Context as _;
use async_trait::async_trait;
use chrono::{Datelike, Duration, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

const TYPES_REPO: &'static str = "rust-lang/types-team";
// T-types/meetings
const TYPES_MEETINGS_STREAM: u64 = 326132;

pub struct TypesPlanningMeetingThreadOpenJob;

#[async_trait]
impl Job for TypesPlanningMeetingThreadOpenJob {
    fn name(&self) -> &'static str {
        "types_planning_meeting_thread_open"
    }

    async fn run(&self, ctx: &super::Context, _metadata: &serde_json::Value) -> anyhow::Result<()> {
        // On the last week of the month, we open a thread on zulip for the next Monday
        let today = chrono::Utc::now().date_naive();
        let first_monday = today + chrono::Duration::days(7);
        // We actually schedule for every Monday, so first check if this is the last Monday of the month
        if first_monday.month() == today.month() {
            return Ok(());
        }
        let meeting_date_string = first_monday.format("%Y-%m-%d").to_string();
        let message = format!("\
            Hello @*T-types/meetings*. Monthly planning meeting in one week.\n\
            This is a reminder to update the current [roadmap tracking issues](https://github.com/rust-lang/types-team/issues?q=is%3Aissue+is%3Aopen+label%3Aroadmap-tracking-issue).\n\
            Extra reminders will be sent later this week.");
        let zulip_req = crate::zulip::MessageApiRequest {
            recipient: crate::zulip::Recipient::Stream {
                id: TYPES_MEETINGS_STREAM,
                topic: &format!("{meeting_date_string} planning meeting"),
            },
            content: &message,
        };
        zulip_req.send(&ctx.github.raw()).await?;

        // Then, we want to schedule the next Thursday after this
        let mut thursday = today;
        while thursday.weekday().num_days_from_monday() != 3 {
            thursday = thursday.succ_opt().unwrap();
        }
        let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let thursday_at_noon = Utc.from_utc_datetime(&thursday.and_time(noon));
        let metadata = serde_json::value::to_value(PlanningMeetingUpdatesPingMetadata {
            date_string: meeting_date_string,
        })
        .unwrap();
        schedule_job(
            &*ctx.db.get().await,
            TypesPlanningMeetingUpdatesPing.name(),
            metadata,
            thursday_at_noon,
        )
        .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlanningMeetingUpdatesPingMetadata {
    pub date_string: String,
}

pub struct TypesPlanningMeetingUpdatesPing;

#[async_trait]
impl Job for TypesPlanningMeetingUpdatesPing {
    fn name(&self) -> &'static str {
        "types_planning_meeting_updates_ping"
    }

    async fn run(&self, ctx: &super::Context, metadata: &serde_json::Value) -> anyhow::Result<()> {
        let metadata = serde_json::from_value(metadata.clone())?;
        // On the thursday before the first monday, we want to ping for updates
        request_updates(ctx, metadata).await?;
        Ok(())
    }
}

pub async fn request_updates(
    ctx: &super::Context,
    metadata: PlanningMeetingUpdatesPingMetadata,
) -> anyhow::Result<()> {
    let gh = &ctx.github;
    let types_repo = gh.repository(TYPES_REPO).await?;

    let tracking_issues_query = github::Query {
        filters: vec![("state", "open"), ("is", "issue")],
        include_labels: vec!["roadmap-tracking-issue"],
        exclude_labels: vec![],
    };
    let issues = types_repo
        .get_issues(&gh, &tracking_issues_query)
        .await
        .with_context(|| "Unable to get issues.")?;

    let mut issues_needs_updates = vec![];
    for issue in issues {
        // If the issue has been updated in the past 7 days, we consider this "updated". We *could* be more clever, but
        // this is fine under the assumption that tracking issues should only contain updates.
        let older_than_7_days = issue.updated_at < (Utc::now() - Duration::days(7));
        if !older_than_7_days {
            continue;
        }
        // In the future, we should reach out to specific people in charge of specific issues. For now, because our tracking
        // method is crude and will over-estimate the issues that need updates.
        /*
        let mut dmed_assignee = false;
        for assignee in issue.assignees {
            let zulip_id_and_email = zulip_id_and_email(ctx, assignee.id.unwrap()).await?;
            let (zulip_id, email) = match zulip_id_and_email {
                Some(id) => id,
                None => continue,
            };
            let message = format!(
                "Type team tracking issue needs an update. [Issue #{}]({})",
                issue.number, issue.html_url
            );
            let zulip_req = crate::zulip::MessageApiRequest {
                recipient: crate::zulip::Recipient::Private {
                    id: zulip_id,
                    email: &email,
                },
                content: &message,
            };
            zulip_req.send(&ctx.github.raw()).await?;
            dmed_assignee = true;
        }
        if !dmed_assignee {
            let message = format!(
                "Type team tracking issue needs an update, and was unable to reach an assignee. \
                [Issue #{}]({})",
                issue.number, issue.html_url
            );
            let zulip_req = crate::zulip::MessageApiRequest {
                recipient: crate::zulip::Recipient::Stream {
                    id: 144729,
                    topic: "tracking issue updates",
                },
                content: &message,
            };
            zulip_req.send(&ctx.github.raw()).await?;
        }
        */
        issues_needs_updates.push(format!("- [Issue #{}]({})", issue.number, issue.html_url));
    }

    let issue_list = issues_needs_updates.join("\n");

    let message = format!("The following issues still need updates:\n\n{issue_list}");

    let meeting_date_string = metadata.date_string;
    let zulip_req = crate::zulip::MessageApiRequest {
        recipient: crate::zulip::Recipient::Stream {
            id: TYPES_MEETINGS_STREAM,
            topic: &format!("{meeting_date_string} planning meeting"),
        },
        content: &message,
    };
    zulip_req.send(&ctx.github.raw()).await?;

    Ok(())
}
