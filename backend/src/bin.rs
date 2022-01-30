use backend::settings;
use env_logger::{Builder as LoggerBuilder, Env};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> io::Result<()> {
    LoggerBuilder::from_env(Env::default().default_filter_or("info")).init();
    let settings = settings::get_config().expect("failed to read config.");
    let db_connection_string = settings::get_database_connection_string(&settings.database);
    let conn_pool = PgPool::connect(&db_connection_string)
        .await
        .expect("failed to connect to postgres.");
    let addr = format!("{}:{}", settings.server.host, settings.server.port);
    log::info!("Serving on {}", addr);
    let listener = TcpListener::bind(addr)?;
    backend::start::start(settings, listener, conn_pool)?.await
}
