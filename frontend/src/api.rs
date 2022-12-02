use std::fmt::Display;

use anyhow::Context;
use derive_more::Constructor;
use gloo_utils::window;
use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestCredentials, RequestMode, RequestRedirect};

#[derive(Debug, Clone, Constructor)]
pub(crate) struct Error {
    msg: String,
    location: Option<String>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl Error {
    pub(crate) fn location(&self) -> &Option<String> {
        &self.location
    }
}

pub(crate) trait RequestBuild {
    fn build(url: &str) -> Request {
        Request::new(url)
            .mode(RequestMode::Cors)
            .credentials(RequestCredentials::Include)
        // .header("WantThis-Location", "")
    }
}

impl RequestBuild for Request {}

pub(crate) async fn fetch<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    log::debug!("fetch {}", url);
    let resp = Request::build(url)
        .method(Method::GET)
        .send()
        .await
        .map_err(|e| Error {
            msg: e.to_string(),
            location: None,
        })?;

    for (k, v) in resp.headers().entries() {
        log::debug!("{}: {}", k, v);
    }

    if let Some(loc) = resp.headers().get("WantThis-Location") {
        if !loc.is_empty() {
            return Err(Error::new(
                format!("{}: {}", resp.status(), resp.status_text()),
                Some(loc),
            ));
        }
    }

    let json: T = resp.json().await.unwrap();

    Ok(json)
}

pub(crate) async fn put_json<T>(url: &str, json: T) -> Result<(), Error>
where
    T: Serialize,
{
    log::debug!("post {}", url);
    let _resp = Request::build(url)
        .method(Method::PUT)
        .header("Content-type", "application/json")
        .body(serde_json::to_string(&json).unwrap())
        .send()
        .await
        .map_err(|e| Error {
            msg: e.to_string(),
            location: None,
        })?;

    Ok(())
}
