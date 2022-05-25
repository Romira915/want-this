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

use crate::{auth::decode_google_jwt_with_jwturl, session::SessionKey};

#[derive(Debug, Serialize, Deserialize)]
struct GoogleOAuth {
    pub g_csrf_token: String,
    pub credential: String,
}

#[post("/auth")]
async fn auth(
    _req: HttpRequest,
    google: web::Form<GoogleOAuth>,
    session: Session,
    pool: Data<Pool<MySql>>,
) -> Result<HttpResponse> {
    let google_payload = if let Ok(g) = decode_google_jwt_with_jwturl(&google.credential).await {
        g
    } else {
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    };

    session.renew();
    if let Err(e) = session.insert(SessionKey::GoogleId.as_ref(), &google_payload.sub) {
        log::warn!("Failed to session insert {}", &e);
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    }
    if let Err(e) = session.insert(SessionKey::ExpirationTime.as_ref(), &google_payload.exp) {
        log::warn!("Failed to session insert {}", &e);
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    }

    let uuid = Uuid::new_v4().as_u128().to_string();
    log::debug!("uuid {}", uuid);

    if let Err(e) = sqlx::query!(
        "INSERT IGNORE INTO users (user_id, user_name, friend_id) VALUES (?, ?, UUID())",
        &google_payload.sub,
        &google_payload.name,
    )
    .execute(pool.as_ref())
    .await
    {
        log::warn!("Failed to DB insert {}", &e);
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
    }

    // response
    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .append_header((header::LOCATION, "http://localhost:8080/login/state"))
        .finish())
}
