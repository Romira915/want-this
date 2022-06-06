use want_this_frontend::component::header::Header;
use want_this_frontend::CONFIG;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestCredentials;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{html::Scope, prelude::*};
use yew_hooks::prelude::*;

mod bindings;

use yew_hooks::use_async;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/news")]
    News,
    #[at("/contact")]
    Contact,
    #[at("/settings/:s")]
    Settings,
    #[at("/login/:s")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum LoginRoute {
    #[at("/login/callback")]
    Callback,
    #[at("/login/state")]
    State,
    #[not_found]
    #[at("/login/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings/profile")]
    Profile,
    #[at("/settings/friends")]
    Friends,
    #[at("/settings/theme")]
    Theme,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

fn switch_main(route: &MainRoute) -> Html {
    match route {
        MainRoute::Home => {
            html! {
                <div>
                    <button class="bg-red-500 hover:bg-violet-500 active:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-300 ...">{ "+1" }</button>
                    <button class="bg-violet-500 hover:bg-violet-600 active:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-300 ...">{ "-1" }</button>
                    <Link<MainRoute> to={MainRoute::News}>{ "click here to go home" }</Link<MainRoute>>
                    <script src="https://accounts.google.com/gsi/client"></script>

                    <div id="g_id_onload"
                        data-client_id="839980808596-tq6nkmcik0nrohr079rj4vt5bdhvr15g.apps.googleusercontent.com"
                        data-context="signup"
                        data-ux_mode="popup"
                        data-login_uri={format!("{}/auth",CONFIG.backend_origin)}
                        data-auto_prompt="false">
                    </div>

                    <div class="g_id_signin"
                        data_type="standard"
                        data-shape="rectangular"
                        data-theme="filled_blue"
                        data-text="$ {button.text}"
                        data-size="large"
                        data-locale="ja"
                        data-logo_alignment="left">
                    </div>

                </div>
            }
        }
        MainRoute::News => html! {<h1>{"News"}</h1>},
        MainRoute::Contact => html! {<h1>{"Contact"}</h1>},
        MainRoute::Login => html! {
            <Switch<LoginRoute> render={Switch::render(switch_login)} />
        },
        MainRoute::Settings => html! {
            <Switch<SettingsRoute> render={Switch::render(switch_settings)} />
        },
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

fn switch_settings(route: &SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<h1>{"Profile"}</h1>},
        SettingsRoute::Friends => html! {<h1>{"Friends"}</h1>},
        SettingsRoute::Theme => html! {<h1>{"Theme"}</h1>},
        SettingsRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        },
    }
}

fn switch_login(route: &LoginRoute) -> Html {
    match route {
        LoginRoute::Callback => html! {<h1>{"Login"}</h1>},
        LoginRoute::State => {
            html!(
                <State />
            )
        }
        LoginRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        },
    }
}

#[function_component(State)]
pub fn state() -> Html {
    let state = use_async_with_options(
        async move { fetch(&format!("{}/login/state", CONFIG.backend_origin)).await },
        UseAsyncOptions::enable_auto(),
    );
    log::info!("{:?}", &state.data);

    html!(
                <div>
                {
                    if state.loading {
                        html! { "Loading" }
                    } else {
                        html! { "end" }
                    }
                }
                {
                    if let Some(data) = &state.data {
                        html! { data }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(error) = &state.error {
                        html! { format!("{:?}",error) }
                    } else {
                        html! {}
                    }
                }
            </div>
    )
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
        <Header />

            <Switch<MainRoute> render={Switch::render(switch_main)} />
        </BrowserRouter>
    }
}

async fn fetch(url: &str) -> Result<String, JsValue> {
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

fn main() {
    let config = wasm_logger::Config::new(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    });
    wasm_logger::init(config);
    yew::start_app::<App>();
}
