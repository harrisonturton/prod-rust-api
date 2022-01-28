use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::profile::profile_api::{get_user, GetUserRequest};
use crate::crypto::{create_token, check_password};
use super::auth_repo::{AuthRepo, Db};

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("api-auth-postgres", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/api/auth", routes![signin])
    })
}

// Don't derive `Debug` to avoid logging sensitive `password` field
#[derive(Clone, Deserialize)]
pub struct SignInRequest<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

// Don't derive `Debug` to avoid logging sensitive `token` field
#[derive(Clone, Serialize)]
pub struct SignInResponse {
    pub token: String,
}

#[post("/signin", format = "json", data = "<req>")]
async fn signin(req: Json<SignInRequest<'_>>, db: Db) -> Option<Json<SignInResponse>> {
    let get_user_req = GetUserRequest::ByEmail {
        email: req.email,
    };
    //let user = get_user(get_user_req, db).await?;

    let repo = AuthRepo::new(&db);
    let attempt = String::from(req.password);
    let email = String::from(req.email);
    let hash = repo.get_password_hash_by_email(email).await?;
    if !check_password(&attempt, &hash) {
        println!("Bad password >:(");
        return None;
    }
    let token = create_token()?;
    //repo.create_session(user.id, token);
    let res = SignInResponse { token: token };
    Some(Json::from(res))
}