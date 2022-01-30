use actix_web::http::StatusCode as ActixStatus;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use serde::Serialize;
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug, Serialize)]
pub struct StatusCode {
    pub status: u16,
    pub message: &'static str,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for StatusCode {
    fn status_code(&self) -> ActixStatus {
        StatusCode::actix_status(*self)
    }

    fn error_response(&self) -> HttpResponse {
        let status = StatusCode::actix_status(*self);
        HttpResponse::build(status).json(self)
    }
}

macro_rules! status_codes {
    (
        $(
            ($num:expr, $konst:ident, $canonical_reason:expr, $actix_status:expr);
        )+
    ) => {
        impl StatusCode {
            $(
                pub const $konst: StatusCode = StatusCode {
                    status: $num,
                    message: $canonical_reason,
                };
            )+

            pub fn reason(status: StatusCode) -> &'static str {
                status.message
            }

            pub fn actix_status(status: StatusCode) -> ActixStatus {
                match status {
                    $(StatusCode { status: $num, .. } => $actix_status,)+
                    _ => ActixStatus::INTERNAL_SERVER_ERROR,
                }
            }
        }
    };
}

status_codes! {
    (200, OK, "Ok", ActixStatus::UNAUTHORIZED);
    (401, UNAUTHORIZED, "Forbidden", ActixStatus::UNAUTHORIZED);
    (404, NOT_FOUND, "Not Found", ActixStatus::NOT_FOUND);
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error", ActixStatus::INTERNAL_SERVER_ERROR);
}
