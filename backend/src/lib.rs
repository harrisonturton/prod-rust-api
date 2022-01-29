pub mod services;
pub mod settings;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

pub fn start(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(services::health::configure)
            .configure(services::user::configure)
            .configure(services::auth::configure)
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
