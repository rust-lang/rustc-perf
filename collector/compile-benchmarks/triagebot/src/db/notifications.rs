use anyhow::Context as _;
use chrono::{DateTime, FixedOffset};
use tokio_postgres::Client as DbClient;
use tracing as log;

pub struct Notification {
    pub user_id: u64,
    pub origin_url: String,
    pub origin_html: String,
    pub short_description: Option<String>,
    pub time: DateTime<FixedOffset>,

    /// If this is Some, then the notification originated in a team-wide ping
    /// (e.g., @rust-lang/libs). The String is the team name (e.g., libs).
    pub team_name: Option<String>,
}

pub async fn record_ping(db: &DbClient, notification: &Notification) -> anyhow::Result<()> {
    db.execute("INSERT INTO notifications (user_id, origin_url, origin_html, time, short_description, team_name, idx)
        VALUES (
            $1, $2, $3, $4, $5, $6,
            (SELECT max(notifications.idx) + 1 from notifications where notifications.user_id = $1)
        )",
        &[&(notification.user_id as i64), &notification.origin_url, &notification.origin_html, &notification.time, &notification.short_description, &notification.team_name],
        ).await.context("inserting notification")?;

    Ok(())
}

#[derive(Copy, Clone)]
pub enum Identifier<'a> {
    Url(&'a str),
    Index(std::num::NonZeroU32),
    /// Glob identifier (`all` or `*`).
    All,
}

pub async fn delete_ping(
    db: &mut DbClient,
    user_id: u64,
    identifier: Identifier<'_>,
) -> anyhow::Result<Vec<NotificationData>> {
    match identifier {
        Identifier::Url(origin_url) => {
            let rows = db
                .query(
                    "DELETE FROM notifications WHERE user_id = $1 and origin_url = $2
                    RETURNING origin_html, time, short_description, metadata",
                    &[&(user_id as i64), &origin_url],
                )
                .await
                .context("delete notification query")?;
            Ok(rows
                .into_iter()
                .map(|row| {
                    let origin_text: String = row.get(0);
                    let time: DateTime<FixedOffset> = row.get(1);
                    let short_description: Option<String> = row.get(2);
                    let metadata: Option<String> = row.get(3);
                    NotificationData {
                        origin_url: origin_url.to_owned(),
                        origin_text,
                        time,
                        short_description,
                        metadata,
                    }
                })
                .collect())
        }
        Identifier::Index(idx) => loop {
            let t = db
                .build_transaction()
                .isolation_level(tokio_postgres::IsolationLevel::Serializable)
                .start()
                .await
                .context("begin transaction")?;

            let notifications = t
                .query(
                    "select notification_id, idx, user_id
                    from notifications
                    where user_id = $1
                    order by idx asc nulls last;",
                    &[&(user_id as i64)],
                )
                .await
                .context("failed to get ordering")?;

            let notification_id: i64 = notifications
                .get((idx.get() - 1) as usize)
                .ok_or_else(|| anyhow::anyhow!("No such notification with index {}", idx.get()))?
                .get(0);

            let row = t
                .query_one(
                    "DELETE FROM notifications WHERE notification_id = $1
                RETURNING origin_url, origin_html, time, short_description, metadata",
                    &[&notification_id],
                )
                .await
                .context(format!(
                    "Failed to delete notification with id {}",
                    notification_id
                ))?;

            let origin_url: String = row.get(0);
            let origin_text: String = row.get(1);
            let time: DateTime<FixedOffset> = row.get(2);
            let short_description: Option<String> = row.get(3);
            let metadata: Option<String> = row.get(4);
            let deleted_notification = NotificationData {
                origin_url,
                origin_text,
                time,
                short_description,
                metadata,
            };

            if let Err(e) = t.commit().await {
                if e.code().map_or(false, |c| {
                    *c == tokio_postgres::error::SqlState::T_R_SERIALIZATION_FAILURE
                }) {
                    log::trace!("serialization failure, restarting deletion");
                    continue;
                } else {
                    return Err(e).context("transaction commit failure");
                }
            } else {
                return Ok(vec![deleted_notification]);
            }
        },
        Identifier::All => {
            let rows = db
                .query(
                    "DELETE FROM notifications WHERE user_id = $1
                        RETURNING origin_url, origin_html, time, short_description, metadata",
                    &[&(user_id as i64)],
                )
                .await
                .context("delete all notifications query")?;
            Ok(rows
                .into_iter()
                .map(|row| {
                    let origin_url: String = row.get(0);
                    let origin_text: String = row.get(1);
                    let time: DateTime<FixedOffset> = row.get(2);
                    let short_description: Option<String> = row.get(3);
                    let metadata: Option<String> = row.get(4);
                    NotificationData {
                        origin_url,
                        origin_text,
                        time,
                        short_description,
                        metadata,
                    }
                })
                .collect())
        }
    }
}

