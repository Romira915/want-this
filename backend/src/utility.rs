use actix_session::Session;
use actix_web::Error;

use crate::session::SessionKey;

pub(crate) fn get_user_id(session: &Session) -> Result<Option<u64>, Error> {
    session.get::<u64>(SessionKey::UserId.as_ref())
}
