use anyhow::{Context, Ok};
use sqlx::{query::Query, Database, Executor, IntoArguments, MySql, Pool};
use uuid::Uuid;

use super::{Friend, User};

pub async fn new_user(pool: &Pool<MySql>, user: &User) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT IGNORE INTO users (user_id, user_name, friend_id) VALUES (?, ?, UUID())",
        user.id,
        user.name,
    )
    .execute(pool)
    .await
    .context("Failed to new_user")?;

    Ok(())
}

pub async fn follow_user(
    pool: &Pool<MySql>,
    src_fid: &Uuid,
    dist_fid: &Uuid,
) -> anyhow::Result<()> {
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

pub async fn friend_list(pool: &Pool<MySql>, friend_id: &Uuid) -> anyhow::Result<()> {
    // let follower = sqlx::query!(
    //     "SELECT source AS friend_id FROM friends_relationship WHERE destination='?'",
    //     friend_id.to_string()
    // )
    // .fetch_all(pool)
    // .await?;

    let friend_id = friend_id.to_string();

    // let friend_list: Vec<Friend> = sqlx::query_as!(
    //     Friend,
    //     "
    //     SELECT follow AS friend_id FROM
    //     (SELECT source AS follower FROM friends_relationship WHERE destination='?') AS follower
    //     INNER JOIN
    //     (SELECT destination AS follow FROM friends_relationship WHERE source='?') AS follow
    //     ON follower.follower = follow.follow
    //     ",
    //     &friend_id,
    //     &friend_id,
    // )
    // .fetch_all(pool)
    // .await
    // .context("Failed to friend_list")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::actions::User;

    use super::new_user;
    #[actix_web::test]
    async fn test_new_user() {
        dotenv::dotenv().unwrap();
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .connect(&env::var("TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();

        for _ in 0..100 {
            let user = User::new(
                rand::random::<u64>().to_string(),
                rand::random::<u64>().to_string(),
            );
            new_user(&pool, &user).await.unwrap();
        }
    }
}
