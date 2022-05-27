use anyhow::Context;
use derive_more::Constructor;
use sqlx::{query::Query, Database, Executor, IntoArguments, MySql, Pool};
use uuid::Uuid;

use crate::actions::Friend;

use super::{NewUser, User};

pub async fn add_new_user(pool: &Pool<MySql>, new_user: &NewUser) -> anyhow::Result<()> {
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

pub async fn add_follow_user(
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

pub async fn fetch_friend_list(pool: &Pool<MySql>, user_id: u64) -> anyhow::Result<Vec<User>> {
    let friend_list: Vec<User> = sqlx::query_as!(
        User,
        "
        SELECT google_id, user_id, user_name 
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
    .unwrap();

    Ok(friend_list)
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::{MySql, Pool};
    use uuid::{uuid, Uuid};

    use crate::actions::NewUser;

    use super::{add_new_user, fetch_friend_list};
    #[actix_web::test]
    async fn test_insert_new_user() {
        let pool = create_pool().await;
        for _ in 0..100 {
            let user = NewUser::new(
                rand::random::<u64>().to_string(),
                Some(rand::random::<u64>().to_string()),
            );
            add_new_user(&pool, &user).await.unwrap();
        }
    }

    #[actix_web::test]
    async fn test_select_friend_list() {
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