#[derive(Debug)]
pub struct NotificationData {
    pub origin_url: String,
    pub origin_text: String,
    pub short_description: Option<String>,
    pub time: DateTime<FixedOffset>,
    pub metadata: Option<String>,
}

pub async fn move_indices(
    db: &mut DbClient,
    user_id: u64,
    from: u32,
    to: u32,
) -> anyhow::Result<()> {
    let from = usize::try_from(from)?;
    let to = usize::try_from(to)?;
    loop {
        let t = db
            .build_transaction()
            .isolation_level(tokio_postgres::IsolationLevel::Serializable)
            .start()
            .await
            .context("begin transaction")?;

        let notifications = t
            .query(
                "select notification_id, idx, user_id
        from notifications
        where user_id = $1
        order by idx asc nulls last;",
                &[&(user_id as i64)],
            )
            .await
            .context("failed to get initial ordering")?;

        let mut notifications = notifications
            .into_iter()
            .map(|n| n.get(0))
            .collect::<Vec<i64>>();

        if notifications.get(from).is_none() {
            anyhow::bail!(
                "`from` index not present, must be less than {}",
                notifications.len()
            );
        }

        if notifications.get(to).is_none() {
            anyhow::bail!(
                "`to` index not present, must be less than {}",
                notifications.len()
            );
        }

        if from < to {
            notifications[from..=to].rotate_left(1);
        } else if to < from {
            notifications[to..=from].rotate_right(1);
        }

        for (idx, id) in notifications.into_iter().enumerate() {
            t.execute(
                "update notifications SET idx = $2
                 where notification_id = $1",
                &[&id, &(idx as i32)],
            )
            .await
            .context("update notification id")?;
        }

        if let Err(e) = t.commit().await {
            if e.code().map_or(false, |c| {
                *c == tokio_postgres::error::SqlState::T_R_SERIALIZATION_FAILURE
            }) {
                log::trace!("serialization failure, restarting index movement");
                continue;
            } else {
                return Err(e).context("transaction commit failure");
            }
        } else {
            break;
        }
    }

    Ok(())
}

pub async fn add_metadata(
    db: &mut DbClient,
    user_id: u64,
    idx: u32,
    metadata: Option<&str>,
) -> anyhow::Result<()> {
    let idx = usize::try_from(idx)?;
    loop {
        let t = db
            .build_transaction()
            .isolation_level(tokio_postgres::IsolationLevel::Serializable)
            .start()
            .await
            .context("begin transaction")?;

        let notifications = t
            .query(
                "select notification_id, idx, user_id
        from notifications
        where user_id = $1
        order by idx asc nulls last;",
                &[&(user_id as i64)],
            )
            .await
            .context("failed to get initial ordering")?;

        let notifications = notifications
            .into_iter()
            .map(|n| n.get(0))
            .collect::<Vec<i64>>();

        match notifications.get(idx) {
            None => anyhow::bail!(
                "index not present, must be less than {}",
                notifications.len()
            ),
            Some(id) => {
                t.execute(
                    "update notifications SET metadata = $2
                 where notification_id = $1",
                    &[&id, &metadata],
                )
                .await
                .context("update notification id")?;
            }
        }

        if let Err(e) = t.commit().await {
            if e.code().map_or(false, |c| {
                *c == tokio_postgres::error::SqlState::T_R_SERIALIZATION_FAILURE
            }) {
                log::trace!("serialization failure, restarting index movement");
                continue;
            } else {
                return Err(e).context("transaction commit failure");
            }
        } else {
            break;
        }
    }

    Ok(())
}

pub async fn get_notifications(
    db: &DbClient,
    username: &str,
) -> anyhow::Result<Vec<NotificationData>> {
    let notifications = db
        .query(
            "
        select username, origin_url, origin_html, time, short_description, idx, metadata
        from notifications
        join users on notifications.user_id = users.user_id
        where username = $1
        order by notifications.idx asc nulls last;",
            &[&username],
        )
        .await
        .context("Getting notification data")?;

    let mut data = Vec::new();
    for notification in notifications {
        let origin_url: String = notification.get(1);
        let origin_text: String = notification.get(2);
        let time: DateTime<FixedOffset> = notification.get(3);
        let short_description: Option<String> = notification.get(4);
        let metadata: Option<String> = notification.get(6);

        data.push(NotificationData {
            origin_url,
            origin_text,
            short_description,
            time,
            metadata,
        });
    }

    Ok(data)
}
