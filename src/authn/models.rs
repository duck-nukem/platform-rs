use axum_login::AuthUser;
use axum_login::secrecy::SecretVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize)]
pub struct NewUser {
    pub password_hash: String,
    pub name: String,
}

#[derive(Debug, Default, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub password_hash: String,
    pub name: String,
}

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
