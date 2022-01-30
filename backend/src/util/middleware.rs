use crate::services::auth::{AuthService, ValidateTokenRequest, ValidateTokenResponse};
use crate::util::http::ServiceError;
use chrono::TimeZone;
use chrono::{DateTime, Utc};
use std::future::{ready, Ready};

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::LocalBoxFuture;
use std::rc::Rc;

const SESSION_ID_COOKIE_NAME: &str = "app_session_id";

pub struct CheckLogin {
    pub auth_service: AuthService,
}

impl<S: 'static, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware {
            service: Rc::new(service),
            auth_service: Rc::new(self.auth_service.clone()),
        }))
    }
}

pub struct CheckLoginMiddleware<S> {
    auth_service: Rc<AuthService>,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("Checking login");
        let svc = self.service.clone();
        let (http_req, payload) = req.into_parts();
        let token: Option<String> = http_req
            .cookie(SESSION_ID_COOKIE_NAME)
            .map(|cookie| cookie.value().to_owned());
        let auth_service = self.auth_service.clone();
        Box::pin(async move {
            match token {
                Some(token) => {
                    log::info!("Got token: {}", token);
                    let validate_req = ValidateTokenRequest { token };
                    match auth_service.validate_session(validate_req).await {
                        Ok(ValidateTokenResponse { is_valid: true }) => {
                            log::info!("token is valid!");
                            let req = ServiceRequest::from_parts(http_req, payload);
                            svc.call(req).await.map(ServiceResponse::map_into_left_body)
                        }
                        _ => {
                            log::info!("token is not valid");
                            let response = HttpResponse::from_error(ServiceError::unauthorized())
                                .map_into_right_body();
                            Ok(ServiceResponse::new(http_req, response))
                        }
                    }
                }
                None => {
                    log::info!("No token :(");
                    let response = HttpResponse::from_error(ServiceError::unauthorized())
                        .map_into_right_body();
                    Ok(ServiceResponse::new(http_req, response))
                }
            }
        })
    }

    // fn call(&self, req: ServiceRequest) -> Self::Future {
    //     let svc = self.service.clone();
    //     let (http_req, payload) = req.into_parts();
    //     let is_logged_in = match http_req.cookie(SESSION_ID_COOKIE_NAME) {
    //         Some(cookie) => {
    //             // Make call to auth service here somehow
    //             let token = cookie.value().to_owned();
    //             let req = ValidateTokenRequest { token };
    //             //let res = self.auth_service.validate_session(req).await;
    //             true
    //             // if let Some(date) = expires {
    //             //     let expires = date.unix_timestamp();
    //             //     let now = chrono::Utc::now().timestamp();
    //             //     log::info!("expires: {}, now: {}, result: {}", expires, now, now <= expires);
    //             //     now <= expires
    //             // } else {
    //             //     log::info!("no cookie expiry date?");
    //             //     false
    //             // }
    //         },
    //         None => {
    //             log::info!("no cookie :(");
    //             false
    //         }
    //     };
    //     Box::pin(async move {
    //         if is_logged_in {
    //             let req = ServiceRequest::from_parts(http_req, payload);
    //             svc.call(req).await.map(ServiceResponse::map_into_left_body)
    //         } else {
    //             let response = HttpResponse::from_error(ServiceError::unauthorized())
    //                 .map_into_right_body();
    //             Ok(ServiceResponse::new(http_req, response))
    //         }
    //     })
    // }
}
