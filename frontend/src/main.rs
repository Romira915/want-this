use want_this_frontend::App;

fn main() {
    let config = wasm_logger::Config::new(if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    });
    wasm_logger::init(config);
    yew::start_app::<App>();
}
