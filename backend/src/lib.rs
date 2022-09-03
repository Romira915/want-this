use once_cell::sync::Lazy;
use serde::Deserialize;

pub mod auth;
pub mod domain;
pub mod infrastructure;
pub mod media;
pub mod session;
pub(crate) mod utility;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mariadb_database: String,
    pub mariadb_user: String,
    pub mariadb_password: String,
    pub mariadb_address: String,
    pub mariadb_port: u32,
    // Debug用
    pub database_url: String,
    pub redis_url: String,
    pub frontend_origin: String,
}

impl Config {
    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.mariadb_user,
            self.mariadb_password,
            self.mariadb_address,
            self.mariadb_port,
            self.mariadb_database
        )
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv::dotenv().unwrap();
    envy::from_env().unwrap()
});
