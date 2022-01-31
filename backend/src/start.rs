use crate::services::{auth, health, user};
use crate::config::Config;
use crate::util::http;
use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

use crate::util::middleware::CheckLogin;

pub fn start(
    config: Config,
    listener: TcpListener,
    // Note: `PgPool` is threadsafe and cheap to clone.
    db_pool: PgPool,
) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        // Middleware
        let user_service = user::UserService::new(db_pool.clone());
        let auth_service =
            auth::AuthService::new(config.auth.clone(), db_pool.clone(), user_service);
        let auth_middleware = CheckLogin { auth_service };

        // Service constructors
        let user_service = user::configure(db_pool.clone(), auth_middleware.clone());
        let auth_service = auth::configure(db_pool.clone(), config.auth.clone());
        let health_service = health::configure(db_pool.clone());

        // By default, actix returns the message of `Json` deserialzation errors
        // directly to the client. This catches those errors and returns a
        // `ServiceError::bad_request()` instead, like all our other endpoints.
        let error_handlers =
            ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, http::handle_bad_request);

        App::new()
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .service(scope("/auth").configure(auth_service))
            .service(scope("/health").configure(health_service))
            .service(scope("/user").configure(user_service))
            .wrap(error_handlers)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
