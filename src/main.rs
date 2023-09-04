use std::time::Duration;

use axum::{middleware, routing::get, routing::post, Extension, Router, ServiceExt};
use axum_login::axum_sessions::async_session::CookieStore;
use axum_login::{
    axum_sessions::SessionLayer, AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use dotenv::dotenv;
use rand::random;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_request_id::RequestIdLayer;
use tracing::Level;

use authn::views::login;
use database::get_pool;
use http::{route_auth_guard, set_security_headers};

use crate::authn::models::User;
use crate::authn::views;

mod authn;
mod database;
mod deserialization;
mod http;
mod session;
mod templates;

type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;
pub async fn app() -> Router {
    let secret = random::<[u8; 64]>();
    let pool = get_pool().await;

    let session_store = CookieStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_session_ttl(Some(Duration::from_secs(10 * 60)))
        .with_http_only(true)
        // has to be false for safari on localhost as it doesn't seem to respect
        // that secure=true should be transmitted for http://localhost
        .with_secure(false);

    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let log_level = Level::INFO;
    let trace_layer = TraceLayer::new_for_http()
        .on_request(trace::DefaultOnRequest::new().level(log_level))
        .make_span_with(trace::DefaultMakeSpan::new().level(log_level))
        .on_response(trace::DefaultOnResponse::new().level(log_level));

    Router::new()
        .route("/greet", get(views::logged_in_view))
        // ⬆️ authenticated views go above
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // ⬇️ public views go below
        .route("/login", get(login::login_view))
        .route("/login", post(login::login_handler))
        .route("/logout", post(views::logout_handler))
        .route("/signup", get(views::signup_view))
        .route("/signup", post(views::signup_handler))
        .route_layer(middleware::from_fn(route_auth_guard))
        .route_layer(middleware::from_fn(set_security_headers))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(trace_layer)
        .layer(Extension(pool.clone()))
        .layer(RequestIdLayer)
        .layer(tower_http::compression::CompressionLayer::new())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .with_max_level(Level::INFO)
        .init();

    let thread = tokio::spawn(
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app().await.into_make_service()),
    );
    tracing::info!("Ready to accept connections at :3000");
    let _ = tokio::try_join!(thread);
}
