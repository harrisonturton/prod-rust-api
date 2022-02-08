use crate::config;
use crate::start::start;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

// Starts the API server in the background on a random available port and return
// the address it is being served on.
pub async fn start_test_server() -> io::Result<String> {
    let config = config::get_config().expect("failed to read config.");
    let database_uri = config::get_database_uri(&config.database);
    let conn_pool = PgPool::connect(&database_uri)
        .await
        .expect("failed to connect to postgres.");

    let listener = TcpListener::bind("127.0.01:0").expect("Failed to bind to random port");
    let port = listener.local_addr()?.port();
    let server = start(config, listener, conn_pool)?;
    tokio::spawn(server);
    Ok(format!("http://127.0.0.1:{}", port))
}
