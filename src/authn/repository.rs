use bcrypt::hash_with_salt;
use sqlx::postgres::PgRow;
use sqlx::Error;
use tracing::instrument;

use crate::authn::models::{NewUser, User};
use crate::database::DatabaseConnection;
use crate::environment::read_mandatory_env_var;

#[instrument]
pub async fn find_by_username(
    mut connection: DatabaseConnection,
    username: &str,
) -> Result<User, Error> {
    sqlx::query_as("SELECT * FROM users WHERE name = $1 LIMIT 1;")
        .bind(username)
        .fetch_one(connection.as_mut())
        .await
}

#[instrument]
pub async fn create_user(
    mut connection: DatabaseConnection,
    user: NewUser,
) -> Result<Option<PgRow>, Error> {
    let mut salt: [u8; 16] = Default::default();
    salt.copy_from_slice(read_mandatory_env_var("PASSWORD_SALT").as_bytes());
    let password_hash = hash_with_salt(user.raw_password, 10, salt).unwrap();
    sqlx::query(
        "INSERT INTO users (name, password_hash, locale) VALUES ($1, $2, 'en') RETURNING id;",
    )
    .bind(user.name)
    .bind(password_hash.to_string())
    .fetch_optional(connection.as_mut())
    .await
}
