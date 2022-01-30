use crate::services;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;

pub fn start(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // `PgPool` is threadsafe and cheap to clone.
            .app_data(db_pool.clone())
            .configure(services::health::configure(db_pool.clone()))
            .configure(services::user::configure(db_pool.clone()))
            .configure(services::auth::configure(db_pool.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// use actix_web::middleware::{ErrorHandlers, ErrorHandlerResponse};
// use actix_web::dev;
// use actix_web::http;
// use actix_web::{Result, web::Json};
// use actix_web::body::EitherBody;
// use serde::Serialize;

// fn wrap_with_json<B>(mut res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<EitherBody<B>>> {
//     // res.response_mut()
//     //     .headers_mut()
//     //     .insert(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("Error"));
//     Ok(ErrorHandlerResponse::Response(res))
// }

// F: Fn(ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> + 'static,

/*
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;

pub struct SerializeToJson;

impl<S, B> Transform<S, ServiceRequest> for SerializeToJson
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SerializeToJsonMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SerializeToJsonMiddleware { service })
    }
}

pub struct SerializeToJsonMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SerializeToJsonMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("JSON RESPONSE MAPPER AWAITING RESPONSE");
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            log::info!("JSON RESPONSE MAPPER GOT RESPONSE");
            Ok(res)
        })
    }
}*/

/*use serde::Serialize;
use actix_web::{web::Json, Responder, HttpRequest, HttpResponse};

type Resp<T: Serialize> = T;

impl<T: Serialize> Responder for Resp<T> {
    fn respond_to(self, req: &HttpRequest) -> Ready<Result<HttpResponse, Error>> {
        let body = Json(&self);
        let string = body.serialize();
        todo!()
    }
}

// impl<T: Serialize> Responder for Json<T> {
//     type Error = Error;
//     type Future = Ready<Result<Response, Error>>;

//     fn respond_to(self, _: &HttpRequest) -> Self::Future {
//         let body = match serde_json::to_string(&self.0) {
//             Ok(body) => body,
//             Err(e) => return err(e.into()),
//         };

//         ok(Response::build(StatusCode::OK)
//             .content_type("application/json")
//             .body(body))
//     }
// }
// impl Responder for MyObj {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }*/
