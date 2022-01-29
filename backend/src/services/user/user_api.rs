//! Endpoints that expose the user service to the internet. These methods should
//! do the absolute minimum amount of logic; they exist only as a translation
//! layer to pipe requests from `actix_web` to the user service. If we were to
//! change actix for something else, that should be possible simply by replacing
//! this file.

use super::user_service::UserService;
use super::User;
use actix_web::web::{scope, Data, Json, ServiceConfig};
use actix_web::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn configure(pool: PgPool) -> impl Fn(&mut ServiceConfig) {
    move |cfg| {
        let user_service = UserService::new(pool.clone());
        cfg.service(
            scope("/user")
                .app_data(Data::new(user_service))
                .configure(routes),
        );
    }
}

fn routes(cfg: &mut ServiceConfig) {
    cfg.service(list_users);
    cfg.service(find_user);
    cfg.service(create_user);
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
}

#[get("/")]
async fn list_users(service: Data<UserService>) -> Option<Json<ListUsersResponse>> {
    let res = service.into_inner().list_users().await?;
    Some(Json(res))
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FindUserRequest {
    ById { by_id: String },
    ByEmail { by_email: String },
}

#[derive(Debug, Serialize)]
pub struct FindUserResponse {
    pub user: User,
}

#[post("/")]
async fn find_user(
    service: Data<UserService>,
    req: Json<FindUserRequest>,
) -> Option<Json<FindUserResponse>> {
    let res = service.find_user(req.into_inner()).await?;
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
    service: Data<UserService>,
    req: Json<CreateUserRequest>,
) -> Option<Json<CreateUserResponse>> {
    let res = service.create_user(req.into_inner()).await?;
    Some(Json(res))
}
