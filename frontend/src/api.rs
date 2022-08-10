use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

pub(crate) async fn fetch(url: &str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET")
        .mode(RequestMode::Cors)
        .credentials(RequestCredentials::Include);

    let request = Request::new_with_str_and_init(url, &opts)?;
    log::info!("request");

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    log::info!("resp_value");

    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?;
    log::info!("text");

    Ok(text.as_string().unwrap())
}
