use api_format::{Organization, User};
use yew::{function_component, html};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    api::{fetch, put_json},
    CONFIG,
};

pub(crate) mod cta;
pub(crate) mod header;
pub(crate) mod login;
pub(crate) mod modal;
pub(crate) mod side_menu;
pub(crate) mod team;

enum Message {
    ToggleSideMenu(bool),
}

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
        format!("{:?}", user)
        // user.google_id.clone().unwrap_or_else(|| "None".to_string())
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

#[function_component(DebugComponent)]
pub(crate) fn debug_component() -> Html {
    let handle = use_async_with_options(
        async move {
            let org = Organization::new(
                0.to_string(),
                "test".to_string(),
                Some("desc".to_string()),
                1,
                0.to_string(),
            );
            put_json(&format!("{}/organizations/0", CONFIG.backend_origin), org).await
        },
        UseAsyncOptions::enable_auto(),
    );

    let display = if handle.loading {
        "Loading".to_string()
    } else if let Some(_) = &handle.data {
        "on put".to_string()
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
