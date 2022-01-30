use crate::services;
use crate::settings::Settings;
use actix_session::CookieSession;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

use crate::util::middleware::CheckLogin;

const SESSION_ID_COOKIE_NAME: &str = "app_session_id";

pub fn start(
    settings: Settings,
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                CookieSession::signed(&[0; 32])
                    .name(SESSION_ID_COOKIE_NAME)
                    .secure(false),
            )
            .wrap(CheckLogin)
            // `PgPool` is threadsafe and cheap to clone.
            .app_data(db_pool.clone())
            .configure(services::health::configure(db_pool.clone()))
            .configure(services::user::configure(db_pool.clone()))
            .configure(services::auth::configure(
                settings.auth.clone(),
                db_pool.clone(),
            ))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
