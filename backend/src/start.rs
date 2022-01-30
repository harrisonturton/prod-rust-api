use futures::FutureExt;
use actix_web::dev::Service;
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
        let user_service = services::user::UserService::new(db_pool.clone());
        let auth_service =
            services::auth::AuthService::new(settings.auth.clone(), db_pool.clone(), user_service);
        App::new()
            .wrap(Logger::default())
            // `PgPool` is threadsafe and cheap to clone.
            .app_data(db_pool.clone())
            .configure(services::user::configure(db_pool.clone(), settings.auth.clone()))
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| {
                    log::info!("3");
                    res
                })
            })
            //.wrap(CheckLogin { auth_service })
            .configure(services::auth::configure(
                settings.auth.clone(),
                db_pool.clone(),
            ))
            // .wrap(
            //     CookieSession::signed(&[0; 32])
            //         .name(SESSION_ID_COOKIE_NAME)
            //         .expires_in(10)
            //         .secure(false),
            // )
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| {
                    log::info!("2");
                    res
                })
            })
            .configure(services::health::configure(db_pool.clone()))
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| {
                    log::info!("1");
                    res
                })
            })
    })
    .listen(listener)?
    .run();
    Ok(server)
}
