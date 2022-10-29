use actix_session::Session;
use actix_web::Error;

use crate::session::SessionKey;

pub(crate) fn is_login(session: &Session) -> Result<bool, Error> {
    Ok(session.get::<u64>(SessionKey::UserId.as_ref())?.is_some())
}
