use super::user_model::User;
use crate::util::http::{Result, ServiceError};
use sqlx::{query_as, PgPool};

pub async fn find_user_by_id(pool: &PgPool, id: String) -> Result<User> {
    let query = r#"
        SELECT id, email, hash FROM users WHERE id = $1
    "#;
    let row: (String, String, String) = query_as(query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::from)?;
    Ok(User::from(row))
}

pub async fn find_user_by_email(pool: &PgPool, email: String) -> Result<User> {
    let query = r#"
        SELECT id, email, hash FROM users WHERE email = $1
    "#;
    let row: (String, String, String) = query_as(query)
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::from)?;
    Ok(User::from(row))
}

pub async fn list_all_users(pool: &PgPool) -> Result<Vec<User>> {
    let query = r#"
        SELECT id, email, hash FROM users
    "#;
    let rows: Vec<(String, String, String)> = query_as(query)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::from)?;
    Ok(rows.into_iter().map(User::from).collect())
}

pub async fn create_user(pool: &PgPool, user: &User) -> Result<User> {
    let query = r#"
        INSERT INTO users (id, email, hash) VALUES ($1, $2, $3) RETURNING id, email, hash
    "#;
    let row: (String, String, String) = sqlx::query_as(query)
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.hash)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::from)?;
    Ok(User::from(row))
}

impl From<(String, String, String)> for User {
    fn from(row: (String, String, String)) -> User {
        let (id, email, hash) = row;
        User { id, email, hash }
    }
}
