use actix_files::{Files, NamedFile};
use actix_web::{
    cookie::Cookie,
    get,
    http::header::ContentType,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder, Result,
};
use reqwest::StatusCode;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

use crate::{
    domain::repositories::users::{MySqlUsersRepository, UsersRepository},
    media::load_bytes,
};

#[get("/users/{user_id}/icon")]
async fn icon(
    path: web::Path<u64>,
    user_repo: Data<MySqlUsersRepository>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();

    let icon_path = match user_repo.get_icon_path_by_user_id(user_id).await {
        Ok(path) => path,
        Err(e) => {
            log::warn!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let icon = if let Some(path) = icon_path {
        let icon = match load_bytes(&path).await {
            Ok(icon) => icon,
            Err(e) => {
                log::warn!("{:?}", &e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        };

        icon
    } else {
        return Ok(HttpResponse::NotFound().finish());
    };

    // let icon_path = if let Some(path) = icon_path {
    //     path
    // } else {
    //     return Ok(HttpResponse::NotFound().finish());
    // };

    // TODO: NamedFile等のライブラリ準拠を使いたい
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::jpeg())
        .body(icon))
}
