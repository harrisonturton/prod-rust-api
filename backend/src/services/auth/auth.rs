use actix_web::web::{scope, ServiceConfig};
use actix_web::{get, Responder};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").service(sign_in));
}

#[get("/")]
async fn sign_in() -> impl Responder {
    "Signed in!"
}
