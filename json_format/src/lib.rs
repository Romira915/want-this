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
