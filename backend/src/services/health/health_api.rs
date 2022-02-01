use crate::util::request::{Identity, RequestContext};
use actix_web::web::{Data, HttpResponse, ServiceConfig};
use actix_web::{get, http::StatusCode, Responder};
use sqlx::PgPool;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(ping_healthcheck);
    cfg.service(session_healthcheck);
    cfg.service(database_healthcheck);
}

// This endpoint returns a 200 OK always, used to check if the server is alive.
#[get("/")]
async fn ping_healthcheck() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
}

// This checks that we are correctly parsing the RequestContext
#[get("/session")]
async fn session_healthcheck(ctx: RequestContext) -> impl Responder {
    match ctx.identity {
        Identity::Anon => String::from("Anonymous"),
        Identity::User(_) => String::from("Session cookie"),
        Identity::Service(service) => format!("{:?}", service),
    }
}

// This endpoint returns a 200 OK upon successful connection to the database.
#[get("/db")]
async fn database_healthcheck(db: Data<PgPool>) -> impl Responder {
    let query = r#"
        SELECT 'ok'
    "#;
    let res: Result<(String,), sqlx::Error> = sqlx::query_as(query).fetch_one(db.get_ref()).await;
    match res {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
