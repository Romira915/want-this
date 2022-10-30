use anyhow::Context;
use api_format::Organization as OrganizationAPI;
use sqlx::{Connection, MySqlConnection};

use crate::domain::entity::{
    organizations::{JoinOrganization, NewOrganization, Organization},
    users::User,
};

use super::take_n_str;

const MAX_LEN_ORG_NAME: usize = 100;
const MAX_LEN_ORG_DESCRIPTION: usize = 255;

pub(crate) struct InternalOrganizationRepository;

impl InternalOrganizationRepository {
    pub(crate) async fn create_organization(
        conn: &mut MySqlConnection,
        org_id: u64,
        new_org: &NewOrganization,
    ) -> anyhow::Result<u64> {
        let name = take_n_str(&new_org.name, MAX_LEN_ORG_NAME);
        let id = sqlx::query!(
            "INSERT INTO organizations (organization_id, organization_name, description, owner)
            VALUES (?, ?, ?, ?);",
            org_id,
            name,
            new_org.description,
            new_org.owner_id
        )
        .execute(conn)
        .await
        .context("Failed to create_organization")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn join_organization(
        conn: &mut MySqlConnection,
        join_org: &JoinOrganization,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "INSERT INTO users_organizations (user_id, organization_id, edit_permission) 
            VALUES (?, ?, ?);",
            join_org.user_id,
            join_org.org_id,
            join_org.edit_permission
        )
        .execute(conn)
        .await
        .context("Failed to join_organization")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn find_org_by_org_id(
        conn: &mut MySqlConnection,
        org_id: u64,
    ) -> anyhow::Result<Organization> {
        let org = sqlx::query_as!(
            Organization,
            "SELECT organization_id, organization_name, description, is_public, owner 
             FROM organizations WHERE organization_id = ?;",
            org_id
        )
        .fetch_one(conn)
        .await
        .context("Failed to find_org_by_org_id")?;

        Ok(org)
    }

    pub(crate) async fn update_org(
        conn: &mut MySqlConnection,
        update_org: &OrganizationAPI,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "UPDATE organizations SET organization_name = ?, description = ?,
            is_public = ?, owner = ? 
            WHERE organization_id = ?;",
            update_org.organization_name,
            update_org.description,
            update_org.is_public,
            update_org.owner,
            update_org.organization_id
        )
        .execute(conn)
        .await
        .context("Failed to update_org")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_org_name(
        conn: &mut MySqlConnection,
        org_id: u64,
        org_name: &str,
    ) -> anyhow::Result<u64> {
        let org_name = take_n_str(org_name, MAX_LEN_ORG_NAME);
        let id = sqlx::query!(
            "UPDATE organizations SET organization_name = ? WHERE organization_id = ?;",
            org_name,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_org_name")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_org_description(
        conn: &mut MySqlConnection,
        org_id: u64,
        org_description: &str,
    ) -> anyhow::Result<u64> {
        let org_description = take_n_str(org_description, MAX_LEN_ORG_DESCRIPTION);
        let id = sqlx::query!(
            "UPDATE organizations SET description = ? WHERE organization_id = ?;",
            org_description,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_org_description")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_is_public(
        conn: &mut MySqlConnection,
        org_id: u64,
        is_public: bool,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "UPDATE organizations SET is_public = ? WHERE organization_id = ?;",
            is_public,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_is_public")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_owner(
        conn: &mut MySqlConnection,
        org_id: u64,
        owner_id: bool,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "UPDATE organizations SET owner = ? WHERE organization_id = ?;",
            owner_id,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_owner")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn fetch_public_orgs(
        conn: &mut MySqlConnection,
    ) -> anyhow::Result<Vec<Organization>> {
        let org_list = sqlx::query_as!(
            Organization,
            "SELECT organization_id, organization_name, description, is_public, owner 
        FROM organizations WHERE is_public = 1;"
        )
        .fetch_all(conn)
        .await
        .context("Failed to fetch_public_org_list")?;

        Ok(org_list)
    }

    pub(crate) async fn fetch_joined_orgs(
        conn: &mut MySqlConnection,
        user_id: u64,
    ) -> anyhow::Result<Vec<Organization>> {
        let org_list = sqlx::query_as!(
            Organization,
            "SELECT organization_id, organization_name, description, is_public, owner 
        FROM organizations INNER JOIN
        (SELECT organization_id FROM users_organizations WHERE user_id = ?) AS joined_list
        USING(organization_id);",
            user_id
        )
        .fetch_all(conn)
        .await
        .context("Failed to fetch_joined_org_list")?;

        Ok(org_list)
    }

    pub(crate) async fn fetch_joined_users(
        conn: &mut MySqlConnection,
        org_id: u64,
    ) -> anyhow::Result<Vec<User>> {
        let user_list = sqlx::query_as!(
            User,
            "SELECT user_id, google_id, user_name
        FROM users INNER JOIN
        (SELECT user_id FROM users_organizations WHERE organization_id = ?) AS joined_list
        USING(user_id);",
            org_id
        )
        .fetch_all(conn)
        .await
        .context("Failed to fetch_joined_user_list")?;

        Ok(user_list)
    }

    pub(crate) async fn delete_org(conn: &mut MySqlConnection, org_id: u64) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "DELETE FROM organizations WHERE organization_id = ?",
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to delete_org")?
        .last_insert_id();

        Ok(id)
    }
}
