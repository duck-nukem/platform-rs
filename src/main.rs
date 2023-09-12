#[macro_use]
extern crate rust_i18n;

use std::env;
use std::time::Duration;

use axum::{middleware, routing::get, routing::post, Extension, Router};
use axum_login::{
    axum_sessions::SessionLayer, AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use dotenv::dotenv;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tower_request_id::RequestIdLayer;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use database::get_pool;
use http::{route_auth_guard, set_security_headers};

use crate::authn::models::User;
use crate::authn::views;
use crate::authn::views::{auth, signup};
use crate::http::handler_404;
use crate::session::DatabaseSessionStore;

mod authn;
mod database;
mod deserialization;
mod http;
mod session;
mod templates;

i18n!("locales", fallback = "en");
type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;

pub async fn app(pool: PgPool, with_tracing: bool) -> Router {
    let mut secret: [u8; 64] = [0; 64];
    secret.copy_from_slice(
        env::var("APP_SECRET")
            .expect("App Secret is either undefined or not exactly 64 char long!")
            .as_bytes(),
    );

    let session_store = DatabaseSessionStore::new(pool.clone());
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_session_ttl(Some(Duration::from_secs(10 * 60)))
        .with_http_only(true)
        .with_secure(
            env::var("SECURE_COOKIE")
                .unwrap_or("true".to_string())
                .to_ascii_lowercase()
                .eq("true"),
        );

    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let mut router = Router::new()
        .route("/greet", get(views::logged_in_view))
        // ⬆️ authenticated views go above
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // ⬇️ public views go below
        .route("/login", get(auth::login_view))
        .route("/login", post(auth::login_handler))
        .route("/logout", post(auth::logout_handler))
        .route("/signup", get(signup::signup_view))
        .route("/signup", post(signup::signup_handler))
        .route_layer(middleware::from_fn(set_security_headers))
        .route_layer(middleware::from_fn(route_auth_guard))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(pool.clone()))
        .layer(RequestIdLayer)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower::ServiceBuilder::new().concurrency_limit(32))
        .fallback(handler_404);

    if with_tracing {
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("platform-rs")
            .install_simple()
            .expect("Telemetry Agent setup failed");
        let trace_layer = TraceLayer::new_for_http()
            .on_request(tower_http::trace::DefaultOnRequest::new().level(Level::INFO))
            .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO));
        tracing_subscriber::registry()
            .with(LevelFilter::INFO)
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()
            .expect("Failed to register tracer with registry");
        router = router.layer(trace_layer);
    }

    router = router.fallback(handler_404);

    router
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    let pool = get_pool().await;

    tracing::info!("Ready to accept connections at :3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(pool.clone(), true).await.into_make_service())
        .await
        .unwrap();

    opentelemetry::global::shutdown_tracer_provider();
}
