use crate::component::modal::*;
use api_format::Organization;
use reqwasm::http::{Method, Request};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlButtonElement, Window};
use yew::{
    function_component, html, use_node_ref, use_state, Callback, Html, NodeRef, Properties,
    UseStateHandle,
};
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::{
    api::{fetch, Error, RequestBuild},
    CONFIG,
};

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) org: Organization,
    pub(crate) toggle_ref: NodeRef,
}

#[function_component(TeamContent)]
pub(crate) fn team_content() -> Html {
    let handle = use_async_with_options(
        async move {
            fetch::<Vec<Organization>>(&format!("{}/organizations", CONFIG.backend_origin)).await
        },
        UseAsyncOptions::enable_auto(),
    );
    let toggle_ref = use_node_ref();

    let orgs = if let Some(orgs) = handle.data.clone() {
        orgs
    } else if let Some(e) = &handle.error {
        log::debug!("Handle error {}", e);
        let loc = window().unwrap().location();

        if let Some(url) = e.location() {
            loc.set_href(&url).unwrap();
        } else {
            loc.set_href("/").unwrap();
        }
        return html!();
    } else {
        Vec::new()
    };
    log::debug!("orgs {:?}", orgs);

    html!(
        <div class="dark:bg-gray-700 dark:text-gray-300">
            <div class="flex flex-col">
                {for orgs.iter().map(|o| html!{<Team org={o.clone()} toggle_ref={toggle_ref.clone()} />})}
            </div>
            <button ref={toggle_ref} id={"org-create-modal-toggle"} hidden={true} data-modal-toggle="notice-modal">{"toggle"}</button>
            <Modal message={"OK!".to_string()} modal_id={"notice-modal"} />
        </div>
    )
}

#[function_component(Team)]
pub(crate) fn team(props: &Props) -> Html {
    let org_id = use_state(|| props.org.organization_id.clone());
    let disabled = use_state(|| false);
    let handle = use_async(async move {
        Request::build(&format!(
            "{}/organizations/{}",
            CONFIG.backend_origin, *org_id
        ))
        .method(Method::POST)
        .send()
        .await
        .map(|r| r.ok())
        .map_err(|e| Error::new(e.to_string(), None))
    });
    let onclick = {
        let disabled = disabled.clone();
        let handle = handle.clone();
        Callback::from(move |_| {
            disabled.set(true);
            handle.run();
        })
    };

    if let Some(true) = handle.data {
        let toggle_ref = props.toggle_ref.clone();
        if let Some(toggle) = toggle_ref.cast::<HtmlButtonElement>() {
            toggle.click();
        }
    }

    html!(
        <div class="grid grid-cols-4 justify-items-center">
            <div class="text-white text-2xl">{props.org.organization_name.as_str()}</div>
            <div>{props.org.description.clone().unwrap_or_default()}</div>
            <div>{&props.org.owner}</div>
            <button
             onclick={onclick}
             disabled={*disabled}
             class="flex-shrink-0 text-white dark:bg-indigo-700 disabled:bg-gray-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg mt-10 sm:mt-0">{"Join"}</button>
        </div>
    )
}
