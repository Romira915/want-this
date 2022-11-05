use yew::{function_component, html, Callback};
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{route::main::MainRoute, CONFIG};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="text-light-text dark:text-dark-text bg-light-content-background dark:bg-dark-content-background body-font">
            <div class="container sm:mx-0 mx-auto flex flex-wrap p-5 flex-col sm:flex-row items-center">
                <svg class="w-6 h-6 m-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"></path>
                </svg>
                <Link<MainRoute> to={MainRoute::Home}>
                    <span class="flex title-font font-medium items-center text-lightf-text dark:text-dark-text sm:ml-4">
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
                        class="inline-flex items-center dark:bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-light-secondary dark:hover:bg-dark-secondary rounded text-base mt-4 sm:mt-0">
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
