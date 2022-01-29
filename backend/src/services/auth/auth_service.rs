use crate::services::user::UserServiceApi;
use crate::services::user::FindUserRequest;
use super::auth_api::{SignInRequest, SignInResponse};
use super::auth_api::{SignOutRequest, SignOutResponse};
use async_trait::async_trait;

// The API is not really used internally; it is imported by clients or mocks.
#[async_trait]
pub trait AuthServiceApi {
    async fn sign_in(&self, req: SignInRequest) -> Option<SignInResponse>;
    async fn sign_out(&self, req: SignOutRequest) -> Option<SignOutResponse>;
}

pub struct AuthService {
    pub user_service: Box<dyn UserServiceApi + Send + Sync>
}

#[async_trait]
impl AuthServiceApi for AuthService {
    async fn sign_in(&self, req: SignInRequest) -> Option<SignInResponse> {
        let find_user_req = FindUserRequest::ByEmail { by_email: req.email };
        let find_user_res = self.user_service.find_user(find_user_req).await?;
        Some(SignInResponse { token: find_user_res.user.email })
    }

    async fn sign_out(&self, _req: SignOutRequest) -> Option<SignOutResponse> {
        Some(SignOutResponse)
    }
}