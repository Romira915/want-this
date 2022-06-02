use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub(crate) struct Organization {
    pub organization_id: u64,
    pub organization_name: String,
    pub description: Option<String>,
    pub is_public: i8,
    pub owner: u64,
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
