use crate::authn::views::auth::{login_handler, login_view, logout_handler};
use crate::authn::views::signup::{signup_handler, signup_view};
use crate::routing::SerializableAsUrl;
use axum::routing::{get, post};
use axum::Router;

#[derive(Clone, Copy, Debug)]
pub enum AuthRoute {
    Login,
    Logout,
    Signup,
}

impl SerializableAsUrl for AuthRoute {
    fn as_url(&self) -> &'static str {
        match self {
            Self::Login => "/login",
            Self::Logout => "/logout",
            Self::Signup => "/signup",
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
