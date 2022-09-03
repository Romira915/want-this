use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub google_id: Option<String>,
}

impl User {
    pub fn new(google_id: Option<String>) -> Self {
        Self { google_id }
    }
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct Organization {
    pub organization_id: u64,
    pub organization_name: String,
    pub description: Option<String>,
    pub is_public: i8,
    pub owner: u64,
}
