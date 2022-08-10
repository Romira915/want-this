use yew::{function_component, html, Html};

use crate::CONFIG;

#[function_component(LoginButton)]
pub fn login_button() -> Html {
    html!(
        <>
            <script src="https://accounts.google.com/gsi/client"></script>

            <div id="g_id_onload"
                data-client_id="839980808596-tq6nkmcik0nrohr079rj4vt5bdhvr15g.apps.googleusercontent.com"
                data-context="signup"
                data-ux_mode="popup"
                data-login_uri={format!("{}/auth",CONFIG.backend_origin)}
                data-auto_prompt="false">
            </div>

            <div class="g_id_signin"
                data_type="standard"
                data-shape="rectangular"
                data-theme="filled_blue"
                data-text="$ {button.text}"
                data-size="large"
                data-locale="ja"
                data-logo_alignment="left">
            </div>
        </>
    )
}
