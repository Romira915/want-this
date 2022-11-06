use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::Element;
use yew::{
    function_component, html, use_context, use_effect, use_node_ref, use_state, Callback, Children,
    Html, Properties, UseStateHandle,
};
use yew_router::prelude::Link;

#[derive(Debug, PartialEq)]
pub(crate) enum ButtonColorType {
    General,
    Primary,
    Secondary,
}

impl Default for ButtonColorType {
    fn default() -> Self {
        Self::General
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub color_type: ButtonColorType,
}

#[function_component(Button)]
pub(crate) fn button(props: &Props) -> Html {
    html!(
        <button class="rounded py-1 px-3 bg-light-button-bg dark:bg-dark-button-bg border-2
                border-light-button-border dark:border-dark-button-border 
                hover:border-light-button-border-hover dark:hover:border-dark-button-border-hover 
                hover:bg-light-button-bg-hover 
                active:bg-light-button-bg-active dark:active:bg-dark-button-bg-active">
            {
                for props.children.iter()
            }
        </button>
    )
}
