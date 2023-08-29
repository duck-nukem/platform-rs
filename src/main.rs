use axum::{Extension, Router, routing::get, routing::post};
use axum_login::{
    AuthLayer, axum_sessions::{async_session::MemoryStore, SessionLayer}, RequireAuthorizationLayer, SqliteStore,
};
use dotenv::dotenv;
use rand::random;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use authn::views::login;

use crate::authn::models::User;
use crate::authn::views;

mod authn;
mod templates;
mod database;
mod deserialization;

type AuthContext = axum_login::extractors::AuthContext<i64, User, SqliteStore<User>>;

pub async fn app() -> Router {
    let secret = random::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);
    let pool = SqlitePoolOptions::new().connect("sqlite.db").await.unwrap();

    let user_store = SqliteStore::<User>::new(pool.clone());
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
        .layer(auth_layer)
        .layer(session_layer)
        .layer(trace_layer)
        .layer(Extension(pool.clone()))
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
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app().await.into_make_service())
    );
    tracing::info!("Ready to accept connections at :3000");
    let _ = tokio::try_join!(thread);
}
