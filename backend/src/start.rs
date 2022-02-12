use crate::base::http;
use crate::config::Config;
use crate::services::{auth, health, user};
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

macro_rules! create_app {
    (
        $(
            {
                path: $path:expr,
                routes: $routes:expr,
                service: $service:expr,
            },
        )+
    ) => {
        App::new()
            $(
                .app_data(Data::new($service))
                .service(scope($path).configure($routes))
            )+
    };
}

pub fn start(
    config: Config,
    listener: TcpListener,
    // `PgPool` is threadsafe and cheap to clone.
    db: PgPool,
) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        // Bit of a hack to replace dependency injection for now. Should replace
        // this with a better ownership model that still allows services to talk
        // to eachother. Maybe "I want to make a request, request" ...pass
        // message to say "I want to make request to X" instead of references?
        // And have some sort of message bus to get around it the ownership
        // problem.  Would avoid lots of the cloning here, but not sure if
        // that's worthwhile. Cloning from database pool is cheap.
        //
        // A "UserService" could be wrapped by a "UserServiceClient" that
        // implements the same API (maybe use a trait), but passes messages
        // through message bus instead. Client can then be trivially replaced
        // with actual HTTP client later on!
        let app = create_app! {
            {
                path: "/auth",
                routes: auth::routes,
                service: {
                    let user_service = user::UserService::install(&db);
                    let auth_service = auth::AuthService::install(&config.auth, &db, &user_service);
                    auth_service
                },
            },
            {
                path: "/user",
                routes: user::routes,
                service: {
                    let user_service = user::UserService::install(&db);
                    user_service
                },
            },
            {
                path: "/health",
                routes: health::routes,
                service: db.clone(),
            },
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
