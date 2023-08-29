use sqlx::{Sqlite, SqlitePool};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteConnectOptions;

pub type DatabaseConnection = PoolConnection<Sqlite>;

pub async fn get_connection() -> DatabaseConnection {
    let options = SqliteConnectOptions::new()
        .filename("test_sqlite.db")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.unwrap();

    pool.acquire().await.unwrap()
}
