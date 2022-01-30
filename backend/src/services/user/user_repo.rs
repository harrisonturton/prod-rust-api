use super::user_model::User;
use sqlx::{query_as, Error as PgError, PgPool};

pub enum RepoError {
    NotFound,
    UnexpectedError(PgError),
}

pub async fn find_user_by_id(pool: &PgPool, id: String) -> Result<User, RepoError> {
    let query = r#"
        SELECT id, email, hash FROM users WHERE id = $1
    "#;
    let row: Result<(String, String, String), PgError> =
        query_as(query).bind(id).fetch_one(pool).await;
    row.map(User::from).map_err(RepoError::from)
}

pub async fn find_user_by_email(pool: &PgPool, email: String) -> Result<User, RepoError> {
    let query = r#"
        SELECT id, email, hash FROM users WHERE email = $1
    "#;
    let row: (String, String, String) = query_as(query)
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(RepoError::from)?;
    Ok(User::from(row))
}

pub async fn list_all_users(pool: &PgPool) -> Result<Vec<User>, RepoError> {
    let query = r#"
        SELECT id, email, hash FROM users
    "#;
    let rows: Vec<(String, String, String)> = query_as(query)
        .fetch_all(pool)
        .await
        .map_err(RepoError::from)?;
    Ok(rows.into_iter().map(User::from).collect())
}

pub async fn create_user(pool: &PgPool, user: &User) -> Result<User, RepoError> {
    let query = r#"
        INSERT INTO users (id, email, hash) VALUES ($1, $2, $3) RETURNING id, email, hash
    "#;
    let row: (String, String, String) = sqlx::query_as(query)
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.hash)
        .fetch_one(pool)
        .await
        .map_err(RepoError::from)?;
    Ok(User::from(row))
}

impl From<PgError> for RepoError {
    fn from(err: PgError) -> RepoError {
        match err {
            PgError::RowNotFound => RepoError::NotFound,
            _ => RepoError::UnexpectedError(err),
        }
    }
}

impl From<(String, String, String)> for User {
    fn from(row: (String, String, String)) -> User {
        let (id, email, hash) = row;
        User { id, email, hash }
    }
}
