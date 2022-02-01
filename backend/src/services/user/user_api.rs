use super::user_service::UserService;
use super::User;
use crate::util::http::Result;
use crate::util::request::RequestContext;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{get, post};
use serde::{Deserialize, Serialize};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(list_users);
    cfg.service(find_user);
    cfg.service(create_user);
}

#[derive(Debug, Serialize)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
}

#[get("/")]
async fn list_users(
    service: Data<UserService>,
    ctx: RequestContext,
) -> Result<Json<ListUsersResponse>> {
    service.into_inner().list_users(&ctx).await.map(Json)
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
    ctx: RequestContext,
    req: Json<FindUserRequest>,
) -> Result<Json<FindUserResponse>> {
    service.find_user(&ctx, req.into_inner()).await.map(Json)
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
    ctx: RequestContext,
    req: Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>> {
    service.create_user(&ctx, req.into_inner()).await.map(Json)
}
