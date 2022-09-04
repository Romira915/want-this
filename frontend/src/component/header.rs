use yew::{function_component, html, Callback};
use yew_hooks::use_async;

use crate::{api::get, CONFIG};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="text-gray-400 dark:bg-gray-900 body-font">
            <div class="container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center">
                <a href="/" class="flex title-font font-medium items-center text-white mb-4 md:mb-0">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" stroke-linecap="round"
                        stroke-linejoin="round" stroke-width="2" class="w-10 h-10 text-white p-2 bg-indigo-500 rounded-full"
                        viewBox="0 0 24 24">
                        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>
                    </svg>
                    <span class="ml-3 text-xl" href="/">{"Want This"}</span>
                </a>
                <nav class="md:ml-auto flex flex-wrap items-center text-base justify-center">
                    <a href="/" class="mr-5 hover:text-white" >{"Home"}</a>
                    <a href="/friend" class="mr-5 hover:text-white">{"Friend"}</a>
                    <a href="/team" class="mr-5 hover:text-white">{"Team"}</a>
                    <a href="/mypage" class="mr-5 hover:text-white">{"MyPage"}</a>
                </nav>
                <a
                    href={format!("{}/auth/logout", CONFIG.backend_origin)}
                    class="inline-flex items-center dark:bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0">{"Logout"}
                    <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        class="w-4 h-4 ml-1" viewBox="0 0 24 24">
                        <path d="M5 12h14M12 5l7 7-7 7"></path>
                    </svg>
                </a>
            </div>
        </header>
    }
}
