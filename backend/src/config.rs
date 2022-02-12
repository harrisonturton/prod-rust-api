use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub sat_cookie_name: String,
    pub sat_cookie_lifetime_mins: u32,
}

pub fn from_file(file: &str) -> Result<Config, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(file))?;
    settings.try_into()
}

pub fn get_database_uri(db: &DatabaseConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        db.username, db.password, db.host, db.port, db.database
    )
}
