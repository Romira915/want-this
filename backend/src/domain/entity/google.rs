use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GoogleOAuth {
    pub g_csrf_token: String,
    pub credential: String,
}
