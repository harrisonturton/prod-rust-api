use backend::config;
use backend::start::start;
use env_logger::{Builder as LoggerBuilder, Env};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> io::Result<()> {
    LoggerBuilder::from_env(Env::default().default_filter_or("info")).init();
    let config = config::get_config().expect("failed to read config.");

    let postgres_uri = config::get_database_uri(&config.database);
    let db_pool = PgPool::connect(&postgres_uri)
        .await
        .expect("failed to connect to postgres.");

    // Start serving requests
    let addr = format!("{}:{}", config.server.host, config.server.port);
    log::info!("Serving on {}", addr);
    let listener = TcpListener::bind(addr)?;
    start(config, listener, db_pool)?.await
}
