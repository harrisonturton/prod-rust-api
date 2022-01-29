use config::{Config, ConfigError, File};
use serde::Deserialize;

pub static CONFIG_FILE: &str = "config";

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
    settings.merge(File::with_name(CONFIG_FILE))?;
    settings.try_into()
}

pub fn get_database_connection_string(db: &DatabaseSettings) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        db.username, db.password, db.host, db.port, db.database
    )
}
