mod organizations;
mod users;

pub use organizations::*;

use std::fmt::Display;

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
