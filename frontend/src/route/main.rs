use crate::component::{cta::Cta, team::*};
use yew::prelude::*;
use yew_router::prelude::*;

use super::{
    login::{switch_login, LoginRoute},
    settings::{switch_settings, SettingsRoute},
};

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum MainRoute {
    #[at("/")]
    Home,
    #[at("/friend")]
    Friend,
    #[at("team")]
    Team,
    #[at("/mypage")]
    MyPage,
    #[at("/settings/:s")]
    Settings,
    #[at("/login/:s")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn switch_main(route: &MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {
           <Cta />
        },
        MainRoute::Friend => html! {
            <h1> {"Frend"} </h1>
        },
        MainRoute::Team => html! {
            <TeamContent />
            // <TeamList />
        },
        MainRoute::MyPage => html! {
            <h1> {"MyPage"} </h1>
        },
        MainRoute::Login => html! {
            <Switch<LoginRoute> render={Switch::render(switch_login)} />
        },
        MainRoute::Settings => html! {
            <Switch<SettingsRoute> render={Switch::render(switch_settings)} />
        },
        MainRoute::NotFound => html! {<h1 class="flex justify-center">{"404 Not Found"}</h1>},
    }
}
