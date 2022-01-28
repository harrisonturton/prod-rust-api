use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::crypto::hash_password;

use super::profile_id_generator::generate_user_id;
use super::profile_model::User;
use super::profile_repo::{Db, ProfileRepo};

pub fn setup() -> AdHoc {
    AdHoc::on_ignite("api-profile-postgres", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/api/profile", routes![get_user, create_user])
    })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum GetUserRequest<'r> {
    ById { id: &'r str },
    ByEmail { email: &'r str },
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserResponse {
    pub user: User,
}

#[post("/user", format = "json", data = "<req>")]
pub async fn get_user(req: Json<GetUserRequest<'_>>, db: Db) -> Option<Json<GetUserResponse>> {
    let repo = ProfileRepo::new(&db);
    let user = match req.into_inner() {
        GetUserRequest::ById { id } => {
            let id = String::from(id);
            repo.get_user_by_id(id).await?
        },
        GetUserRequest::ByEmail { email } => {
            let email = String::from(email);
            repo.get_user_by_email(email).await?
        }
    };
    let res = GetUserResponse { user };
    Some(Json::from(res))
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateUserResponse {
    pub user: User,
}

#[post("/user/new", format = "json", data = "<req>")]
pub async fn create_user(req: Json<CreateUserRequest<'_>>, db: Db) -> Option<Json<CreateUserResponse>> {
    let password = String::from(req.password);
    let hash = hash_password(&password)?;
    let hash_string = hash.to_base64_string();

    let mut repo = ProfileRepo::new(&db);
    let user = User {
        id: generate_user_id(),
        email: String::from(req.email),
        hash: hash_string,
    };
    repo.create_user(user.clone()).await?;
    println!("Stored user!");
    let res = CreateUserResponse { user };
    Some(Json::from(res))
}