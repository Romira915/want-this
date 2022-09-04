use json_format::Organization;
use yew::{function_component, html, Html, Properties};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{api::fetch, CONFIG};

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    org: Organization,
}

#[function_component(TeamContent)]
pub(crate) fn team_content() -> Html {
    let handle = use_async_with_options(
        async move {
            fetch::<Vec<Organization>>(&format!("{}/organizations", CONFIG.backend_origin)).await
        },
        UseAsyncOptions::enable_auto(),
    );

    let orgs = if let Some(orgs) = handle.data.clone() {
        orgs
    } else {
        Vec::new()
    };
    log::debug!("orgs {:?}", orgs);

    html!(
        <div class="container mx-auto">
            <div class="flex flex-col ">
                {for orgs.iter().map(|o| html!{<Team org={o.clone()} />})}
            </div>
        </div>
    )
}

#[function_component(Team)]
pub(crate) fn team(props: &Props) -> Html {
    html!(
        <div class="grid grid-cols-3 justify-items-center">
            <div>{props.org.organization_name.as_str()}</div>
            <div>{props.org.description.clone().unwrap_or_default()}</div>
            <div>{props.org.owner}</div>
        </div>
    )
}
