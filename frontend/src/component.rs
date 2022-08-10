use yew::{function_component, html};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{api::fetch, CONFIG};

pub mod cta;
pub mod header;
pub mod login;

#[function_component(State)]
pub(crate) fn state() -> Html {
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
