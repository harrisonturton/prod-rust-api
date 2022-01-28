use serde::Deserialize;
use config::{Config, File, ConfigError};

#[derive(Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let mut settings = Config::default();
    settings.merge(File::with_name("config"))?;
    settings.try_into()
}