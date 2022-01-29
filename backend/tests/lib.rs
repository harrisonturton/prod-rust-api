pub mod services;

use backend::settings;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

// Starts the API server in the background on a random available port and return
// the address it is being served on.
async fn start_test_server() -> io::Result<String> {
    let config = settings::get_config().expect("failed to read config.");
    let db_connection_string = settings::get_database_connection_string(&config.database);
    let conn_pool = PgPool::connect(&db_connection_string)
        .await
        .expect("failed to connect to postgres.");
    let listener = TcpListener::bind("127.0.01:0").expect("Failed to bind to random port");
    let port = listener.local_addr()?.port();
    let server = backend::start::start(listener, conn_pool)?;
    tokio::spawn(server);
    Ok(format!("http://127.0.0.1:{}", port))
}
