use crate::auth::AuthServiceApi;
use std::rc::Rc;
use crate::user::UserService;
use crate::Services;
use actix_web::web::{scope, Data, Path, ServiceConfig};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug)]
pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self {}
    }
}

impl AuthServiceApi for AuthService {
    fn is_authenticated(&self, user: String) -> String {
        format!("{} are authenticated", user)
    }
}

// These two methods are similar to the modules files in Spring boot.
// This function would initialize the database and auth service state.

// // Higher-order function so we can inject dependencies
// pub fn build_auth_service<'a>(auth_service: &'static AuthService) -> impl Fn(&mut ServiceConfig) + 'a {
//     |cfg: &mut ServiceConfig| {
//         // Make AuthService mockable or fakeable
//         // Call it the dependencies struct? State struct?

//         // Probably initialize package-level dependencies here
//         // This is called for every handler... once? How should we best think
//         // about concurrent DB access?

//         cfg.service(scope("/auth").data(state).configure(init_routes));
//     }
// }

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(signin);
}

#[get("/")]
async fn index() -> impl Responder {
    "auth index"
}

// These are the "presenter" layer and only exist to transform web-level data
// into service calls. Actual business logic should live in the service instances.
#[get("/{user}")]
async fn signin(user: Path<String>, serv: Data<AuthService>) -> impl Responder {
    serv.is_authenticated(user.to_string())
}