use yew::{
    function_component, html, use_context, use_state, Callback, Html, Properties, UseStateHandle,
};
use yew_router::prelude::Link;

use crate::{route::main::MainRoute, SideMenuState};

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub(crate) side_menu_state: UseStateHandle<SideMenuState>,
}

#[function_component(SideMenu)]
pub(crate) fn side_menu(props: &Props) -> Html {
    let side_bar_close_handle = {
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
            // NOTE: サイドメニュー
            <div class={format!("{animate_class_name} fixed z-50 top-0 w-80 h-full bg-light-content-background dark:bg-dark-content-background text-light-text dark:text-dark-text
                                flex flex-col text-2xl items-center",
                                animate_class_name=animate_class_name)}>
                <div class="container flex items-center justify-between p-3 border-b border-b-gray-500">
                    <div class="w-6 h-6" />
                    <Link<MainRoute> to={MainRoute::Home}>
                        <p onclick={side_bar_close_handle.clone()}>{"Want This"}</p>
                    </Link<MainRoute>>
                    <button onclick={side_bar_close_handle.clone()}>
                        <svg
                        class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    </button>
                </div>

                <ul class="my-3">
                    <li>
                        <Link<MainRoute> to={MainRoute::Team}>
                            <span
                            onclick={side_bar_close_handle.clone()}
                            class="hover:text-light-primary dark:hover:text-dark-primary">{"Team"}</span>
                        </Link<MainRoute>>
                    </li>
                </ul>
                <ul class="hover:overflow-auto w-full flex flex-col items-center">
                    // TODO: fetchでチーム情報取得して置き換え
                    {
                        (0..100).map(|i| html!(<li>{format!("Team {}",i)}</li>)).collect::<Html>()
                    }
                </ul>
            </div>
            <div onclick={side_bar_close_handle} class={format!("{hidden_class_name} fixed z-40 top-0 left-0 w-full h-full bg-black opacity-50", hidden_class_name=hidden_class_name)} />
        </div>
    )
}
