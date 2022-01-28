pub mod settings;
pub mod auth;
pub mod health;
pub mod user;

use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::io;
use std::net::TcpListener;

pub fn start(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .configure(health::configure)
            .configure(user::configure)
            .configure(auth::configure)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
