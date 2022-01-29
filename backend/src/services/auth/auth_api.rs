use super::auth_service;
use actix_web::web::{scope, Data, Json, ServiceConfig};
use actix_web::post;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(routes));
}

fn routes(cfg: &mut ServiceConfig) {
    cfg.service(sign_in);
    cfg.service(sign_out);
}

// Don't derive `Debug` to make it harder to log passwords.
#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

// Don't derive `Debug` to make it harder to log SAT tokens.
#[derive(Serialize)]
pub struct SignInResponse {
    pub token: String,
}

#[post("/sign_in")]
async fn sign_in(db: Data<PgPool>, req: Json<SignInRequest>) -> Option<Json<SignInResponse>> {
    let res = auth_service::sign_in(db.get_ref(), req.into_inner()).await?;
    Some(Json(res))
}

// Don't derive `Debug` to make it harder to log passwords.
#[derive(Deserialize)]
pub struct SignOutRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct SignOutResponse;

#[post("/sign_out")]
async fn sign_out(db: Data<PgPool>, req: Json<SignOutRequest>) -> Option<Json<SignOutResponse>> {
    let res = auth_service::sign_out(db.get_ref(), req.into_inner()).await?;
    Some(Json(res))
}
