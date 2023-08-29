use dotenv::dotenv;
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgConnectOptions;
use sqlx::{PgPool, Postgres};
use std::env;

pub type DatabaseConnection = PoolConnection<Postgres>;

pub async fn get_pool() -> PgPool {
    dotenv().ok();

    let options = PgConnectOptions::new()
        .host(
            env::var("DB_HOST")
                .unwrap_or_else(|_| String::from("localhost"))
                .as_str(),
        )
        .port(
            env::var("DB_PORT")
                .unwrap_or_else(|_| String::from("5432"))
                .parse::<u16>()
                .unwrap(),
        )
        .username(env::var("DB_USER").unwrap().as_str())
        .password(env::var("DB_PASS").unwrap().as_str())
        .database(env::var("DB_NAME").unwrap().as_str());
    PgPool::connect_with(options).await.unwrap()
}

pub async fn get_connection() -> DatabaseConnection {
    get_pool().await.acquire().await.unwrap()
}
