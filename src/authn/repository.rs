use std::env;

use bcrypt::hash_with_salt;
use sqlx::postgres::PgRow;
use sqlx::Error;

use crate::authn::models::{NewUser, User};
use crate::database::DatabaseConnection;

pub async fn find_by_username(
    mut connection: DatabaseConnection,
    username: &str,
) -> Result<User, Error> {
    sqlx::query_as("select * from users where name = $1;")
        .bind(username)
        .fetch_one(connection.as_mut())
        .await
}

pub async fn create_user(
    mut connection: DatabaseConnection,
    user: NewUser,
) -> Result<Option<PgRow>, Error> {
    let mut salt: [u8; 16] = Default::default();
    salt.copy_from_slice(
        env::var("PASSWORD_SALT")
            .expect("Password Salt is either undefined or not 16 chars long!")
            .as_bytes(),
    );
    let password_hash = hash_with_salt(user.raw_password, 10, salt).unwrap();
    sqlx::query(
        "INSERT INTO users (name, password_hash, locale) VALUES ($1, $2, 'en') RETURNING id;",
    )
    .bind(user.name)
    .bind(password_hash.to_string())
    .fetch_optional(connection.as_mut())
    .await
}
