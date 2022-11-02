use actix_session::Session;
use actix_web::Error;
use reqwest::Request;

use crate::session::SessionKey;

pub(crate) fn is_login(session: &Session) -> Result<bool, Error> {
    Ok(session.get::<u64>(SessionKey::UserId.as_ref())?.is_some())
}

pub(crate) fn get_user_id(session: &Session) -> Result<Option<u64>, Error> {
    session.get::<u64>(SessionKey::UserId.as_ref())
}

pub(crate) fn get_user_id_unchecked(session: &Session) -> Option<u64> {
    session
        .get::<u64>(SessionKey::UserId.as_ref())
        .expect("Failed to settsion get()")
}
