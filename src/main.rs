use authn::views::login;
use axum::{
    http::{header::LOCATION, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::get,
    routing::post,
    Extension, Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use database::get_pool;
use dotenv::dotenv;
use rand::random;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::authn::models::User;
use crate::authn::views;

mod authn;
mod database;
mod deserialization;
mod templates;

type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;

async fn http_status_redirect_handler<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;
    let is_unauthenticated = response.status() == StatusCode::UNAUTHORIZED;
    let is_unauthorized = response.status() == StatusCode::FORBIDDEN;
    let is_required_auth_missing_or_invalid = is_unauthenticated || is_unauthorized;

    if is_required_auth_missing_or_invalid {
        *response.status_mut() = StatusCode::FOUND;
        response.headers_mut().insert(
            LOCATION,
            HeaderValue::from_static("login?message=auth_required"),
        );
    }

    Ok(response)
}

pub async fn app() -> Router {
    let secret = random::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);
    let pool = get_pool().await;

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
        .route_layer(axum::middleware::from_fn(http_status_redirect_handler))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(trace_layer)
        .layer(Extension(pool.clone()))
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
