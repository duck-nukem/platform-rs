use std::convert::TryInto;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use axum::{Extension, middleware, Router};
use axum_login::{AuthLayer, SqlxStore};
use axum_login::axum_sessions::{PersistencePolicy, SameSite, SessionLayer};
use axum_login::axum_sessions::async_session::SessionStore;
use sqlx::PgPool;

use crate::authn::models::User;
use crate::environment::{read_bool_env_var, read_mandatory_env_var, read_numeric_env_var};
use crate::http::{handler_404, route_auth_guard, set_security_headers};

pub type AppSecret = [u8; 64];

pub fn get_app_secret() -> AppSecret {
    let secret = read_mandatory_env_var("APP_SECRET");
    let mut secret_bytes: AppSecret = [0; 64];
    secret_bytes.copy_from_slice(secret.as_bytes());

    secret_bytes
}

pub fn build_session_layer(
    session_storage: impl SessionStore,
    secret: &AppSecret,
) -> SessionLayer<impl SessionStore> {
    let session_duration_minutes = read_numeric_env_var("SESSION_LIFETIME_MINUTES", 10);
    let is_secure_cookie = read_bool_env_var("SECURE_COOKIE", true);
    let session_layer = SessionLayer::new(session_storage, secret)
        .with_persistence_policy(PersistencePolicy::ExistingOnly)
        .with_session_ttl(Some(Duration::from_secs(session_duration_minutes * 60)))
        .with_same_site_policy(SameSite::Strict)
        .with_http_only(true)
        .with_secure(is_secure_cookie);

    session_layer
}

pub fn configure_auxiliary_routing(
    router_with_app_routes: Router,
    user_store: SqlxStore<PgPool, User>,
    session_storage: impl SessionStore,
    database_pool: PgPool,
) -> Router {
    let secret = get_app_secret();
    let auth_layer = AuthLayer::new(user_store, &secret);
    let session_layer = build_session_layer(session_storage, &secret);
    let max_concurrency_limit = env::var("MAX_CONCURRENCY_LIMIT")
        .unwrap_or("32".to_string())
        .parse::<usize>()
        .expect("Invalid concurrency limit; can't convert to numeric value");

    let configured_router = router_with_app_routes
        .route_layer(middleware::from_fn(set_security_headers))
        .route_layer(middleware::from_fn(route_auth_guard))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(database_pool))
        .layer(tower_request_id::RequestIdLayer)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower::ServiceBuilder::new().concurrency_limit(max_concurrency_limit))
        .fallback(handler_404);

    configured_router
}

pub fn build_socket_from_ip_port(ipv4_address: String, port: u16) -> SocketAddr {
    let octets: [u8; 4] = ipv4_address
        .split('.')
        .map(|o| o.parse::<u8>().expect("Can't parse octet to numeric format"))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("IPv4 address must have exactly 4 octets");
    let ip = IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));

    SocketAddr::new(ip, port)
}