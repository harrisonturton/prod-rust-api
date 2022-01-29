use super::auth_service::AuthService;
use crate::services::user::UserService;
use actix_web::post;
use actix_web::web::{scope, Data, Json, ServiceConfig};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn configure(pool: PgPool) -> impl Fn(&mut ServiceConfig) {
    move |cfg| {
        let user_service = UserService::new(pool.clone());
        let auth_service = AuthService::new(user_service);
        cfg.service(
            scope("/auth")
                .app_data(Data::new(auth_service))
                .configure(routes),
        );
    }
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
async fn sign_in(
    service: Data<AuthService>,
    req: Json<SignInRequest>,
) -> Option<Json<SignInResponse>> {
    let res = service.into_inner().sign_in(req.into_inner()).await?;
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
async fn sign_out(
    service: Data<AuthService>,
    req: Json<SignOutRequest>,
) -> Option<Json<SignOutResponse>> {
    let res = service.into_inner().sign_out(req.into_inner()).await?;
    Some(Json(res))
}
