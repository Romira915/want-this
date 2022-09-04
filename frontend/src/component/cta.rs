use crate::component::login::LoginButton;
use yew::{function_component, html};

#[function_component(Cta)]
pub fn cta() -> Html {
    html!(
    <section class="dark:text-gray-600 body-font">
        <div class="container px-5 py-24 mx-auto">
            <div class="lg:w-2/3 flex flex-col sm:flex-row sm:items-center items-start mx-auto">
                <h1 class="flex-grow sm:pr-16 text-2xl font-medium title-font text-gray-900">{"Welcome to Want This"}</h1>
                <LoginButton />
            </div>
        </div>
    </section>
    )
}
