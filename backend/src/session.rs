use strum::{AsRefStr, EnumString};

#[derive(Debug, EnumString, AsRefStr)]
pub enum SessionKey {
    GoogleId,
    ExpirationTime,
}
