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

macro_rules! create_api {
    (
        $(
            ($route:expr, $service_data:expr, $route_config:expr),
        )+
    ) => {
        App::new()
            $(
                .app_data(Data::new($service_data))
                .service(scope($route).configure($route_config))
            )+
    };
}

pub fn start(
    config: Config,
    listener: TcpListener,
    // `PgPool` is threadsafe and cheap to clone.
    db_pool: PgPool,
) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        let user_service = user::UserService::new(db_pool.clone());
        let auth_service =
            auth::AuthService::new(config.auth.clone(), db_pool.clone(), user_service.clone());

        let app = create_api! {
            ("/auth", auth_service, auth::routes),
            ("/user", user_service, user::routes),
            ("/health", db_pool.clone(), health::routes),
        };

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

        app.wrap(Logger::default())
            .app_data(Data::new(config.clone()))
            .wrap(error_handlers)
            .wrap(cors)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
