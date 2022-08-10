use crate::component::State;
use yew::{prelude::*};
use yew_router::prelude::*;

use super::main::MainRoute;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum LoginRoute {
    #[at("/login/callback")]
    Callback,
    #[at("/login/state")]
    State,
    #[not_found]
    #[at("/login/404")]
    NotFound,
}

pub(crate) fn switch_login(route: &LoginRoute) -> Html {
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
