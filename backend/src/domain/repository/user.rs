use anyhow::Context;
use async_trait::async_trait;
use sqlx::{MySql, Pool};

use crate::{
    domain::entity::user::{NewUser, User},
    infrastructure::user::InternalUserRepository,
};

#[async_trait]
pub(crate) trait UserRepository {
    async fn add_new_user(&self, new_user: &NewUser) -> anyhow::Result<u64>;
    async fn add_new_user_return_it(&self, new_user: &NewUser) -> anyhow::Result<User>;
    async fn find_user_by_google_id(&self, google_id: &str) -> anyhow::Result<Option<User>>;
    async fn find_user_by_user_id(&self, user_id: u64) -> anyhow::Result<Option<User>>;
    async fn add_follow_user(&self, src_uid: u64, dist_uid: u64) -> anyhow::Result<()>;
    async fn fetch_friend_list(&self, user_id: u64) -> anyhow::Result<Vec<User>>;
}

pub struct MySqlUserRepository {
    pool: Pool<MySql>,
}

impl MySqlUserRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn add_new_user(&self, new_user: &NewUser) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::add_new_user(&mut conn, new_user).await
    }

    async fn add_new_user_return_it(&self, new_user: &NewUser) -> anyhow::Result<User> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::add_new_user_return_it(&mut conn, new_user).await
    }

    async fn find_user_by_google_id(&self, google_id: &str) -> anyhow::Result<Option<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::find_user_by_google_id(&mut conn, google_id).await
    }

    async fn find_user_by_user_id(&self, user_id: u64) -> anyhow::Result<Option<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::find_user_by_user_id(&mut conn, user_id).await
    }

    async fn add_follow_user(&self, src_uid: u64, dist_uid: u64) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;
        InternalUserRepository::add_follow_user(&mut conn, src_uid, dist_uid).await?;

        Ok(())
    }

    async fn fetch_friend_list(&self, user_id: u64) -> anyhow::Result<Vec<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;
        let friend_list = InternalUserRepository::fetch_friend_list(&mut conn, user_id).await?;

        Ok(friend_list)
    }
}
