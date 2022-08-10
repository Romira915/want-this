use want_this_frontend::component::cta::Cta;
use want_this_frontend::component::header::Header;
use want_this_frontend::{App, CONFIG};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestCredentials;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{html::Scope, prelude::*};
use yew_hooks::prelude::*;

use yew_hooks::use_async;
use yew_router::prelude::*;

fn main() {
    let config = wasm_logger::Config::new(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    });
    wasm_logger::init(config);
    yew::start_app::<App>();
}
