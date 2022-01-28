use rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub hash: String,
}
