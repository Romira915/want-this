use anyhow::Context;
use api_format::Organization as OrganizationAPI;
use async_trait::async_trait;
use derive_more::Constructor;
use sqlx::{MySql, Pool};

use crate::{
    domain::entity::{
        organizations::{
            JoinOrganization, JoinRequestOrganization, JoinStatus, NewOrganization, Organization,
        },
        users::User,
    },
    infrastructure::{create_uuid_short, organizations::InternalOrganizationRepository},
};

#[async_trait]
pub(crate) trait OrganizationsRepository {
    // Create
    async fn create_organization_and_join(&self, new_org: &NewOrganization) -> anyhow::Result<u64>;
    async fn join_organization(&self, join_org: &JoinOrganization) -> anyhow::Result<u64>;
    async fn join_request_organization(
        &self,
        join_req_org: &JoinRequestOrganization,
    ) -> anyhow::Result<u64>;
    // Read
    async fn find_org_by_org_id(&self, org_id: u64) -> anyhow::Result<Organization>;
    async fn fetch_public_orgs(&self) -> anyhow::Result<Vec<Organization>>;
    async fn fetch_joined_orgs(&self, user_id: u64) -> anyhow::Result<Vec<Organization>>;
    async fn fetch_joined_users(&self, org_id: u64) -> anyhow::Result<Vec<User>>;
    async fn fetch_edit_permission(
        &self,
        user_id: u64,
        org_id: u64,
    ) -> anyhow::Result<Option<bool>>;
    async fn fetch_join_request_is_pending_users(&self, org_id: u64) -> anyhow::Result<Vec<User>>;
    // Update
    async fn update_org(&self, update_org: &OrganizationAPI) -> anyhow::Result<u64>;
    async fn update_org_name(&self, org_id: u64, org_name: &str) -> anyhow::Result<u64>;
    async fn update_org_description(
        &self,
        org_id: u64,
        org_description: &str,
    ) -> anyhow::Result<u64>;
    async fn update_is_public(&self, org_id: u64, is_public: bool) -> anyhow::Result<u64>;
    async fn update_owner(&self, org_id: u64, owner_id: u64) -> anyhow::Result<u64>;
    async fn update_join_status(
        &self,
        user_id: u64,
        org_id: u64,
        join_status: &JoinStatus,
    ) -> anyhow::Result<u64>;
    // Delete
    async fn delete_org(&self, org_id: u64) -> anyhow::Result<u64>;
    async fn delete_user_from_organization(
        &self,
        org_id: u64,
        delete_user_id: u64,
    ) -> anyhow::Result<u64>;
}

#[derive(Debug, Constructor)]
pub struct MySqlOrganizationsRepository {
    pool: Pool<MySql>,
}

#[async_trait]
impl OrganizationsRepository for MySqlOrganizationsRepository {
    async fn create_organization_and_join(&self, new_org: &NewOrganization) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        let org_id = create_uuid_short(&mut conn).await?;
        InternalOrganizationRepository::create_organization(&mut conn, org_id, new_org).await?;
        let join_org = JoinOrganization::new(new_org.owner_id, org_id, true);
        InternalOrganizationRepository::join_organization(&mut conn, &join_org).await
    }

    async fn join_organization(&self, join_org: &JoinOrganization) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::join_organization(&mut conn, &join_org).await
    }

    async fn join_request_organization(
        &self,
        join_req_org: &JoinRequestOrganization,
    ) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::join_request_organization(&mut conn, join_req_org).await
    }

    async fn find_org_by_org_id(&self, org_id: u64) -> anyhow::Result<Organization> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::find_org_by_org_id(&mut conn, org_id).await
    }

    async fn fetch_public_orgs(&self) -> anyhow::Result<Vec<Organization>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::fetch_public_orgs(&mut conn).await
    }

    async fn fetch_joined_orgs(&self, user_id: u64) -> anyhow::Result<Vec<Organization>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::fetch_joined_orgs(&mut conn, user_id).await
    }

    async fn fetch_joined_users(&self, org_id: u64) -> anyhow::Result<Vec<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::fetch_joined_users(&mut conn, org_id).await
    }

    async fn fetch_edit_permission(
        &self,
        user_id: u64,
        org_id: u64,
    ) -> anyhow::Result<Option<bool>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::fetch_edit_permission(&mut conn, user_id, org_id).await
    }

    async fn fetch_join_request_is_pending_users(&self, org_id: u64) -> anyhow::Result<Vec<User>> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::fetch_join_request_is_pending_users(&mut conn, org_id).await
    }

    async fn update_org(&self, update_org: &OrganizationAPI) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::update_org(&mut conn, update_org).await
    }

    async fn update_org_name(&self, org_id: u64, org_name: &str) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::update_org_name(&mut conn, org_id, org_name).await
    }

    async fn update_org_description(
        &self,
        org_id: u64,
        org_description: &str,
    ) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::update_org_description(&mut conn, org_id, org_description)
            .await
    }

    async fn update_is_public(&self, org_id: u64, is_public: bool) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::update_is_public(&mut conn, org_id, is_public).await
    }

    async fn update_owner(&self, org_id: u64, owner_id: u64) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::update_owner(&mut conn, org_id, owner_id).await
    }

    async fn update_join_status(
        &self,
        user_id: u64,
        org_id: u64,
        join_status: &JoinStatus,
    ) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::update_join_status(&mut conn, user_id, org_id, join_status)
            .await
    }

    async fn delete_org(&self, org_id: u64) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::delete_org(&mut conn, org_id).await
    }

    async fn delete_user_from_organization(
        &self,
        org_id: u64,
        delete_user_id: u64,
    ) -> anyhow::Result<u64> {
        let mut conn = self.pool.acquire().await.context("Failed to acquire")?;

        InternalOrganizationRepository::delete_user_from_organization(
            &mut conn,
            org_id,
            delete_user_id,
        )
        .await
    }
}
