use crate::config::Config;
use crate::services::{auth, health, user};
use crate::util::http;
use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

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
            auth::AuthService::new(config.auth.clone(), db_pool.clone(), user_service.clone());

        // By default, actix returns the message of `Json` deserialzation errors
        // directly to the client. This catches those errors and returns a
        // `ServiceError::bad_request()` instead, like all our other endpoints.
        let error_handlers =
            ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, http::handle_bad_request);

        // Required since when the API and browser are served from different
        // origins when serving locally.
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(config.clone()))
            // User service
            .app_data(Data::new(user_service))
            .service(scope("/user").configure(user::routes))
            // Auth service
            .app_data(Data::new(auth_service))
            .service(scope("/auth").configure(auth::routes))
            // Healthcheck endpoints
            .app_data(Data::new(db_pool.clone()))
            .service(scope("health").configure(health::routes))
            // Other config
            .wrap(error_handlers)
            .wrap(cors)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
