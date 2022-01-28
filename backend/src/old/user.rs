use crate::auth::AuthServiceApi;
use crate::auth::service::AuthClient;
use actix_web::web::{scope, Data, Path, ServiceConfig};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug)]
pub struct UserService {
    users: Vec<String>,
    auth: Box<dyn AuthServiceApi>,
}

impl UserService {
    pub fn new(auth: Box<dyn AuthServiceApi>) -> Self {
        Self { users: vec!["Harry".to_owned()], auth }
    }

    pub fn get_user(&self) -> String {
        format!("{:?}", self.users)
    }
}

// These two methods are similar to the modules files in Spring boot.
// This function would initialize the database and user service state.

// Higher-order function so we can inject dependencies
// pub fn build_user_service(/* DAO holding all possible services */) -> impl Fn(&mut ServiceConfig) {
//     |cfg: &mut ServiceConfig| {
//         // Make UserService mockable or fakeable
//         // Call it the dependencies struct? State struct?

//         // Probably initialize package-level dependencies here

//         //let state = UserService::new(/* Inject deps here */);
//         cfg.service(scope("/user").data(state).configure(init_routes));
//     }
// }

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(list);
    cfg.service(hello);
}

#[get("/")]
async fn index() -> impl Responder {
    "Index"
}

// Should we defer to service implementation?
#[get("/list")]
async fn list(serv: Data<UserService>) -> impl Responder {
    serv.get_user()
}

#[get("/hello/{name}")]
async fn hello(name: Path<String>, serv: Data<UserService>) -> impl Responder {
    format!("Hello {}", serv.as_ref().auth.is_authenticated(format!("{}", name)))
}
