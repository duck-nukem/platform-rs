#[macro_use]
extern crate rust_i18n;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use axum::{middleware, routing::get, Extension, Router};
use axum_login::axum_sessions::{PersistencePolicy, SameSite};
use axum_login::{
    axum_sessions::SessionLayer, AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use dotenv::dotenv;
use session::CookieStore;
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

use crate::http::handler_404;

mod authn;
mod database;
mod deserialization;
mod http;
pub mod routing;
mod session;
mod templates;

pub mod forms;
#[cfg(test)]
mod tests;

i18n!("locales", fallback = "en");
type AuthContext = axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;

#[derive(Clone)]
pub enum Tracing {
    Enabled,
    Disabled,
}

pub async fn app(pool: PgPool, with_tracing: Tracing) -> Router {
    let mut secret: [u8; 64] = [0; 64];
    secret.copy_from_slice(
        env::var("APP_SECRET")
            .expect("App Secret is either undefined or not exactly 64 char long!")
            .as_bytes(),
    );

    let session_duration_minutes = env::var("SESSION_LIFETIME_MINUTES")
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .expect("Invalid session lifetime; can't convert to numeric value");

    let session_store = CookieStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_persistence_policy(PersistencePolicy::ExistingOnly)
        .with_session_ttl(Some(Duration::from_secs(session_duration_minutes * 60)))
        .with_same_site_policy(SameSite::Strict)
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
        // ↑ authenticated views go above
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // ↓ public views go below
        .nest("/auth", authn::routes())
        .route_layer(middleware::from_fn(set_security_headers))
        .route_layer(middleware::from_fn(route_auth_guard))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(pool.clone()))
        .layer(RequestIdLayer)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower::ServiceBuilder::new().concurrency_limit(32))
        .fallback(handler_404);

    match with_tracing {
        Tracing::Enabled => {
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
        Tracing::Disabled => (),
    }

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

    opentelemetry::global::shutdown_tracer_provider();
}
