use crate::environment::read_mandatory_env_var;
use dotenv::dotenv;
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres};

pub type DatabaseConnection = PoolConnection<Postgres>;

/// Try to be mindful of the usage.
/// Usually you'd want Extension(pool): Extension<PgPool>
/// in views/handlers.
///
/// If this is used in too many places (more than 1?) then
/// pooling won't work properly and postgres will run out
/// of connections to give.
///
#[allow(clippy::expect_used)]
pub async fn get_pool() -> PgPool {
    dotenv().ok();

    PgPoolOptions::new()
        .max_connections(90)
        .connect_lazy(read_mandatory_env_var("DATABASE_URL").as_str())
        .expect("Can't connect to the Database!")
}
