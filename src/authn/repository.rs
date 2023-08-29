use sqlx::Error;

use crate::authn::models::User;
use crate::database::DatabaseConnection;

pub async fn find_by_username(mut connection: DatabaseConnection, username: &str) -> Result<User, Error> {
    sqlx::query_as("select * from users where name = $1;")
        .bind(username)
        .fetch_one(connection.as_mut())
        .await
}
