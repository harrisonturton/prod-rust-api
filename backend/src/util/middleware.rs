use std::future::{ready, Ready};
use crate::util::http::ServiceError;

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{Error, HttpResponse};
use futures::future::LocalBoxFuture;
use std::rc::Rc;

const SESSION_ID_COOKIE_NAME: &str = "app_session_id";

pub struct CheckLogin;

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
        }))
    }
}

pub struct CheckLoginMiddleware<S> {
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
        let svc = self.service.clone();
        let (http_req, payload) = req.into_parts();
        let is_logged_in = http_req.cookie(SESSION_ID_COOKIE_NAME).is_some();
        Box::pin(async move {
            if is_logged_in {
                let req = ServiceRequest::from_parts(http_req, payload);
                svc.call(req).await.map(ServiceResponse::map_into_left_body)
            } else {
                let response = HttpResponse::from_error(ServiceError::unauthorized())
                    .map_into_right_body();
                Ok(ServiceResponse::new(http_req, response))
            }
        })
    }
}
