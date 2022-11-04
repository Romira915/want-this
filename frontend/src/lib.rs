use crate::component::header::Header;
use derive_more::Constructor;
use once_cell::sync::Lazy;
use route::main::{switch_main, MainRoute};
use serde::Deserialize;
use yew::{function_component, html};
use yew_router::prelude::*;

pub(crate) mod api;
pub(crate) mod bindings;
pub(crate) mod component;
pub(crate) mod route;

#[derive(Debug, Deserialize, Constructor)]
pub(crate) struct Config {
    pub backend_origin: &'static str,
}

pub(crate) static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new(if cfg!(debug_assertions) {
        "http://localhost:4080"
    } else {
        "https://api.want-this.romira.dev"
    })
});

#[function_component(App)]
pub fn app() -> Html {
    // TODO: ダークモード切り替えに対応する
    let color_mode = "dark";
    html! {
        <div class={color_mode}>
            <div class="w-screen h-screen bg-light-background dark:bg-dark-background">
                <BrowserRouter>
                    <Header />
                    <Switch<MainRoute> render={Switch::render(switch_main)} />
                </BrowserRouter>
            </div>
        </div>
    }
}
