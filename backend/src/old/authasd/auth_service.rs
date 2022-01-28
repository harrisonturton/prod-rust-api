
pub enum ServiceError {
    Unauthorized,
    ServerError,
}

pub struct AuthService {
    repo: AuthRepo,
    profile: ProfileService,
}

// Don't derive `Debug` to avoid logging sensitive `password` field
#[derive(Clone, Deserialize)]
pub struct SignInRequest<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

// Don't derive `Debug` to avoid logging sensitive `token` field
#[derive(Clone, Serialize)]
pub struct SignInResponse {
    pub token: String,
}

impl AuthService {
    pub fn new(repo: AuthRepo, profile: ProfileService) -> AuthService {
        AuthService { repo, profile }
    }

    pub fn sign_in(&mut self, req: SignInRequest) -> Result<SignInResponse, ServiceError> {
        todo!()
    }
}