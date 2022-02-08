use crate::config::Config;
use crate::services::auth::AuthService;
use crate::services::auth::ValidateTokenRequest;
use crate::base::http::ServiceError;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub identity: Identity,
}

#[derive(Clone, Debug)]
pub enum Identity {
    Anon,
    Service(ServiceIdentity),
    User(String),
}

#[derive(Clone, Debug)]
pub enum ServiceIdentity {
    UserService,
    AuthService,
}

impl RequestContext {
    pub fn anon() -> RequestContext {
        RequestContext {
            identity: Identity::Anon,
        }
    }

    pub fn user(token: String) -> RequestContext {
        RequestContext {
            identity: Identity::User(token),
        }
    }

    pub fn service(service: ServiceIdentity) -> RequestContext {
        RequestContext {
            identity: Identity::Service(service),
        }
    }
}

impl FromRequest for RequestContext {
    type Error = ServiceError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let config = req.app_data::<Data<Config>>().ok_or_else(|| {
                ServiceError::server_error()
                    .with_message("RequestContext extractor could not get AuthCOnfig")
            })?;

            let maybe_cookie = req.cookie(&config.auth.sat_cookie_name);
            let token = match maybe_cookie {
                Some(cookie) => cookie.value().to_owned(),
                None => {
                    let ctx = RequestContext::anon();
                    return Ok(ctx);
                }
            };

            let auth_service = match req.app_data::<Data<AuthService>>() {
                Some(auth_service) => auth_service,
                None => {
                    let err = ServiceError::server_error()
                        .with_message("RequestContext could not get AuthService");
                    return Err(err);
                }
            };
            let req = ValidateTokenRequest {
                token: token.clone(),
            };
            let res = auth_service.validate_session(req).await?;
            if res.is_valid {
                Ok(RequestContext::user(token))
            } else {
                Ok(RequestContext::anon())
            }
        })
    }
}
