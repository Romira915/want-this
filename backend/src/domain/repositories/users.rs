use anyhow::Context;
use async_trait::async_trait;
use derive_more::Constructor;
use sqlx::{MySql, Pool};

use crate::{
    domain::entity::users::{NewUser, UpdateUser, User},
    infrastructure::users::InternalUserRepository,
};

#[async_trait]
pub(crate) trait UsersRepository {
    // Create
    async fn add_new_user(&self, new_user: &NewUser) -> anyhow::Result<u64>;
    async fn add_new_user_return_it(&self, new_user: &NewUser) -> anyhow::Result<User>;
    async fn add_follow_user(&self, src_uid: u64, dist_uid: u64) -> anyhow::Result<()>;
    // Read
    async fn find_user_by_google_id(&self, google_id: &str) -> anyhow::Result<Option<User>>;
    async fn find_user_by_user_id(&self, user_id: u64) -> anyhow::Result<Option<User>>;
    async fn fetch_follows(&self, source_user_id: u64) -> anyhow::Result<Vec<User>>;
    async fn fetch_followers(&self, destination_user_id: u64) -> anyhow::Result<Vec<User>>;
    async fn fetch_friends(&self, user_id: u64) -> anyhow::Result<Vec<User>>;
    async fn get_icon_path_by_user_id(&self, user_id: u64) -> anyhow::Result<Option<String>>;
    // Update
    async fn update_user_name(&self, update_user: &UpdateUser) -> anyhow::Result<u64>;
}

#[derive(Debug, Constructor)]
pub struct MySqlUsersRepository {
    pool: Pool<MySql>,
}

#[async_trait]
impl UsersRepository for MySqlUsersRepository {
    async fn add_new_user(&self, new_user: &NewUser) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::add_new_user(&mut conn, new_user).await
    }

    async fn add_new_user_return_it(&self, new_user: &NewUser) -> anyhow::Result<User> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::add_new_user_return_it(&mut conn, new_user).await
    }

    async fn add_follow_user(&self, src_uid: u64, dist_uid: u64) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::add_follow_user(&mut conn, src_uid, dist_uid).await
    }

    async fn find_user_by_google_id(&self, google_id: &str) -> anyhow::Result<Option<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::find_user_by_google_id(&mut conn, google_id).await
    }

    async fn find_user_by_user_id(&self, user_id: u64) -> anyhow::Result<Option<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::find_user_by_user_id(&mut conn, user_id).await
    }

    async fn fetch_follows(&self, source_user_id: u64) -> anyhow::Result<Vec<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::fetch_follows(&mut conn, source_user_id).await
    }

    async fn fetch_followers(&self, destination_user_id: u64) -> anyhow::Result<Vec<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::fetch_followers(&mut conn, destination_user_id).await
    }

    async fn fetch_friends(&self, user_id: u64) -> anyhow::Result<Vec<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::fetch_friends(&mut conn, user_id).await
    }

    async fn get_icon_path_by_user_id(&self, user_id: u64) -> anyhow::Result<Option<String>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::get_icon_path_by_user_id(&mut conn, user_id).await
    }

    async fn update_user_name(&self, update_user: &UpdateUser) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalUserRepository::update_user_name(&mut conn, update_user).await
    }
}
