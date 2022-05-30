use anyhow::Context;
use sqlx::MySqlConnection;

use crate::domain::entity::user::{NewUser, UpdateUser, User};

pub(crate) struct InternalUserRepository;

impl InternalUserRepository {
    pub(crate) async fn add_new_user(
        conn: &mut MySqlConnection,
        new_user: &NewUser,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "INSERT INTO users (google_id, user_name) VALUES (?, ?);",
            new_user.google_id,
            new_user.name,
        )
        .execute(conn)
        .await
        .context("Failed to new_user")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn add_new_user_return_it(
        conn: &mut MySqlConnection,
        new_user: &NewUser,
    ) -> anyhow::Result<User> {
        Self::add_new_user(conn, new_user).await?;

        Ok(Self::find_user_by_google_id(conn, &new_user.google_id)
            .await?
            .context("Failed to add_new_user_return_it")?)
    }

    pub(crate) async fn update_user_name(
        conn: &mut MySqlConnection,
        update_user: UpdateUser,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "UPDATE users SET user_name = ? WHERE user_id = ?",
            update_user.name,
            update_user.user_id
        )
        .execute(conn)
        .await?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn find_user_by_google_id(
        conn: &mut MySqlConnection,
        google_id: &str,
    ) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, google_id, user_name FROM users WHERE google_id = ?;",
            google_id
        )
        .fetch_optional(conn)
        .await
        .context("Failed to find_user_by_google_id")?;

        Ok(user)
    }

    pub(crate) async fn find_user_by_user_id(
        conn: &mut MySqlConnection,
        user_id: u64,
    ) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, google_id, user_name FROM users WHERE user_id = ?;",
            user_id
        )
        .fetch_optional(conn)
        .await
        .context("Failed to find_user_by_google_id")?;

        Ok(user)
    }

    pub(crate) async fn add_follow_user(
        conn: &mut MySqlConnection,
        src_uid: u64,
        dist_uid: u64,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO friends_relationship (source, destination) VALUES (?, ?);",
            src_uid,
            dist_uid
        )
        .execute(conn)
        .await
        .context("Failed to follow_user")?;

        Ok(())
    }

    pub(crate) async fn fetch_follow_list(
        conn: &mut MySqlConnection,
        source_user_id: u64,
    ) -> anyhow::Result<Vec<User>> {
        let follow_list: Vec<User> = sqlx::query_as!(
            User,
            "SELECT user_id, google_id, user_name 
            FROM users INNER JOIN 
            (SELECT destination FROM friends_relationship WHERE source = ?) AS follow 
            ON users.user_id = follow.destination;",
            source_user_id
        )
        .fetch_all(conn)
        .await
        .context("Failed to fetch_follow_list")?;

        Ok(follow_list)
    }

    pub(crate) async fn fetch_follower_list(
        conn: &mut MySqlConnection,
        destination_user_id: u64,
    ) -> anyhow::Result<Vec<User>> {
        let follow_list: Vec<User> = sqlx::query_as!(
            User,
            "SELECT user_id, google_id, user_name 
             FROM users INNER JOIN 
             (SELECT source FROM friends_relationship WHERE destination = ?) AS follow 
             ON users.user_id = follow.source;",
            destination_user_id
        )
        .fetch_all(conn)
        .await
        .context("Failed to fetch_follower_list")?;

        Ok(follow_list)
    }

    pub(crate) async fn fetch_friend_list(
        conn: &mut MySqlConnection,
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
        user_id,
        user_id
    )
    .fetch_all(conn)
    .await
    .unwrap();

        Ok(friend_list)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::{MySql, Pool, Transaction};

    use crate::domain::entity::user::{NewUser, UpdateUser};

    use super::InternalUserRepository;

    #[tokio::test]
    async fn test_user_repository() -> anyhow::Result<()> {
        let pool = create_pool().await;
        let mut tx: Transaction<MySql> = pool.begin().await?;

        let new_user_list = vec![
            NewUser::new("1".to_string(), Some("高海千歌".to_string())),
            NewUser::new("2".to_string(), Some("桜内梨子".to_string())),
            NewUser::new("3".to_string(), Some("松浦果南".to_string())),
            NewUser::new("4".to_string(), Some("黒澤ダイヤ".to_string())),
            NewUser::new("5".to_string(), Some("渡辺曜".to_string())),
            NewUser::new("6".to_string(), Some("津島善子".to_string())),
            NewUser::new("7".to_string(), Some("国木田花丸".to_string())),
            NewUser::new("8".to_string(), Some("小原鞠莉".to_string())),
            NewUser::new("9".to_string(), Some("黒澤ルビィ".to_string())),
        ];

        tx.rollback().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_add_new_user_and_find_user_by_google_id() -> anyhow::Result<()> {
        let pool = create_pool().await;
        let mut tx = pool.begin().await?;

        let google_id = uuid::Uuid::new_v4().as_u128().to_string();
        let new_user = NewUser::new(google_id.clone(), Some("高海千歌".to_string()));
        assert_eq!(
            None,
            InternalUserRepository::find_user_by_google_id(&mut tx, &google_id).await?
        );
        InternalUserRepository::add_new_user(&mut tx, &new_user).await?;
        assert!(
            InternalUserRepository::find_user_by_google_id(&mut tx, &google_id)
                .await?
                .is_some()
        );

        tx.rollback().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_update_user() -> anyhow::Result<()> {
        let pool = create_pool().await;
        let mut tx = pool.begin().await?;

        let google_id = uuid::Uuid::new_v4().as_u128().to_string();
        let init_name = "高海千歌".to_string();
        let new_user = NewUser::new(google_id.clone(), Some(init_name.clone()));

        let mut user = InternalUserRepository::add_new_user_return_it(&mut tx, &new_user).await?;
        assert_eq!(init_name, user.user_name.unwrap());

        let new_name = "渡辺曜".to_string();
        let update_user = UpdateUser::new(user.user_id, new_name.clone());
        InternalUserRepository::update_user_name(&mut tx, update_user).await?;
        let updated_user = InternalUserRepository::find_user_by_user_id(&mut tx, user.user_id)
            .await?
            .unwrap();
        assert_eq!(new_name, updated_user.user_name.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_friend() -> anyhow::Result<()> {
        let pool = create_pool().await;
        let mut tx = pool.begin().await?;

        let source_google_id = uuid::Uuid::new_v4().as_u128().to_string();
        let source_user = NewUser::new(source_google_id.clone(), Some("高海千歌".to_string()));
        let destination_google_id = uuid::Uuid::new_v4().as_u128().to_string();
        let destination_user =
            NewUser::new(destination_google_id.clone(), Some("渡辺曜".to_string()));

        let source_user =
            InternalUserRepository::add_new_user_return_it(&mut tx, &source_user).await?;
        let destinatino_user =
            InternalUserRepository::add_new_user_return_it(&mut tx, &destination_user).await?;

        InternalUserRepository::add_follow_user(
            &mut tx,
            source_user.user_id,
            destinatino_user.user_id,
        )
        .await?;

        let followed_user =
            InternalUserRepository::fetch_follow_list(&mut tx, source_user.user_id).await?;
        assert_eq!(&destinatino_user, followed_user.first().unwrap());

        let follow_user =
            InternalUserRepository::fetch_follower_list(&mut tx, destinatino_user.user_id).await?;
        assert_eq!(&source_user, follow_user.first().unwrap());

        InternalUserRepository::add_follow_user(
            &mut tx,
            destinatino_user.user_id,
            source_user.user_id,
        )
        .await?;
        let friend_list =
            InternalUserRepository::fetch_friend_list(&mut tx, source_user.user_id).await?;
        assert_eq!(&destinatino_user, friend_list.first().unwrap());

        Ok(())
    }

    async fn create_pool() -> Pool<MySql> {
        dotenv::dotenv().expect("Failed to dotenv");
        sqlx::mysql::MySqlPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("Failed to envkey"))
            .await
            .expect("Failed to pool")
    }
}
