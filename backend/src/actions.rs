use derive_more::Constructor;
use uuid::Uuid;

pub mod users;

#[derive(Debug, Constructor)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Constructor)]
pub struct Friend {
    pub follow: u64,
}
