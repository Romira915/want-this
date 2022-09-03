use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::time::Duration;
use std::{convert::Infallible, fmt::format};
use std::{env, io};

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::cookie::time::macros::offset;
use actix_web::cookie::time::UtcOffset;
use actix_web::web::Data;
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, post,
    web::{self},
    App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use actix_web::{http, HttpMessage};
use async_stream::stream;
use chrono::{FixedOffset, Utc};
use json_format::User;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};
use log::LevelFilter;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};
use want_this_backend::auth::{decode_google_jwt_with_jwturl, GooglePayload};
use want_this_backend::domain::repository::user::MySqlUserRepository;
use want_this_backend::domain::service::auth::{auth, logout};
use want_this_backend::domain::service::user::icon;
use want_this_backend::session::SessionKey;
use want_this_backend::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
struct GoogleOAuth {
    pub g_csrf_token: String,
    pub credential: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePublicKey {
    pub kty: String,
    pub n: String,
    pub e: String,
    pub alg: String,
    pub r#use: String,
    pub kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePublicKeyList {
    keys: Vec<GooglePublicKey>,
}

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("./backend/static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
async fn welcome(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{:?}", req);

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.insert("counter", counter)?;
    println!("counter {}", counter);

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(format!("{:?}", session.get::<String>("user_id"))))
}

#[get("login/state")]
async fn login_state(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    let id = session.get::<String>(SessionKey::GoogleId.as_ref())?;
    log::debug!("state {:?}", id);

    let user = User::new(id);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(&user))
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file =
                NamedFile::open("backend/static/404.html")?.set_status_code(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

fn init_logger<P: AsRef<Path>>(log_path: Option<P>) {
    const JST_UTCOFFSET_SECS: i32 = 9 * 3600;

    let jst_now = {
        let jst = Utc::now();
        jst.with_timezone(&FixedOffset::east(JST_UTCOFFSET_SECS))
    };

    let offset = UtcOffset::from_whole_seconds(JST_UTCOFFSET_SECS).unwrap();

    let mut config = ConfigBuilder::new();
    config.set_time_offset(offset);

    let mut logger: Vec<Box<dyn SharedLogger>> = vec![
        #[cfg(not(feature = "termcolor"))]
        TermLogger::new(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
            config.build(),
            TerminalMode::Mixed,
            ColorChoice::Always,
        ),
    ];
    if let Some(log_path) = log_path {
        let log_path = log_path.as_ref();
        fs::create_dir_all(&log_path).unwrap();
        logger.push(WriteLogger::new(
            LevelFilter::Info,
            config.build(),
            File::create(log_path.join(format!("{}.log", jst_now))).unwrap(),
        ));
    }
    CombinedLogger::init(logger).unwrap()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().unwrap();
    env::set_var(
        "RUST_LOG",
        "actix_web=info,actix_redis=info,`actix_server=info",
    );
    init_logger(if cfg!(debug_assertions) {
        None
    } else {
        Some("/var/log/want-this")
    });

    let num_cpus = num_cpus::get();

    log::info!("starting HTTP server at http://0.0.0.0:4080");
    log::debug!("database url {}", CONFIG.get_database_url());

    let private_key = actix_web::cookie::Key::generate();
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(num_cpus as u32)
        .connect_timeout(Duration::from_secs(1))
        .connect(&CONFIG.get_database_url())
        .await
        .unwrap();

    HttpServer::new(move || {
        let user_repository = MySqlUserRepository::new(pool.clone());

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(RedisSession::new(&CONFIG.redis_url, private_key.master()))
            .wrap(if cfg!(debug_assertions) {
                Cors::permissive()
            } else {
                Cors::default()
                    .supports_credentials()
                    .allowed_origin(&CONFIG.frontend_origin)
            })
            .app_data(Data::new(user_repository))
            .service(favicon)
            .service(welcome)
            .service(login_state)
            .service(auth)
            .service(logout)
            .service(icon)
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            .service(Files::new("/static", "backend/static").show_files_listing())
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, CONFIG.frontend_origin.as_str()))
                        .finish()
                })),
            )
            .default_service(web::to(default_handler))
    })
    .bind(("0.0.0.0", 4080))?
    .workers(num_cpus)
    .run()
    .await
}
