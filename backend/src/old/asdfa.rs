// mod profile;
// mod auth;
// mod crypto;

// ------------------------------------
// Actix Web
// ------------------------------------

use crate::auth::service::AuthClient;
use crate::auth::init_routes;
use std::rc::Rc;
use crate::auth::AuthService;
use crate::user::UserService;
use actix_web::dev::Url;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use core::future::ready;
use core::future::Ready;
use core::ops::Deref;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::io;

mod user;
mod auth;

pub struct Services {
    user: user::UserService, 
    auth: auth::AuthService,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        // This is an anonymous function because it creates a new App instance
        // for each thread. Therefore, data must be constructed multiple times.
        // Shared data must be used with Send + Sync.

        // Wire all dependencies here – this is the DI root.
        // How to construct services DAO? We want services to be able to
        // construct themselves...

        // Cannot DI things because the data is constructed separately for every worker.
        // We have to push this logic/dependencies down somehow.
        let auth_client = AuthClient::new(String::from("state"));
        let user_service = UserService::new(Box::new(auth_client));
        let auth_service = AuthService::new();
        //let auth_service = auth::build_auth_service(&user_service); // Returns constructor

        App::new()
            // Maybe a bit gross to stuff dependencies in here?
            // Maybe have higer-level "Root app state" or "Root service" that
            // composes all available sub-services and pass it to the factory
            // constructor, and let it pick and choose from available dependenices.
            //.configure(user::build_user_service(/* Root deps DAO */)) 
            .service(web::scope("/user").data(user_service).configure(user::init_routes))
            .service(web::scope("/auth").data(auth_service).configure(auth::init_routes))
    })
    .bind("localhost:8000")?
    .run()
    .await
}

// #[derive(Debug)]
// pub enum ServiceError {
//     Unauthorised,
// }

// enum Response<T> {
//     Ok(T),
//     Err(ServiceError),
// }

// impl<T: Responder> Responder for Response<T> {
//     type Error = actix_web::Error;
//     type Future = Ready<Result<HttpResponse, Self::Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body("body")))
//     }
// }

// // impl<T> Responder for Response<T> {
// //     type Error = ServiceError;
// //     type Future = Ready<Result<HttpResponse, Self::Error>>;

// //     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
// //         ready(Ok(HttpResponse::Ok()
// //             .content_type("application/json")
// //             .body("body")))
// //     }
// // }

// trait ProfileService {
//     fn get_user(&self);
// }

// struct Appp {
//     profile: Box<dyn ProfileService>,
// }

// #[derive(Deserialize)]
// struct GetUserRequest {
//     id: String,
// }

// // Implement FromRequest for a generic "Request<T>" type that deserializes JSON
// // into that type (if present) but also provides a handle to URL parameters.

// struct Request<T: DeserializeOwned> {
//     body: T,
//     path: Path<Url>,
// }

// impl<T: DeserializeOwned> FromRequest for Request<T> {
//     type Error = todo!();
//     type Future = todo!();
//     type Config = todo!();
//     fn from_request(
//         _: &actix_web::HttpRequest,
//         _: &mut actix_web::dev::Payload,
//     ) -> <Self as actix_web::FromRequest>::Future {
//         todo!()
//     }
// }

// #[derive(Serialize)]
// struct GetUserResponse {
//     user: String,
// }

// struct RequestContext {
//     authed: bool,
// }

// impl FromRequest for RequestContext {
//     type Error = todo!();
//     type Future = todo!();
//     type Config = todo!();
//     fn from_request(
//         _: &actix_web::HttpRequest,
//         _: &mut actix_web::dev::Payload,
//     ) -> <Self as actix_web::FromRequest>::Future {
//         todo!()
//     }
// }

// trait Service<T, S> {
//     fn run(req: T) -> dyn Future<Output = S>;
// }

// pub struct Test;

// #[get("/user/{user_id}")]
// pub async fn get_user(
//     service: Data<dyn ProfileService>,
//     ctx: RequestContext,
//     req: Request<GetUserRequest>,
// ) -> impl Responder {
//     //let id: &str = req.path.get("user_id").unwrap();
//     let res = GetUserResponse {
//         user: String::from("Harry"),
//     };
//     // Response::Ok(res)
//     HttpResponse::Ok().body("Hey there!")
// }

// ------------------------------------
// Rocket
// ------------------------------------

// #[macro_use]
// extern crate rocket;
//
// #[rocket::main]
// async fn main() {
//     rocket::build()
//         .attach(auth::setup())
//         .attach(profile::setup())
//         .launch()
//         .await
//         .expect("failed to launch server");
// }
