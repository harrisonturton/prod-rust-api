use super::auth_model::Session;
use crate::util::http::{Result, ServiceError};
use crate::util::time::Timestamp;
use sqlx::{query_as, PgPool};

pub async fn create_session(pool: &PgPool, session: &Session) -> Result<Session> {
    let query = r#"
        INSERT INTO session (user_id, token) VALUES ($1, $2) RETURNING user_id, token, created_at
    "#;
    let row: (String, String, Timestamp) = query_as(query)
        .bind(&session.user_id)
        .bind(&session.token)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::from)?;
    Ok(Session::from(row))
}

pub async fn find_session_by_token(pool: &PgPool, token: &str) -> Result<Session> {
    let query = r#"
        SELECT user_id, token, created_at FROM session WHERE token = $1
    "#;
    let row: (String, String, Timestamp) = query_as(query)
        .bind(token)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::from)?;
    Ok(Session::from(row))
}

impl From<(String, String, Timestamp)> for Session {
    fn from(row: (String, String, Timestamp)) -> Session {
        Session {
            user_id: row.0,
            token: row.1,
            created_at: row.2,
        }
    }
}
