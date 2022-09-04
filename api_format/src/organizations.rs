use std::fmt::Display;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Constructor, Serialize, Deserialize)]
pub struct Organization {
    pub organization_id: u64,
    pub organization_name: String,
    pub description: Option<String>,
    pub is_public: i8,
    pub owner: u64,
}

impl Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id {}, name {}, dsc {:?}, public {}, owner {}",
            self.organization_id,
            self.organization_name,
            self.description,
            self.is_public,
            self.owner
        )
    }
}
