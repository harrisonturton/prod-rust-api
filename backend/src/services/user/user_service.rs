use super::user_api::ListUsersResponse;
use super::user_api::{CreateUserRequest, CreateUserResponse};
use super::user_api::{FindUserRequest, FindUserResponse};
use super::user_id_generator::generate_id;
use super::user_model::User;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserServiceApi {
    async fn find_user(&self, req: FindUserRequest) -> Option<FindUserResponse>;
}

pub struct UserService {
    pub db: PgPool,
}

#[async_trait]
impl UserServiceApi for UserService {
    async fn find_user(&self, req: FindUserRequest) -> Option<FindUserResponse> {
        let user = match req {
            FindUserRequest::ById { by_id } => find_user_by_id(&self.db, by_id).await?,
            FindUserRequest::ByEmail { by_email } => find_user_by_email(&self.db, by_email).await?,
        };
        Some(FindUserResponse { user })
    }
}

pub async fn list_users(db: &PgPool) -> Option<ListUsersResponse> {
    let query = r#"
        SELECT id, email FROM users
    "#;
    let rows: Vec<(String, String)> = sqlx::query_as(query).fetch_all(db).await.ok()?;
    let users = rows
        .into_iter()
        .map(|row: (String, String)| User {
            id: row.0,
            email: row.1,
        })
        .collect();
    Some(ListUsersResponse { users })
}

pub async fn find_user(db: &PgPool, req: FindUserRequest) -> Option<FindUserResponse> {
    let user = match req {
        FindUserRequest::ById { by_id } => find_user_by_id(db, by_id).await?,
        FindUserRequest::ByEmail { by_email } => find_user_by_email(db, by_email).await?,
    };
    Some(FindUserResponse { user })
}

async fn find_user_by_id(db: &PgPool, id: String) -> Option<User> {
    let query = r#"
        SELECT id, email FROM users WHERE id = $1
    "#;
    let user: (String, String) = sqlx::query_as(query).bind(id).fetch_one(db).await.ok()?;
    Some(User {
        id: user.0,
        email: user.1,
    })
}

async fn find_user_by_email(db: &PgPool, email: String) -> Option<User> {
    let query = r#"
        SELECT id, email FROM users WHERE email = $1
    "#;
    let user: (String, String) = sqlx::query_as(query).bind(email).fetch_one(db).await.ok()?;
    Some(User {
        id: user.0,
        email: user.1,
    })
}

pub async fn create_user(db: &PgPool, req: CreateUserRequest) -> Option<CreateUserResponse> {
    let query = r#"
        INSERT INTO users (id, email, hash) VALUES ($1, $2, $3) RETURNING id, email
    "#;
    let email = req.email;
    let id = generate_id();
    let hash = String::from("password-hash"); // TODO(harry): implement hashing
    let row: (String, String) = sqlx::query_as(query)
        .bind(id)
        .bind(email)
        .bind(hash)
        .fetch_one(db)
        .await
        .ok()?;
    let user = User {
        id: row.0,
        email: row.1,
    };
    Some(CreateUserResponse { user })
}
