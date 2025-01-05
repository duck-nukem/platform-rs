#[macro_use]
extern crate rust_i18n;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::{Extension, middleware, Router, routing::get};
use axum_login::{
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use dotenv::dotenv;
use sqlx::PgPool;

use database::get_pool;
use http::{route_auth_guard, set_security_headers};

pub(crate) use crate::authn::models::User;
use crate::authn::views;

use crate::http::handler_404;
use crate::session::CookieStore;

mod authn;
mod bootstrap;
mod database;
mod deserialization;
mod http;
mod routing;
mod session;
mod templates;

#[cfg(test)]
mod tests;

i18n!("locales", fallback = "en");
type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;

#[derive(Clone)]
pub enum Tracing {
    Enabled,
    Disabled,
}

pub async fn app(pool: PgPool, _with_tracing: Tracing) -> Router {
    let secret = bootstrap::read_secret_from_env();
    let session_layer = bootstrap::build_session_layer(CookieStore::new(), &secret);
    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let mut router = Router::new()
        .route("/greet", get(views::logged_in_view))
        // ↑ authenticated views go above
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // ↓ public views go below
        .nest("/auth", authn::routes())
        .route_layer(middleware::from_fn(set_security_headers))
        .route_layer(middleware::from_fn(route_auth_guard))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(pool.clone()))
        .layer(tower_request_id::RequestIdLayer)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower::ServiceBuilder::new().concurrency_limit(32))
        .fallback(handler_404);

    router = router.fallback(handler_404);

    router
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    let octets = env::var("SERVER_HOST")
        .unwrap_or("0.0.0.0".to_string())
        .split('.')
        .map(|o| {
            o.parse::<u8>()
                .expect("Can't parse octet to numeric format")
        })
        .collect::<Vec<u8>>();
    let ip = IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));
    let port = env::var("SERVER_PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("Can't parse port to numeric format");
    let server_socket_address = SocketAddr::new(ip, port);

    println!("Ready to accept connections at {}", server_socket_address);
    let pool = get_pool().await;

    axum::Server::bind(&server_socket_address)
        .serve(
            app(pool.clone(), Tracing::Enabled)
                .await
                .into_make_service(),
        )
        .await
        .unwrap();
}
