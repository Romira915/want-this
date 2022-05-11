use std::collections::HashMap;
use std::fs::File;
use std::{convert::Infallible, fmt::format};
use std::{env, io};

use actix_files::{Files, NamedFile};
use actix_redis::RedisSession;
use actix_session::Session;
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
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

const AUTH_URL: OnceCell<AuthUrl> = OnceCell::new();
const TOKEN_URL: OnceCell<TokenUrl> = OnceCell::new();
const GOOGLE_CLIENT_SECRET: OnceCell<ClientSecret> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
struct GoogleOAuth {
    pub g_csrf_token: String,
    pub credential: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePayload {
    pub iss: String,
    pub nbf: u32,
    pub aud: String,
    pub sub: String,
    pub hd: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub iat: u32,
    pub exp: u32,
    pub jti: String,
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
    Ok(NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
async fn welcome(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
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

#[post("/login/callback")]
async fn login(
    req: HttpRequest,
    google: web::Form<GoogleOAuth>,
    session: Session,
) -> Result<HttpResponse> {
    println!("{:?}", &req);
    println!("oauth {:?}", &google);
    let id = session.get::<String>("g_csrf_token")?;
    println!("user_id {:?}", id);
    let counter = session.get::<i32>("counter")?;
    println!("counter {:?}", counter);
    session.insert("user_id", &google.g_csrf_token)?;
    session.renew();

    let resp = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await
        .unwrap();
    let publick_key: jwk::JwkSet = resp.json().await.unwrap();

    let header = decode_header(&google.credential).unwrap();
    let kid = header.kid.unwrap();
    if let Some(j) = publick_key.find(&kid) {
        match j.algorithm {
            jwk::AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                let mut validation = Validation::new(j.common.algorithm.unwrap());
                validation.validate_exp = false;
                let mut decoded_token =
                    decode::<GooglePayload>(&google.credential, &decoding_key, &validation)
                        .unwrap();
            }
            _ => unreachable!("this should be a RSA"),
        }
    }

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(format!("{:?}", google)))
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

/// response body
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();

    HttpResponse::Ok().streaming(stream! {
        yield Ok::<_, Infallible>(web::Bytes::from("Hello "));
        yield Ok::<_, Infallible>(web::Bytes::from(name));
        yield Ok::<_, Infallible>(web::Bytes::from("!"));
    })
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Hello {}!", path.0))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().unwrap();
    env::set_var(
        "RUST_LOG",
        "actix_web=debug,actix_redis=info,actix_server=info",
    );
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8888");

    let private_key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            // cookie session middleware
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .wrap(RedisSession::new("localhost:6379", private_key.master()))
            // register favicon
            .service(favicon)
            // register simple route, handle all methods
            .service(welcome)
            .service(login)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async response body
            .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
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
            // static files
            .service(Files::new("/static", "static").show_files_listing())
            // redirect
            // .service(
            //     web::resource("/").route(web::get().to(|req: HttpRequest| async move {
            //         println!("{:?}", req);
            //         HttpResponse::Found()
            //             .insert_header((header::LOCATION, "static/welcome.html"))
            //             .finish()
            //     })),
            // )
            // default
            .service(Files::new("/", "../frontend/dist/").index_file("index.html"))
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", 8888))?
    .workers(2)
    .run()
    .await
}
