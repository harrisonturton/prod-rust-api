
use crate::auth::AuthServiceApi;

#[derive(Debug)]
pub struct AuthClient {
    state: AuthState,
}

impl AuthClient {
    pub fn new(state: String) -> Self {
        let state = AuthState::new(state);
        Self { state }
    }
}

impl AuthServiceApi for AuthClient {
    fn is_authenticated(&self, user: String) -> String {
        is_authenticated(&self.state, &user)
    }
}

#[derive(Debug)]
pub struct AuthState {
    pub state: String,
}

impl AuthState {
    pub fn new(state: String) -> Self {
        Self { state }
    }
}

pub fn is_authenticated(state: &AuthState, user: &String) -> String {
    format!("[{}] {} is authenticated ", state.state, user)
}