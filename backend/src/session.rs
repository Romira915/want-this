use strum::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr)]
pub enum SessionKey {
    UserId,
    GoogleId,
    ExpirationTime,
}
