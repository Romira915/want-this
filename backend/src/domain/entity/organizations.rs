use api_format::Organization as OrganizationAPI;
use std::fmt::Display;

use derive_more::Constructor;

#[derive(Debug, Clone, PartialEq, Constructor)]
pub struct Organization {
    pub organization_id: u64,
    pub organization_name: String,
    pub description: Option<String>,
    pub is_public: i8,
    pub owner: u64,
}

impl Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id {}, name {}, dsc {:?}, public {}, owner {}",
            self.organization_id,
            self.organization_name,
            self.description,
            self.is_public,
            self.owner
        )
    }
}

impl From<Organization> for OrganizationAPI {
    fn from(org: Organization) -> Self {
        Self::new(
            org.organization_id.to_string(),
            org.organization_name,
            org.description,
            org.is_public,
            org.owner.to_string(),
        )
    }
}

#[derive(Debug, Constructor)]
pub(crate) struct NewOrganization {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: u64,
}

#[derive(Debug, Constructor)]
pub(crate) struct JoinOrganization {
    pub user_id: u64,
    pub org_id: u64,
    pub edit_permission: bool,
}
