use axum::{Extension, Form};
use axum::response::{IntoResponse, Redirect, Response};
use bcrypt::verify;
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::Error;
use sqlx::sqlite::SqlitePoolOptions;

use crate::AuthContext;
use crate::authn::models::User;
use crate::templates::render;

pub async fn login_view() -> impl IntoResponse {
    render(LoginTemplate {})
}

pub async fn logged_in_view(
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    render(GreetingsTemplate { user: user.to_owned() })
}

pub async fn login_handler(mut auth: AuthContext, Form(login): Form<UserSignup>) -> impl IntoResponse {
    let pool = SqlitePoolOptions::new().connect("sqlite.db").await.unwrap();
    let mut conn = pool.acquire().await.unwrap();
    let user_query: Result<User, Error> = sqlx::query_as("select * from users where id = 1;")
        .fetch_one(&mut conn)
        .await;

    let user = match user_query {
        Ok(found_user) => found_user,
        Err(_) => return Redirect::to("/signup?reason=invalid").into_response()
    };

    // for registration?
    // salt.copy_from_slice("1234567890123456".as_bytes());
    let verified_password = verify(
        login.password.clone().as_str(),
        user.password_hash.clone().as_str(),
    );
    match verified_password {
        Ok(is_valid_password_for_user) => {
            if is_valid_password_for_user {
                auth.login(&user).await.unwrap();
                Redirect::to("/greet").into_response()
            } else {
                Redirect::to("/signup?reason=invalid").into_response()
            }
        }
        Err(_password_error) => Redirect::to("/signup?reason=error").into_response(),
    }
}

pub async fn logout_handler(mut auth: AuthContext) -> Response {
    auth.logout().await;
    Redirect::to("/signup?reason=invalid").into_response()
}

#[derive(Deserialize, Debug)]
pub struct UserSignup {
    pub username: String,
    pub password: String,
}

#[derive(TemplateOnce)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[derive(TemplateOnce)]
#[template(path = "logged_in.html")]
struct GreetingsTemplate {
    user: User,
}
