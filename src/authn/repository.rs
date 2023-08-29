use sqlx::Error;
use sqlx::sqlite::SqliteRow;

use crate::authn::models::{NewUser, User};
use crate::database::DatabaseConnection;

pub async fn find_by_username(mut connection: DatabaseConnection, username: &str) -> Result<User, Error> {
    sqlx::query_as("select * from users where name = $1;")
        .bind(username)
        .fetch_one(connection.as_mut())
        .await
}

pub async fn create_user(mut connection: DatabaseConnection, user: NewUser) -> Result<Option<SqliteRow>, Error> {
    sqlx::query("INSERT INTO users (name, password_hash) VALUES (?, ?) RETURNING id;")
        .bind(user.name)
        .bind(user.password_hash)
        .fetch_optional(connection.as_mut())
        .await
}