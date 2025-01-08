use crate::dashboard::views::welcome::greet_user;
use axum::routing::get;
use axum::Router;

use crate::routing::SerializableAsUrl;

#[derive(Clone, Copy, Debug)]
pub enum DashboardRoute {
    Greetings,
}

impl SerializableAsUrl for DashboardRoute {
    fn as_url(&self) -> &'static str {
        match self {
            DashboardRoute::Greetings => "/greet",
        }
    }
}

pub fn routes() -> Router {
    Router::new().route(DashboardRoute::Greetings.as_url(), get(greet_user))
}
