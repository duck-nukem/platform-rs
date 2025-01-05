pub mod models;
pub mod repository;
pub mod views;

use axum::{routing::get, routing::post, Router};

use crate::routing::SerializableAsUrl;

use super::views::{
    auth::{login_handler, login_view, logout_handler},
    signup::{signup_handler, signup_view},
};

#[derive(Clone, Copy, Debug)]
pub enum AuthRoute {
    Login,
    Logout,
    Signup,
}

impl SerializableAsUrl for AuthRoute {
    fn as_url(&self) -> &'static str {
        match self {
            AuthRoute::Login => "/login",
            AuthRoute::Logout => "/logout",
            AuthRoute::Signup => "/signup",
        }
    }
}

pub fn routes() -> Router {
    Router::new()
        .route(
            AuthRoute::Login.as_url(),
            get(login_view).post(login_handler),
        )
        .route(AuthRoute::Logout.as_url(), post(logout_handler))
        .route(
            AuthRoute::Signup.as_url(),
            get(signup_view).post(signup_handler),
        )
}
