use derive_more::Constructor;

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
