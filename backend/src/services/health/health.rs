use actix_web::web::{scope, ServiceConfig};
use actix_web::{get, http::StatusCode, HttpResponseBuilder, Responder};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/health").configure(attach_routes));
}

fn attach_routes(cfg: &mut ServiceConfig) {
    cfg.service(ping_healthcheck);
    cfg.service(database_healthcheck);
}

// This endpoint returns a 200 OK always, used to check if the server is alive.
#[get("/")]
async fn ping_healthcheck() -> impl Responder {
    "Ok"
}

// This endpoint returns a 200 OK upon successful connection to the database.
#[get("/db")]
async fn database_healthcheck() -> impl Responder {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    HttpResponseBuilder::new(status).body("Could not connect to database")
}
