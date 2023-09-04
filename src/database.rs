use std::env;

use dotenv::dotenv;
use sqlx::pool::PoolConnection;
use sqlx::{PgPool, Postgres};

pub type DatabaseConnection = PoolConnection<Postgres>;

pub async fn get_pool() -> PgPool {
    dotenv().ok();

    PgPool::connect_lazy(
        env::var("DATABASE_URL")
            .expect("DATABASE_URL envvar is undefined, can't connect to DB!")
            .as_str(),
    )
    .expect("Can't connect to the Database!")
}

pub async fn get_connection() -> DatabaseConnection {
    get_pool().await.acquire().await.unwrap()
}
