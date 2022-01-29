use crate::services;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

pub fn start(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // `PgPool` is threadsafe and cheap to clone.
            .app_data(db_pool.clone())
            .configure(services::health::configure(db_pool.clone()))
            .configure(services::user::configure)
            .configure(services::auth::configure(db_pool.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
