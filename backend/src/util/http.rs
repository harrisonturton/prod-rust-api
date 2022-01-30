use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::Error as DatabaseError;
use std::{fmt, result};

pub type Result<T> = result::Result<T, ServiceError>;

#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct ServiceError {
    pub status: u16,
    pub message: Option<String>,
}

impl ServiceError {
    pub fn unauthorized() -> Self {
        let status = StatusCode::UNAUTHORIZED;
        Self {
            status: status.as_u16(),
            message: status.canonical_reason().map(|msg| msg.to_owned()),
        }
    }

    pub fn server_error() -> Self {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        Self {
            status: status.as_u16(),
            message: status.canonical_reason().map(|msg| msg.to_owned()),
        }
    }

    pub fn not_found() -> Self {
        let status = StatusCode::NOT_FOUND;
        Self {
            status: status.as_u16(),
            message: status.canonical_reason().map(|msg| msg.to_owned()),
        }
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }
}

// This is required for `Error` to implement the `std::error::Error` trait.
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}", message),
            None => write!(f, "{}", self.status),
        }
    }
}

// This is required by `actix_web` for `Error` to be returned in a `Result` enum
// in the HTTP handlers.
impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match StatusCode::from_u16(self.status) {
            Ok(status) => status,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        HttpResponse::build(status).json(self)
    }
}

impl From<ActixError> for ServiceError {
    fn from(_: ActixError) -> ServiceError {
        ServiceError::server_error()
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(err: DatabaseError) -> ServiceError {
        match err {
            DatabaseError::RowNotFound => ServiceError::not_found(),
            _ => ServiceError::server_error(),
        }
    }
}
