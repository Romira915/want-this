use anyhow::Context;
use derive_more::Constructor;
use sqlx::{query::Query, Database, Executor, IntoArguments, MySql, Pool};
use uuid::Uuid;

use crate::actions::Friend;

use super::{NewUser, User};

pub(crate) async fn add_new_user(pool: &Pool<MySql>, new_user: &NewUser) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT IGNORE INTO users (google_id, user_name) VALUES (?, ?)",
        new_user.google_id,
        new_user.name,
    )
    .execute(pool)
    .await
    .context("Failed to new_user")?;

    Ok(())
}

pub(crate) async fn add_follow_user(
    pool: &Pool<MySql>,
    src_uid: u64,
    dist_uid: u64,
) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO friends_relationship (source, destination) VALUES (?, ?)",
        src_uid,
        dist_uid
    )
    .execute(pool)
    .await
    .context("Failed to follow_user")?;

    Ok(())
}

pub(crate) async fn fetch_friend_list(
    pool: &Pool<MySql>,
    user_id: u64,
) -> anyhow::Result<Vec<User>> {
    let friend_list: Vec<User> = sqlx::query_as!(
        User,
        "
        SELECT user_id, google_id, user_name 
        FROM users INNER JOIN 
        (SELECT follow AS user_id FROM 
        (SELECT source AS follower FROM friends_relationship WHERE destination = ?) AS follower
        INNER JOIN (SELECT destination AS follow FROM friends_relationship WHERE source = ?) AS follow 
        ON follower.follower = follow.follow) 
        AS friend_list USING(user_id);
        ",
        &user_id,
        &user_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch_friend_list")?;

    Ok(friend_list)
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::{MySql, Pool};
    use uuid::{uuid, Uuid};

    use crate::actions::NewUser;

    use super::{add_new_user, fetch_friend_list};
    #[tokio::test]
    async fn test_add_new_user() {
        let pool = create_pool().await;
        let google_id = rand::random::<u64>().to_string();
        let new_user = NewUser::new(google_id.clone(), Some(rand::random::<u64>().to_string()));
        add_new_user(&pool, &new_user)
            .await
            .expect("Failed to add_new_user");
        let user = sqlx::query!(
            "SELECT google_id FROM users WHERE google_id = ?",
            &google_id
        )
        .fetch_one(&pool)
        .await
        .expect(&format!("Not Found User google_id {}", &google_id));

        assert_eq!(google_id, user.google_id);
    }

    #[tokio::test]
    async fn test_add_follow_user() -> anyhow::Result<()> {
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_friend_list() {
        let pool = create_pool().await;
        let friend_list = fetch_friend_list(&pool, 99799836211019858).await.unwrap();
        println!("{:#?}", friend_list);
    }

    async fn create_pool() -> Pool<MySql> {
        dotenv::dotenv().unwrap();
        sqlx::mysql::MySqlPoolOptions::new()
            .connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap()
    }
}
