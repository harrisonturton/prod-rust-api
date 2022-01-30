use super::auth_api::{SignInRequest, SignInResponse};
use super::auth_api::{SignOutRequest, SignOutResponse};
use crate::services::user::FindUserRequest;
use crate::services::user::UserService;
use crate::util::hash::{generate_token, Hash};

pub struct AuthService {
    pub user_service: UserService,
}

impl AuthService {
    pub fn new(user_service: UserService) -> AuthService {
        AuthService { user_service }
    }

    pub async fn sign_in(&self, req: SignInRequest) -> Option<SignInResponse> {
        let find_user_req = FindUserRequest::ByEmail {
            by_email: req.email,
        };
        let find_user_res = self.user_service.find_user(find_user_req).await?;
        let user = find_user_res.user;

        let hash = Hash::deserialize(&user.hash)?;
        if !Hash::verify(&hash, &req.password) {
            return None;
        }
        let token = generate_token()?;
        Some(SignInResponse { token })
    }

    pub async fn sign_out(&self, _req: SignOutRequest) -> Option<SignOutResponse> {
        Some(SignOutResponse)
    }
}
