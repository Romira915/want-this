use yew::{function_component, html, use_context, use_state, Callback, Properties, UseStateHandle};

use crate::SideMenuState;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) side_menu_state: UseStateHandle<SideMenuState>,
}

#[function_component(SideMenu)]
pub(crate) fn side_menu(props: &Props) -> Html {
    let onclick_content_aria = {
        let side_menu_state = props.side_menu_state.clone();
        Callback::from(move |_| {
            side_menu_state.set(SideMenuState::Close);
        })
    };

    let (hidden_class_name, animate_class_name) = match *props.side_menu_state {
        // NOTE: コンテンツエリアを隠さず，左平行移動
        SideMenuState::Init => ("hidden", "-translate-x-full"),
        // NOTE: コンテンツエリアを隠して，スライドインアニメーションを再生
        SideMenuState::Open => ("", "animate-slide-in-left"),
        // NOTE: コンテンツエリアを隠さず，スライドアウトアニメーションを再生
        SideMenuState::Close => ("hidden", "animate-slide-out-left"),
    };

    html!(
        <div>
            <div class={format!("{animate_class_name} fixed dark:bg-red-500 z-50 top-0 w-80 h-full", animate_class_name=animate_class_name)}>
                <p>{"サイドメニュー"}</p>
                <ul>
                    <li class="contents1">{"メニュー1"}</li>
                    <li class="contents2">{"メニュー2"}</li>
                    <li class="has-child-menu">{"メニュー3"}
                        <ul>
                            <li class="contents3-1">{"メニュー3-1"}</li>
                            <li class="contents3-2">{"メニュー3-2"}</li>
                            <li class="contents3-3">{"メニュー3-3"}</li>
                        </ul>
                    </li>
                </ul>
                <div class="side-menu-button">{"サイドメニュー"}</div>
            </div>
            <div onclick={onclick_content_aria} class={format!("{hidden_class_name} fixed z-40 top-0 left-0 w-full h-full bg-black opacity-50", hidden_class_name=hidden_class_name)} />
        </div>
    )
}
