use yew::prelude::*;

mod bindings;

use yew::prelude::*;
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
    let onclick = Callback::from(move |_| {
        bindings::send();
    });

    match route {
        MainRoute::Home => {
            html! {
                <div>
                    <Link<MainRoute> to={MainRoute::News}>{ "click here to go home" }</Link<MainRoute>>
                    <script src="https://accounts.google.com/gsi/client"></script>

                    <div id="g_id_onload"
                      data-client_id="839980808596-tq6nkmcik0nrohr079rj4vt5bdhvr15g.apps.googleusercontent.com"
                      data-login_uri="http://localhost:8080/login/callback"
                      data-auto_prompt="false">
                      </div>
                      <div class="g_id_signin"
                      data_type="standard"
                      data-size="large"
                      data-theme="outline"
                      data-text="sign_in_with"
                      data-shape="rectangular"
                      data-logo_alignment="left">
                      </div>
                      <button onclick={onclick}>{"Authorize with Google"}</button>
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
        LoginRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <script src="https://accounts.google.com/gsi/client"></script>

            <Switch<MainRoute> render={Switch::render(switch_main)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
