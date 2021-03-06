use derive_more::Constructor;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub mod bindings;
pub mod component;

#[derive(Debug, Deserialize, Constructor)]
pub struct Config {
    pub backend_origin: &'static str,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::new(if cfg!(debug_assertions) {
        "http://localhost:4080"
    } else {
        "https://api.want-this.romira.dev"
    })
});
