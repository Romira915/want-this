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

#[function_component(DraftTeamContent)]
pub(crate) fn draft_team_content() -> Html {
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
                {for orgs.iter().map(|o| html!{<DraftTeam org={o.clone()} toggle_ref={toggle_ref.clone()} />})}
            </div>
            <button ref={toggle_ref} id={"org-create-modal-toggle"} hidden={true} data-modal-toggle="notice-modal">{"toggle"}</button>
            <Modal message={"参加申請しました!".to_string()} modal_id={"notice-modal"} />
        </div>
    )
}

#[function_component(DraftTeam)]
pub(crate) fn draft_team(props: &Props) -> Html {
    let org_id = use_state(|| props.org.organization_id.clone());
    let disabled = use_state(|| false);
    let handle = use_async(async move {
        Request::build(&format!(
            "{}/organizations/{}/join_request",
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
        <div class="grid grid-cols-4 justify-items-center my-1">
            <div class="text-white text-2xl">{props.org.organization_name.as_str()}</div>
            <div>{props.org.description.clone().unwrap_or_default()}</div>
            <div>{&props.org.owner}</div>
            <button
             onclick={onclick}
             disabled={*disabled}
             class="flex-shrink-0 text-white dark:bg-indigo-700 disabled:bg-gray-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg mt-10 sm:mt-0">
             {"Join request"}
             </button>
        </div>
    )
}

#[function_component(TeamContent)]
pub(crate) fn team_content() -> Html {
    html!(
        <div class="container mx-auto px-5 py-4 my-5 bg-light-content-background dark:bg-dark-content-background text-light-text dark:text-dark-text max-w-xl">
            <div class="container flex">
                <h2 class="text-2xl">
                    {"My Team"}
                </h2>
                <button class="ml-auto rounded py-1 px-3 bg-light-button-bg dark:bg-dark-button-bg hover:bg-light-primary dark:hover:border-dark-content-border">
                    {"New Team"}
                </button>
            </div>
            <TeamList />
        </div>
    )
}

#[function_component(TeamList)]
pub(crate) fn team_list() -> Html {
    html!(
        <ul class="border-2 border-light-content-border dark:border-dark-content-border px-2 py-1 my-3 bg-light-content-background dark:bg-dark-content-background text-light-text dark:text-dark-text">
            {
                (0..10).map(|_| html!(<TeamElement />)).collect::<Html>()
            }
        </ul>
    )
}

#[function_component(TeamElement)]
pub(crate) fn team_element() -> Html {
    html!(
        <li class="flex flex-nowrap my-1 items-center">
                <div class="grow text-link-text hover:underline hover:underline-offset-2">{"Romira-Team"}</div>
                // TODO: Buttonを共通化する
                <div class="ml-4 rounded py-1 px-3 bg-light-button-bg dark:bg-dark-button-bg border-2
                border-light-button-border dark:border-dark-button-border 
                hover:border-light-button-border-hover dark:hover:border-dark-button-border-hover 
                hover:bg-light-button-bg-hover 
                active:bg-light-button-bg-active dark:active:bg-dark-button-bg-active">{"Request"}</div>
                <div class="ml-4 rounded py-1 px-3 bg-light-button-bg dark:bg-dark-button-bg">{"Settings"}</div>
                <div class="ml-4 text-light-secondary dark:text-dark-secondary">{"Leave"}</div>
        </li>
    )
}
