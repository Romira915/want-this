use anyhow::{Context, Ok};
use sqlx::{query::Query, Database, Executor, IntoArguments, MySql, Pool};
use uuid::Uuid;

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

pub async fn follow_user(pool: &Pool<MySql>, src_fid: Uuid, dist_fid: Uuid) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO friends_relationship (source, destination) VALUES (?, ?)",
        src_fid.to_string(),
        dist_fid.to_string()
    )
    .execute(pool)
    .await
    .context("Failed to follow_user")?;

    Ok(())
}
