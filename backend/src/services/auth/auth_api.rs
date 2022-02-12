use super::auth_service::AuthService;
use crate::base::http::Result;
use crate::base::request::RequestContext;
use crate::config::Config;
use actix_web::cookie::Cookie;
use actix_web::post;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(sign_in);
    cfg.service(sign_out);
    cfg.service(validate_session);
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
    config: Data<Config>,
    service: Data<AuthService>,
    ctx: RequestContext,
    req: Json<SignInRequest>,
) -> Result<HttpResponse> {
    let res = service.into_inner().sign_in(&ctx, req.into_inner()).await?;
    let mut cookie = Cookie::new(&config.auth.sat_cookie_name, &res.token);
    cookie.set_path("/");
    cookie.set_same_site(None);
    cookie.set_http_only(true);
    cookie.set_secure(false);
    let http_res = HttpResponse::Ok().cookie(cookie).json(res);
    Ok(http_res)
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

// Don't derive `Debug` to make it harder to log SAT tokens.
#[derive(Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct ValidateTokenResponse {
    pub is_valid: bool,
}

#[post("/sat")]
async fn validate_session(
    service: Data<AuthService>,
    req: Json<ValidateTokenRequest>,
) -> Result<Json<ValidateTokenResponse>> {
    service
        .into_inner()
        .validate_session(req.into_inner())
        .await
        .map(Json)
}
