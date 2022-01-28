mod auth;
use std::fmt::Debug;
pub use auth::*;

pub mod service;

pub trait AuthServiceApi: Debug {
    fn is_authenticated(&self, user: String) -> String;
}