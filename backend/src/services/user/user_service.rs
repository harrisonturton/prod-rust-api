use super::user_api::ListUsersResponse;
use super::user_api::{CreateUserRequest, CreateUserResponse};
use super::user_api::{FindUserRequest, FindUserResponse};
use super::user_model::User;
use super::user_repo;
use crate::util::hash::Hash;
use crate::util::id_generator::generate_id;
use sqlx::PgPool;

pub struct UserService {
    pub db: PgPool,
}

impl UserService {
    pub fn new(db: PgPool) -> UserService {
        UserService { db }
    }
}

impl UserService {
    pub async fn find_user(&self, req: FindUserRequest) -> Option<FindUserResponse> {
        let user = match req {
            FindUserRequest::ById { by_id } => user_repo::find_user_by_id(&self.db, by_id).await,
            FindUserRequest::ByEmail { by_email } => {
                user_repo::find_user_by_email(&self.db, by_email).await
            }
        }
        .ok()?;
        Some(FindUserResponse { user })
    }

    pub async fn list_users(&self) -> Option<ListUsersResponse> {
        let users = user_repo::list_all_users(&self.db).await.ok()?;
        Some(ListUsersResponse { users })
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Option<CreateUserResponse> {
        let CreateUserRequest { email, password } = req;
        let id = generate_id('U');
        let hash = Hash::hash(&password)?.serialize();
        let user = User { id, email, hash };
        user_repo::create_user(&self.db, &user).await.ok()?;
        Some(CreateUserResponse { user })
    }
}
