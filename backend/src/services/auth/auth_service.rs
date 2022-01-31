use super::auth_api::{SignInRequest, SignInResponse};
use super::auth_api::{SignOutRequest, SignOutResponse};
use super::auth_api::{ValidateTokenRequest, ValidateTokenResponse};
use super::auth_model::Session;
use super::auth_repo;
use crate::config::AuthConfig;
use crate::services::user::FindUserRequest;
use crate::services::user::UserService;
use crate::util::hash::{generate_token, Hash};
use crate::util::http::{Result, ServiceError};
use crate::util::time;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AuthService {
    pub config: AuthConfig,
    pub db: PgPool,
    pub user_service: UserService,
}

impl AuthService {
    pub fn new(config: AuthConfig, db: PgPool, user_service: UserService) -> AuthService {
        AuthService {
            config,
            db,
            user_service,
        }
    }

    pub async fn sign_in(&self, req: SignInRequest) -> Result<SignInResponse> {
        let find_user_req = FindUserRequest::ByEmail {
            by_email: req.email,
        };
        let find_user_res = self
            .user_service
            .find_user(find_user_req)
            .await
            .map_err(|_| ServiceError::unauthorized())?;
        let user = find_user_res.user;

        let hash = Hash::deserialize(&user.hash).ok_or_else(ServiceError::unauthorized)?;
        if !Hash::verify(&hash, &req.password) {
            return Err(ServiceError::unauthorized());
        }
        let token = generate_token().ok_or_else(ServiceError::server_error)?;
        let session = Session {
            user_id: user.id,
            token: token.clone(),
            created_at: time::now(),
        };
        auth_repo::create_session(&self.db, &session)
            .await
            .map_err(|_| ServiceError::server_error())?;
        Ok(SignInResponse { token })
    }

    pub async fn sign_out(&self, _req: SignOutRequest) -> Option<SignOutResponse> {
        Some(SignOutResponse)
    }

    pub async fn validate_session(
        &self,
        req: ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse> {
        let session = auth_repo::find_session_by_token(&self.db, &req.token).await?;
        let session_duration = time::now().signed_duration_since(session.created_at);
        log::info!(
            "signed duration since token was created: {}",
            session_duration
        );
        let max_session_duration = self.config.sat_cookie_lifetime_mins;
        let is_valid = session_duration <= chrono::Duration::minutes(max_session_duration.into());
        Ok(ValidateTokenResponse { is_valid })
    }
}
