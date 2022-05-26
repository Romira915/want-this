use derive_more::Constructor;

pub mod users;

#[derive(Debug, Constructor)]
pub struct User {
    pub id: String,
    pub name: String,
}
