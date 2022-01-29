use super::user_model::User;
use sqlx::{query_as, Error as PgError, PgPool};

pub enum RepoError {
    NotFound,
    UnexpectedError(PgError),
}

pub async fn find_user_by_id(pool: &PgPool, id: String) -> Result<User, RepoError> {
    let query = r#"
        SELECT id, email FROM users WHERE id = $1
    "#;
    let row: Result<(String, String), PgError> = query_as(query).bind(id).fetch_one(pool).await;
    row.map(User::from).map_err(RepoError::from)
}

pub async fn find_user_by_email(pool: &PgPool, email: String) -> Result<User, RepoError> {
    let query = r#"
        SELECT id, email FROM users WHERE email = $1
    "#;
    let row: (String, String) = query_as(query)
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(RepoError::from)?;
    Ok(User::from(row))
}

pub async fn list_all_users(pool: &PgPool) -> Result<Vec<User>, RepoError> {
    let query = r#"
        SELECT id, email FROM users
    "#;
    let rows: Vec<(String, String)> = query_as(query)
        .fetch_all(pool)
        .await
        .map_err(RepoError::from)?;
    Ok(rows.into_iter().map(User::from).collect())
}

impl From<PgError> for RepoError {
    fn from(err: PgError) -> RepoError {
        match err {
            PgError::RowNotFound => RepoError::NotFound,
            _ => RepoError::UnexpectedError(err),
        }
    }
}

impl From<(String, String)> for User {
    fn from(row: (String, String)) -> User {
        let (id, email) = row;
        User { id, email }
    }
}
