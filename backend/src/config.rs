use std::error::Error;
use std::fs;

use sqlx::PgPool;
use crate::user::User;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: PgPool,
    pub user: User,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct LoginConfig {
    pub user: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DBConfig {
    pub server_adress: String,
    pub database_alias: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct LogConfig {
    pub enabled: bool,
    pub path: String,
    pub debug: Option<bool>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ApiConfig {
    // pub prefix: String,
    pub url_directory: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub db: DBConfig,
    pub login: LoginConfig,
    pub log: LogConfig,
    pub api: ApiConfig,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let config: Config = toml::from_str(&fs::read_to_string("Config.toml")?).expect("Loading config failed!");
        Ok(config)
    }
}
