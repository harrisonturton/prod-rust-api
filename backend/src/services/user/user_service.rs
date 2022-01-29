use super::user_api::{CreateUserRequest, CreateUserResponse};
use super::user_api::{FindUserRequest, FindUserResponse};
use super::user_id_generator::generate_id;
use super::user_model::User;
use sqlx::PgPool;

pub async fn find_user(db: &PgPool, req: FindUserRequest) -> Option<FindUserResponse> {
    let query = r#"
        SELECT id, email FROM users WHERE id = $1
    "#;
    let user: (String, String) = sqlx::query_as(query)
        .bind(req.id)
        .fetch_one(db)
        .await
        .ok()?;
    let user = User {
        id: user.0,
        email: user.1,
    };
    Some(FindUserResponse { user })
}

pub async fn create_user(db: &PgPool, req: CreateUserRequest) -> Option<CreateUserResponse> {
    let query = r#"
        INSERT INTO users (id, email, hash) VALUES ($1, $2, $3) RETURNING id, email
    "#;
    let email = req.email;
    let id = generate_id();
    let hash = String::from("password-hash");
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
