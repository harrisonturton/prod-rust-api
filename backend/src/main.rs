use backend::config;
use backend::start::start;
use env_logger::{Builder as LoggerBuilder, Env};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> io::Result<()> {
    LoggerBuilder::from_env(Env::default().default_filter_or("info")).init();
    let config = get_config(); // Will panic if not found

    let db_uri = config::get_database_uri(&config.database);
    let db_pool = match PgPool::connect(&db_uri).await {
        Ok(pool) => pool,
        Err(err) => {
            log::error!(
                "Failed to connect to database with URI {} with error: {}",
                db_uri,
                err
            );
            panic!("Cannot continue without a database connection")
        }
    };

    let addr = format!("{}:{}", config.server.host, config.server.port);
    log::info!("Serving on {}", addr);
    let listener = TcpListener::bind(addr)?;
    start(config, listener, db_pool)?.await
}

// If release build
#[cfg(not(debug_assertions))]
fn get_config() -> config::Config {
    let path = "resources/config.prod.toml";
    match config::from_file(&path) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to read prod config from {}: {:?}", path, err);
            panic!("Cannot continue without config")
        }
    }
}

// If development build
#[cfg(debug_assertions)]
fn get_config() -> config::Config {
    let path = "resources/config.local.toml";
    match config::from_file(&path) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to read local config from {}: {:?}", path, err);
            panic!("Cannot continue without config")
        }
    }
}
