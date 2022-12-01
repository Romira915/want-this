use yew::{
    function_component, html, html::Scope, use_context, use_reducer, Callback, Properties,
    UseStateHandle,
};
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{route::main::MainRoute, SideMenuState, CONFIG};

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) side_menu_state: UseStateHandle<SideMenuState>,
}

#[function_component(Header)]
pub(crate) fn header(props: &Props) -> Html {
    let onclick_side_menu_icon = {
        let side_menu_state = props.side_menu_state.clone();
        Callback::from(move |_| {
            side_menu_state.set(SideMenuState::Open);
        })
    };

    html! {
        <header class="text-light-text dark:text-dark-text bg-light-content-background dark:bg-dark-content-background body-font">
            <div class="container sm:mx-0 mx-auto flex flex-wrap p-5 flex-row items-center justify-between">
                <button onclick={onclick_side_menu_icon} class="hover:bg-light-roundbutton-bg-hover dark:hover:bg-dark-roundbutton-bg-hover rounded-full p-1">
                    <svg class="w-6 h-6 m-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"></path>
                    </svg>
                </button>
                <Link<MainRoute> to={MainRoute::Home}>
                    <span class="flex title-font font-medium items-center text-light-text dark:text-dark-text sm:ml-4">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" stroke-linecap="round"
                        stroke-linejoin="round" stroke-width="2" class="w-10 h-10 text-white p-2 bg-indigo-500 rounded-full"
                        viewBox="0 0 24 24">
                                <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>
                        </svg>

                        <span class="hidden sm:block ml-3 text-xl" href="/">{"Want This"}</span>
                    </span>
                </Link<MainRoute>>
                <nav class="sm:ml-auto flex flex-wrap items-center text-base justify-center">
                    <Link<MainRoute> to={MainRoute::Home}>
                        <span class="hidden sm:block mr-5 hover:text-light-primary dark:hover:text-dark-primary">{"Home"}</span>
                    </Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Friend}>
                        <span class="hidden sm:block mr-5 hover:text-light-primary dark:hover:text-dark-primary">{"Friend"}</span>
                    </Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Team}>
                        <span class="hidden sm:block mr-5 hover:text-light-primary dark:hover:text-dark-primary">{"Team"}</span>
                    </Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::MyPage}>
                        <img src="https://avatars.githubusercontent.com/u/40430090?s=40&amp;v=4" alt="" size="20" height="20" width="20" data-view-component="true"
                        class="sm:mr-5 hover:text-light-primary dark:hover:text-dark-primary" />
                        // <span class="mr-5 hover:text-light-primary dark:hover:text-dark-primary">{"MyPage"}</span>
                    </Link<MainRoute>>
                    <span class="hidden sm:block">
                        <a href={format!("{}/auth/logout", CONFIG.backend_origin)}
                        class="inline-flex items-center bg-light-button-bg dark:bg-dark-button-bg border-0 py-1 px-3 focus:outline-none hover:bg-light-secondary dark:hover:bg-dark-secondary rounded text-base mt-4 sm:mt-0">
                            <span>{"Logout"}</span>
                            <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            class="w-4 h-4 ml-1" viewBox="0 0 24 24">
                            <path d="M5 12h14M12 5l7 7-7 7"></path>
                            </svg>
                        </a>
                    </span>
                </nav>
            </div>
        </header>
    }
}
