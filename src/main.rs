#[macro_use]
extern crate rust_i18n;

use axum::Router;
use axum_login::{PostgresStore, RequireAuthorizationLayer};
use dotenv::dotenv;
use sqlx::PgPool;

use crate::authn::models::User;
use database::get_pool;

use crate::bootstrap::build_socket_from_ip_port;
use crate::environment::{read_env_var, read_numeric_env_var};
use crate::session::CookieStore;

mod authn;
mod bootstrap;
mod dashboard;
mod database;
mod deserialization;
mod environment;
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
    let app_router = Router::new()
        .nest("/", dashboard::routes::routes())
        // ↑ authenticated views go above
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        // ↓ public views go below
        .nest("/auth", authn::routes::routes());

    let user_store = PostgresStore::<User>::new(pool.clone());
    let session_store = CookieStore::new();

    bootstrap::configure_auxiliary_routing(app_router, user_store, session_store, pool.clone())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    let server_socket_address = build_socket_from_ip_port(
        read_env_var("SERVER_HOST", "0.0.0.0"),
        read_numeric_env_var("SERVER_PORT", 3000u16),
    );
    println!("Ready to accept connections at {}", server_socket_address);

    axum::Server::bind(&server_socket_address)
        .serve(
            app(get_pool().await, Tracing::Enabled)
                .await
                .into_make_service(),
        )
        .await
        .unwrap();
}
