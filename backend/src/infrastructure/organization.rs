use anyhow::Context;
use sqlx::{Connection, MySqlConnection};

use crate::domain::entity::organization::{JoinOrganization, NewOrganization, Organization};

pub(crate) struct InternalOrganizationRepository;

impl InternalOrganizationRepository {
    pub(crate) async fn create_organization(
        conn: &mut MySqlConnection,
        org_id: u64,
        new_org: &NewOrganization,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "INSERT INTO organizations (organization_id, organization_name, description, owner)
            VALUES (?, ?, ?, ?);",
            org_id,
            new_org.name,
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
    ) -> anyhow::Result<Vec<Organization>> {
        let org = sqlx::query_as!(
            Organization,
            "SELECT organization_id, organization_name, description, is_public, owner 
             FROM organizations WHERE organization_id = ?",
            org_id
        )
        .fetch_all(conn)
        .await
        .context("Failed to find_org_by_org_id")?;

        Ok(org)
    }
}
