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
    let common_class = "rounded-md py-1 px-3 border-2 
                              bg-light-button-bg dark:bg-dark-button-bg  
                              border-light-button-border dark:border-dark-button-border";
    let general_class = "hover:border-light-button-border-hover dark:hover:border-dark-button-border-hover 
                               hover:bg-light-button-bg-hover 
                               active:bg-light-button-bg-active dark:active:bg-dark-button-bg-active";
    // TODO: 使う時がきたら実装する
    let primary_class = "";
    let secondary_class = "text-light-secondary dark:text-dark-secondary 
                                 hover:border-light-secondary dark:hover:border-dark-secondary 
                                 hover:bg-light-secondary dark:hover:bg-dark-secondary 
                                 hover:text-light-button-bg dark:hover:text-dark-text 
                                 active:bg-light-secondary-deep dark:active:bg-dark-secondary-deep";

    let class = match props.color_type {
        ButtonColorType::General => format!("{} {}", common_class, general_class),
        ButtonColorType::Primary => format!("{} {}", common_class, primary_class),
        ButtonColorType::Secondary => format!("{} {}", common_class, secondary_class),
    };

    html!(
        <button class={class}>
            {
                for props.children.iter()
            }
        </button>
    )
}
