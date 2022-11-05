use yew::{function_component, html, use_context, use_state, Callback};

#[function_component(SideMenu)]
pub(crate) fn side_menu() -> Html {
    let is_hidden = use_state(|| false);

    let onclick = {
        let is_hidden = is_hidden.clone();
        Callback::from(move |_| {
            is_hidden.set(true);
        })
    };

    let (hidden_class_name, anime_class_name) = if *is_hidden {
        ("hidden", "animate-slide-out-left")
    } else {
        ("", "")
    };

    html!(
        <div>
            <div class={format!("{anime_class_name} fixed dark:bg-red-500 z-50 top-0 -left-0 w-72 h-full", anime_class_name=anime_class_name)}>
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
            <div onclick={onclick} class={format!("{hidden_class_name} fixed z-40 top-0 left-0 w-full h-full bg-black opacity-50", hidden_class_name=hidden_class_name)} />
        </div>
    )
}
