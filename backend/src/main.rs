use std::io;
use std::net::TcpListener;
use backend::settings;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = settings::get_config().expect("failed to read config.");
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(addr)?;
    backend::start(listener)?.await
}
