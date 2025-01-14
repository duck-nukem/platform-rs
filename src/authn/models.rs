use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct NewUser {
    pub raw_password: String,
    pub name: String,
}

#[derive(Debug, Default, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub password_hash: String,
    pub name: String,
    pub locale: String,
}

pub static ERROR_USER: Lazy<User> = Lazy::new(|| User {
    id: -1,
    password_hash: String::new(),
    name: String::from("Unknown"),
    locale: String::from("en"),
});

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
