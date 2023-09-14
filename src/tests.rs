use axum_test::TestServer;
use sqlx::PgPool;

#[cfg(test)]
pub async fn make_server(pool: PgPool) -> TestServer {
    use crate::{app, Tracing};

    TestServer::new(app(pool, Tracing::Disabled).await.into_make_service()).unwrap()
}
