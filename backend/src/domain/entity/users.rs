use api_format::User as UserAPI;
use derive_more::Constructor;

/// NOTE: 今後のことも考えて `google_id` はOptionにすべき
#[derive(Debug, PartialEq, Eq, Constructor)]
pub(crate) struct User {
    pub user_id: u64,
    pub google_id: String,
    pub user_name: Option<String>,
}

impl From<User> for UserAPI {
    fn from(user: User) -> Self {
        UserAPI::new(
            user.user_id.to_string(),
            Some(user.google_id),
            user.user_name,
        )
    }
}

#[derive(Debug, Constructor)]
pub(crate) struct NewUser {
    pub google_id: String,
    pub name: Option<String>,
    pub icon_path: Option<String>,
}

#[derive(Debug, Constructor)]
pub(crate) struct UpdateUser {
    pub user_id: u64,
    pub name: String,
}

#[derive(Debug, Constructor)]
pub(crate) struct Friend {
    pub friend_id: u64,
}
