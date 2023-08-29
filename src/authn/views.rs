use std::convert::TryFrom;

use axum::{Extension, Form};
use axum::extract::Query;
use axum::response::{IntoResponse, Redirect, Response};
use bcrypt::hash_with_salt;
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::AuthContext;
use crate::authn::models::{Credentials, User};
use crate::templates::render;
use crate::deserialization::empty_string_as_none;

pub mod login;

#[derive(Deserialize)]
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    message: Option<String>,
}

pub async fn signup_view(
    Query(params): Query<Params>,
) -> impl IntoResponse {
    render(SignupTemplate { params })
}

pub async fn signup_handler(
    Extension(pool): Extension<SqlitePool>,
    Form(signup): Form<Credentials>,
) -> impl IntoResponse {
    let salt = "1234567890123456".as_bytes(); // TODO: Use app secret
    let password_hash = hash_with_salt(
        signup.password,
        12,
        <[u8; 16]>::try_from(salt).unwrap(),
    );
    let mut connection = match pool.acquire().await {
        Ok(pool) => pool,
        Err(_) => return Redirect::to("signup?message=error"),
    };
    let query = match password_hash {
        Ok(hash) => {
            let password_hash = hash.to_string();
            sqlx::query("INSERT INTO users (name, password_hash) VALUES (?, ?);")
                .bind(signup.username)
                .bind(password_hash)
        }
        Err(_) => return Redirect::to("/signup?message=error")
    };

    match query.execute(&mut connection).await {
        Ok(_) => Redirect::to("/login?message=success"),
        Err(_) => Redirect::to("/signup?message=error")
    }
}

pub async fn logged_in_view(
    auth: AuthContext,
) -> impl IntoResponse {
    render(GreetingsTemplate { user: auth.current_user.unwrap().to_owned() })
}

pub async fn logout_handler(mut auth: AuthContext) -> Response {
    auth.logout().await;
    Redirect::to("/login?message=logout").into_response()
}

#[derive(TemplateOnce)]
#[template(path = "signup.html")]
struct SignupTemplate {
    params: Params,
}

#[derive(TemplateOnce)]
#[template(path = "logged_in.html")]
struct GreetingsTemplate {
    user: User,
}