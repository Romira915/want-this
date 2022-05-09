use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/hello.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn hello() -> String;

    #[wasm_bindgen]
    pub fn send();
}

#[wasm_bindgen(module = "https://accounts.google.com/gsi/client")]
extern "C" {}
