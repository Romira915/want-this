use reqwasm::http::Request;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub(crate) async fn fetch(url: &str) -> Result<String, JsValue> {
    let resp = Request::get(url).send().await;

    todo!()
}
