use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct User {
    pub user_id: String,
    pub google_id: Option<String>,
    pub user_name: Option<String>,
}
