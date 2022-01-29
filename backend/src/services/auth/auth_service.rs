use super::auth_api::{SignInRequest, SignInResponse};
use super::auth_api::{SignOutRequest, SignOutResponse};
use sqlx::PgPool;

pub async fn sign_in(_db: &PgPool, _req: SignInRequest) -> Option<SignInResponse> {
    Some(SignInResponse {
        token: String::from("Sign-in token")
    })
}

pub async fn sign_out(_db: &PgPool, _req: SignOutRequest) -> Option<SignOutResponse> {
    Some(SignOutResponse)
}