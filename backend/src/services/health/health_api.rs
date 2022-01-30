use actix_session::Session;
use actix_web::web::{scope, Data, HttpResponse, ServiceConfig};
use actix_web::{get, http::StatusCode, Responder};
use sqlx::PgPool;

pub fn configure(pool: PgPool) -> impl Fn(&mut ServiceConfig) {
    move |cfg| {
        let db = Data::new(pool.clone());
        cfg.service(scope("/health").app_data(db).configure(attach_routes));
    }
}

fn attach_routes(cfg: &mut ServiceConfig) {
    cfg.service(ping_healthcheck);
    cfg.service(database_healthcheck);
    cfg.service(session_healthcheck);
}

// This endpoint returns a 200 OK always, used to check if the server is alive.
#[get("/")]
async fn ping_healthcheck() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
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

#[get("/session")]
async fn session_healthcheck(session: Session) -> Option<impl Responder> {
    let count = if let Some(count) = session.get::<i32>("counter").ok()? {
        session.insert("counter", count + 1).ok()?;
        count + 1
    } else {
        session.insert("counter", 1).ok()?;
        1
    };
    Some(format!("counter: {}", count))
}
