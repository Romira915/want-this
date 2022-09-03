use yew::prelude::*;
use yew_router::prelude::*;

use super::main::MainRoute;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum SettingsRoute {
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

pub(crate) fn switch_settings(route: &SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<h1>{"Profile"}</h1>},
        SettingsRoute::Friends => html! {<h1>{"Friends"}</h1>},
        SettingsRoute::Theme => html! {<h1>{"Theme"}</h1>},
        SettingsRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        },
    }
}
