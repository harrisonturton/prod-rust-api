pub mod health;

use std::io;
use std::net::TcpListener;

// Starts the API server in the background on a random available port and return
// the address it is being served on.
fn start_test_server() -> io::Result<String> {
    // Port 0 instructs the operating system to find a random, available port.
    let listener = TcpListener::bind("127.0.01:0").expect("Failed to bind to random port");
    let port = listener.local_addr()?.port();
    let server = backend::start(listener)?;
    tokio::spawn(server);
    Ok(format!("http://127.0.0.1:{}", port))
}