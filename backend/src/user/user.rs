use actix_web::web::{scope, ServiceConfig};
use actix_web::{get, Responder};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/user").service(get_user));
}

#[get("/")]
async fn get_user() -> impl Responder {
    "@harryt"
}
