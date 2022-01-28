use actix_web::{App, HttpServer};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(user_service_api::attach_routes)
            .configure(auth_service_api::attach_routes)
    })
    .bind("localhost:8000")?
    .run()
    .await
}

// ------------------------------------
// User Service
// ------------------------------------

mod user_service_api {
    use async_trait::async_trait;
    use actix_web::web::{scope, ServiceConfig};
    use actix_web::{get, Responder};

    pub fn attach_routes(cfg: &mut ServiceConfig) {
        cfg.service(scope("/user").service(get_user));
    }

    // Service endpoint
    #[get("/")]
    async fn get_user() -> impl Responder {
        let service = UserService;
        let res = service.get_user().await;
        res
    }

    // Trait of the service behaviour so we can mock it
    #[async_trait]
    pub trait UserServiceApi: Send + Sync {
        async fn get_user(&self) -> String;
    }

    // Actual service implementation. Not public.
    pub struct UserService;

    #[async_trait]
    impl UserServiceApi for UserService {
        async fn get_user(&self) -> String {
            use tokio::time::{sleep, Duration};
            sleep(Duration::from_secs(3)).await;
            String::from("Harry Turton")
        }
    }
}

// ------------------------------------
// Auth Service
// ------------------------------------

mod auth_service_api {
    use async_trait::async_trait;
    use actix_web::web::{scope, ServiceConfig, Data};
    use actix_web::{post, Responder};
    use super::user_service_api::{UserServiceApi, UserService};

    pub fn attach_routes(cfg: &mut ServiceConfig) {
        let user_service = Box::new(UserService);
        let auth_service = AuthService { user_service };
        cfg.service(scope("/auth").app_data(Data::new(auth_service)).service(authenticate));
    }

    // Service endpoint
    #[post("/")]
    async fn authenticate(service: Data<AuthService>) -> impl Responder {
        let res = service.sign_in().await;
        res
    }

    // Trait of the service behaviour so we can mock it
    #[async_trait]
    pub trait AuthServiceApi {
        async fn sign_in(&self) -> String;
    }

    // Actual service implementation. Not public.
    struct AuthService {
        user_service: Box<dyn UserServiceApi>,
    }

    #[async_trait]
    impl AuthServiceApi for AuthService {
        async fn sign_in(&self) -> String {
            use tokio::time::{sleep, Duration};
            sleep(Duration::from_secs(3)).await;
            let user = self.user_service.get_user().await;
            format!("{} signed in!", user)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::user_service_api::UserServiceApi;

        struct MockUserService;

        #[async_trait]
        impl UserServiceApi for MockUserService {
            async fn get_user(&self) -> String {
                String::from("Harry Turton")
            }
        }

        #[test]
        fn test_sign_in_calls_user_service() {
            let user_service = Box::new(MockUserService);
            let auth_service = AuthService { user_service };
        }
    }
}