use derive_more::Constructor;

#[derive(Debug, PartialEq, Eq, Constructor)]
pub(crate) struct Item {
    pub item_id: u64,
    pub having_organization_id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub(crate) struct NewItem {
    pub having_organization_id: String,
    pub url: Option<String>,
    pub title: Option<String>,
    pub memo: Option<String>,
}
