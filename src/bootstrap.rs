use std::env;
use std::time::Duration;

use axum::{Extension, middleware, Router};
use axum_login::{AuthLayer, SqlxStore};
use axum_login::axum_sessions::{PersistencePolicy, SameSite, SessionLayer};
use axum_login::axum_sessions::async_session::SessionStore;
use sqlx::PgPool;

use crate::authn::models::User;
use crate::http::{handler_404, route_auth_guard, set_security_headers};

pub type AppSecret = [u8; 64];

pub fn get_app_secret() -> AppSecret {
    let secret = env::var("APP_SECRET").expect("App Secret is either undefined or not exactly 64 char long!");
    let mut secret_bytes: AppSecret = [0; 64];
    secret_bytes.copy_from_slice(secret.as_bytes());

    secret_bytes
}

pub fn build_session_layer(
    session_storage: impl SessionStore,
    secret: &AppSecret,
) -> SessionLayer<impl SessionStore> {
    let session_duration_minutes = env::var("SESSION_LIFETIME_MINUTES")
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .expect("Invalid session lifetime; can't convert to numeric value");
    let is_secure_cookie = env::var("SECURE_COOKIE")
        .unwrap_or("true".to_string())
        .to_ascii_lowercase()
        .eq("true");
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