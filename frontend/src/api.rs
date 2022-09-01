use anyhow::Context;
use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestCredentials, RequestMode};

#[derive(Debug, Clone)]
pub(crate) struct Error {
    msg: String,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.msg.clone()
    }
}

pub(crate) async fn fetch<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let resp = Request::get(url)
        .mode(RequestMode::Cors)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| Error { msg: e.to_string() });

    let t: T = resp?.json().await.unwrap();

    Ok(t)
}
