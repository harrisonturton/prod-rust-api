use crate::services;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;
use std::sync::Arc;

pub fn start(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let pool = Data::new(db_pool.clone());
    let server = HttpServer::new(move || {
        println!("{:?}", db_pool.clone().size());
        App::new()
            .wrap(Logger::default())
            // Each thread gets access to the PgPool and can request connections
            // from it. One connection for each service? So services * threads
            // connections. Assume 15 services and 16 threads on server box...
            // 240 connections. Feels too high? Especially if scale
            // horizontally. There is no reason for services within a single
            // thread to have multiple database connections because they cannot
            // run concurrently. Instead, have one database connection per
            // thread, and share a refernce to this among services. Doesn't need
            // to be threadsafe, since it's within a single thread, so can use
            // Rc.
            // Share a threadsafe reference to the PgPool to configure each service
            .app_data(pool.clone())
            .configure(services::health::configure)
            .configure(services::user::configure)
            .configure(services::auth::configure(db_pool.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
