use derive_more::Constructor;

#[derive(Debug, PartialEq, Eq, Constructor)]
pub(crate) struct User {
    pub user_id: u64,
    pub google_id: String,
    pub user_name: Option<String>,
}

#[derive(Debug, Constructor)]
pub(crate) struct NewUser {
    pub google_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Constructor)]
pub(crate) struct Friend {
    pub friend_id: u64,
}
