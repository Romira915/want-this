use reqwasm::http::Request;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestCredentials, RequestMode};

pub(crate) async fn fetch(url: &str) -> Result<String, JsValue> {
    let resp = Request::get(url)
        .mode(RequestMode::Cors)
        .credentials(RequestCredentials::Include)
        .send()
        .await;
    log::debug!("{:#?}", resp.unwrap().text().await.unwrap());
    todo!()
}
