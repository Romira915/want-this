use json_format::User;
use yew::{function_component, html};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{api::fetch, CONFIG};

pub mod cta;
pub mod header;
pub mod login;

#[function_component(State)]
pub(crate) fn state() -> Html {
    let handle = use_async_with_options(
        async move { fetch::<User>(&format!("{}/login/state", CONFIG.backend_origin)).await },
        UseAsyncOptions::enable_auto(),
    );
    log::debug!("{:?}", &handle.data);

    let display = if handle.loading {
        "Loading".to_string()
    } else if let Some(user) = &handle.data {
        user.google_id.clone().unwrap_or_else(|| "None".to_string())
    } else if let Some(e) = &handle.error {
        e.to_string()
    } else {
        String::new()
    };

    html!(
        <div>
            {
               display
            }
        </div>
    )
}
