use std::convert::TryInto;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use axum::{middleware, Extension, Router};
use axum_login::axum_sessions::async_session::SessionStore;
use axum_login::axum_sessions::{PersistencePolicy, SameSite, SessionLayer};
use axum_login::{AuthLayer, SqlxStore};
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use sqlx::PgPool;

use crate::authn::models::User;
use crate::environment::{
    read_bool_env_var, read_env_var, read_mandatory_env_var, read_numeric_env_var,
};
use crate::http::{handler_404, route_auth_guard, set_security_headers};

pub type AppSecret = [u8; 64];

pub fn get_app_secret() -> AppSecret {
    let secret = read_mandatory_env_var("APP_SECRET");
    let mut secret_bytes: AppSecret = [0; 64];
    secret_bytes.copy_from_slice(secret.as_bytes());

    secret_bytes
}

pub fn build_session_layer<T: SessionStore>(
    session_storage: T,
    secret: &AppSecret,
) -> SessionLayer<impl SessionStore + use<T>> {
    let session_duration_minutes = read_numeric_env_var("SESSION_LIFETIME_MINUTES", &10);
    let is_secure_cookie = read_bool_env_var("SECURE_COOKIE", true);

    SessionLayer::new(session_storage, secret)
        .with_persistence_policy(PersistencePolicy::ExistingOnly)
        .with_session_ttl(Some(Duration::from_secs(session_duration_minutes * 60)))
        .with_same_site_policy(SameSite::Strict)
        .with_http_only(true)
        .with_secure(is_secure_cookie)
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
    let max_concurrency_limit = read_numeric_env_var("MAX_CONCURRENCY_LIMIT", &32usize);

    router_with_app_routes
        .route_layer(middleware::from_fn(set_security_headers))
        .route_layer(middleware::from_fn(route_auth_guard))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(database_pool))
        .layer(tower_request_id::RequestIdLayer)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower::ServiceBuilder::new().concurrency_limit(max_concurrency_limit))
        .fallback(handler_404)
}

#[allow(clippy::expect_used)]
pub fn build_socket_from_ip_port(ipv4_address: &str, port: u16) -> SocketAddr {
    let octets: [u8; 4] = ipv4_address
        .split('.')
        .map(|o| {
            o.parse::<u8>()
                .expect("Can't parse octet to numeric format")
        })
        .collect::<Vec<u8>>()
        .try_into()
        .expect("IPv4 address must have exactly 4 octets");
    let ip = IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));

    SocketAddr::new(ip, port)
}

#[allow(clippy::unused_async)]
pub async fn configure_tracing() {
    let tracer_connection_url = read_env_var("TRACER_CONNECTION_URL", "http://jaeger:4317/");
    let _ = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(tracer_connection_url),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "platform-rs",
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio);
}
