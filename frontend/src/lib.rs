use crate::component::header::Header;
use derive_more::Constructor;
use once_cell::sync::Lazy;
use route::main::{switch_main, MainRoute};
use serde::Deserialize;
use yew::{function_component, html};
use yew_router::prelude::*;

pub(crate) mod api;
pub mod bindings;
pub mod component;
pub mod route;

#[derive(Debug, Deserialize, Constructor)]
pub struct Config {
    pub backend_origin: &'static str,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new(if cfg!(debug_assertions) {
        "http://localhost:4080"
    } else {
        "https://api.want-this.romira.dev"
    })
});

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<MainRoute> render={Switch::render(switch_main)} />
        </BrowserRouter>
    }
}
