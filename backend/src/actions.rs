use derive_more::Constructor;
use uuid::Uuid;

pub mod users;

#[derive(Debug, Constructor)]
pub struct User {
    pub google_id: String,
    pub user_id: u64,
    pub user_name: Option<String>,
}

#[derive(Debug, Constructor)]
pub struct NewUser {
    pub google_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Constructor)]
pub struct Friend {
    pub friend_id: u64,
}
