use super::user_access_checker as access_checker;
use super::user_api::ListUsersResponse;
use super::user_api::{CreateUserRequest, CreateUserResponse};
use super::user_api::{FindUserRequest, FindUserResponse};
use super::user_model::User;
use super::user_repo;
use crate::util::hash::Hash;
use crate::util::http::{Result, ServiceError};
use crate::util::id_generator::generate_id;
use crate::util::time::now;
use crate::util::request::RequestContext;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserService {
    pub db: PgPool,
}

impl UserService {
    pub fn new(db: PgPool) -> UserService {
        UserService { db }
    }

    pub fn install(db: &PgPool) -> UserService {
        UserService { db: db.clone() }
    }

    pub async fn list_users(&self, ctx: &RequestContext) -> Result<ListUsersResponse> {
        access_checker::can_access_list_users(ctx)?;
        let users = user_repo::list_all_users(&self.db).await?;
        Ok(ListUsersResponse { users })
    }

    pub async fn find_user(
        &self,
        ctx: &RequestContext,
        req: FindUserRequest,
    ) -> Result<FindUserResponse> {
        access_checker::can_access_find_user(ctx)?;
        let user = match req {
            FindUserRequest::ById { by_id } => user_repo::find_user_by_id(&self.db, by_id).await?,
            FindUserRequest::ByEmail { by_email } => {
                user_repo::find_user_by_email(&self.db, by_email).await?
            }
        };
        Ok(FindUserResponse { user })
    }

    pub async fn create_user(
        &self,
        ctx: &RequestContext,
        req: CreateUserRequest,
    ) -> Result<CreateUserResponse> {
        access_checker::can_access_create_user(ctx)?;
        let CreateUserRequest { email, password } = req;
        let id = generate_id('U');
        let hash = Hash::hash(&password)
            .ok_or_else(ServiceError::server_error)?
            .serialize();
        let user = User { id, email, hash, created_at: now() };
        user_repo::create_user(&self.db, &user).await?;
        Ok(CreateUserResponse { user })
    }
}
