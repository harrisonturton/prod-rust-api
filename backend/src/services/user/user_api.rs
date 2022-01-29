//! Endpoints that expose the user service to the internet. These methods should
//! do the absolute minimum amount of logic; they exist only as a translation
//! layer to pipe requests from `actix_web` to the user service. If we were to
//! change actix for something else, that should be possible simply by replacing
//! this file.

use super::user_service;
use super::User;
use actix_web::post;
use actix_web::web::{scope, Data, Json, ServiceConfig};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/user").configure(routes));
}

fn routes(cfg: &mut ServiceConfig) {
    cfg.service(find_user);
    cfg.service(create_user);
}

#[derive(Debug, Deserialize)]
pub struct FindUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct FindUserResponse {
    pub user: User,
}

#[post("/")]
async fn find_user(req: Json<FindUserRequest>, db: Data<PgPool>) -> Option<Json<FindUserResponse>> {
    let res = user_service::find_user(db.get_ref(), req.into_inner()).await?;
    Some(Json(res))
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user: User,
}

#[post("/new")]
async fn create_user(
    req: Json<CreateUserRequest>,
    db: Data<PgPool>,
) -> Option<Json<CreateUserResponse>> {
    let res = user_service::create_user(db.get_ref(), req.into_inner()).await?;
    Some(Json(res))
}