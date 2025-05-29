use crate::github::User;
use anyhow::Context;
use tokio_postgres::Client as DbClient;

/// Add a new user.
/// If an user already exists, updates their username.
pub async fn record_username(db: &DbClient, user_id: u64, username: &str) -> anyhow::Result<()> {
    db.execute(
        r"
INSERT INTO users (user_id, username) VALUES ($1, $2)
ON CONFLICT (user_id)
DO UPDATE SET username = $2",
        &[&(user_id as i64), &username],
    )
    .await
    .context("inserting user id / username")?;
    Ok(())
}

/// Return a user from the DB.
pub async fn get_user(db: &DbClient, user_id: u64) -> anyhow::Result<Option<User>> {
    let row = db
        .query_opt(
            r"
SELECT username
FROM users
WHERE user_id = $1;",
            &[&(user_id as i64)],
        )
        .await
        .context("cannot load user from DB")?;
    Ok(row.map(|row| {
        let username: &str = row.get(0);
        User {
            id: user_id,
            login: username.to_string(),
        }
    }))
}

#[cfg(test)]
mod tests {
    use crate::db::users::{get_user, record_username};
    use crate::tests::run_db_test;

    #[tokio::test]
    async fn update_username_on_conflict() {
        run_db_test(|ctx| async {
            let db = ctx.db_client();

            record_username(&db, 1, "Foo").await?;
            record_username(&db, 1, "Bar").await?;

            assert_eq!(get_user(&db, 1).await?.unwrap().login, "Bar");

            Ok(ctx)
        })
        .await;
    }
}
