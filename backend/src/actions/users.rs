use anyhow::Context;
use sqlx::{query::Query, Database, Executor, IntoArguments, MySql, Pool};

use super::User;

pub async fn new_user(pool: &Pool<MySql>, user: User) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT IGNORE INTO users (user_id, user_name, friend_id) VALUES (?, ?, UUID())",
        &user.id,
        &user.name,
    )
    .execute(pool)
    .await
    .context("Failed to new_user")?;

    Ok(())
}
