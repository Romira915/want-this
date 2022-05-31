use actix_session::Session;
use actix_web::{
    http::header,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::{
    auth::decode_google_jwt_with_jwturl,
    domain::{
        entity::{google::GoogleOAuth, user::NewUser},
        repository::user::{MySqlUserRepository, UserRepository},
    },
    media::save_bytes,
    session::SessionKey,
};

#[post("/auth")]
async fn auth(
    _req: HttpRequest,
    google: web::Form<GoogleOAuth>,
    session: Session,
    user_repo: Data<MySqlUserRepository>,
) -> Result<HttpResponse> {
    let google_payload = if let Ok(g) = decode_google_jwt_with_jwturl(&google.credential).await {
        g
    } else {
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    };

    session.renew();

    let user = match user_repo.find_user_by_google_id(&google_payload.sub).await {
        // exist already
        Ok(Some(user)) => user,
        // register
        Ok(None) => {
            let icon_path = match reqwest::get(&google_payload.picture).await {
                Ok(request) => match request.bytes().await {
                    Ok(bytes) => {
                        let icon_path =
                            format!("image/{}/{}", &google_payload.sub, &google_payload.sub);
                        if let Err(e) = save_bytes(&icon_path, &bytes).await {
                            log::warn!("{:?}", &e);
                        }

                        Some(icon_path)
                    }
                    Err(e) => {
                        log::warn!("{:?}", &e);
                        None
                    }
                },
                Err(e) => {
                    log::warn!("{:?}", &e);
                    None
                }
            };

            let new_user = NewUser::new(
                google_payload.sub.clone(),
                Some(google_payload.name.clone()),
                icon_path,
            );

            match user_repo.add_new_user_return_it(&new_user).await {
                Ok(user) => user,
                Err(e) => {
                    log::warn!("{:?}", &e);
                    return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to find_user_by_google_id {:?}", &e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };
    log::debug!("login user {:?}", google_payload);

    if let Err(e) = session.insert(SessionKey::UserId.as_ref(), &user.user_id) {
        log::warn!("Failed to session insert {:?}", &e);
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    }
    if let Err(e) = session.insert(SessionKey::GoogleId.as_ref(), &google_payload.sub) {
        log::warn!("Failed to session insert {:?}", &e);
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    }
    if let Err(e) = session.insert(SessionKey::ExpirationTime.as_ref(), &google_payload.exp) {
        log::warn!("Failed to session insert {:?}", &e);
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    }

    // response
    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .append_header((header::LOCATION, "http://localhost:8080/login/state"))
        .finish())
}
