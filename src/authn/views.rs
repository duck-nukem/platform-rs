use std::convert::TryFrom;

use axum::{Extension, Form};
use axum::body::HttpBody;
use axum::extract::Query;
use axum::response::{IntoResponse, Redirect, Response};
use bcrypt::{hash_with_salt, verify};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};

use crate::AuthContext;
use crate::authn::models::User;
use crate::templates::render;

#[derive(Deserialize)]
pub struct Params {
    message: String,
}

pub async fn login_view(
    Query(params): Query<Params>,
) -> impl IntoResponse {
    render(LoginTemplate { params })
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

pub async fn login_handler(
    mut auth: AuthContext,
    Extension(pool): Extension<SqlitePool>,
    Form(login): Form<Credentials>,
) -> impl IntoResponse {
    let mut conn = pool.acquire().await.unwrap();
    let user_query: Result<User, Error> = sqlx::query_as("select * from users where name = $1;")
        .bind(login.username)
        .fetch_one(&mut conn)
        .await;

    let user = match user_query {
        Ok(found_user) => found_user,
        Err(_) => {
            /*
            If the user doesn't exist we simulate a password verification
            to get the same response time as if there was a match.

            This is required to avoid account enumeration by inspecting the response times.
             */
            let _ = verify(
                login.password.clone().as_str(),
                "$2y$10$tfFECZbEbCSq1.xBBK5nrOUWbpR2bQig/5T0/SjuEvpY5Diaonk9u", // "password" with cost 10
            );
            return Redirect::to("/login?message=invalid").into_response();
        }
    };

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
                Redirect::to("/login?message=invalid").into_response()
            }
        }
        Err(_password_error) => Redirect::to("/login?message=error").into_response(),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(TemplateOnce)]
#[template(path = "login.html")]
struct LoginTemplate {
    params: Params,
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

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use axum_test::TestServer;
    use bcrypt::hash_with_salt;
    use sqlx::query;
    use sqlx::sqlite::SqlitePoolOptions;

    use crate::app;
    use crate::authn::views::Credentials;

    #[tokio::test]
    async fn test_login_handler_should_redirect_if_user_is_not_found() {
        let server = TestServer::new(app().await.into_make_service()).unwrap();

        let response = server.post("/login")
            .form(&Credentials { username: "".into(), password: "".into() })
            .await;

        assert_eq!(response.header("Location"), "/login?message=invalid")
    }

    #[tokio::test]
    async fn test_login_handler_should_redirect_if_password_is_invalid() {
        // TODO: Define separate test DB
        let pool = SqlitePoolOptions::new().connect("sqlite.db").await.unwrap();
        let query = query!("INSERT INTO users (name, password_hash) VALUES (?, ?);", "user", "no_hash");
        query.execute(&pool).await.unwrap();
        let server = TestServer::new(app().await.into_make_service()).unwrap();

        let response = server.post("/login")
            .form(&Credentials { username: "user".into(), password: "no_hash".into() })
            .await;

        assert_eq!(response.header("Location"), "/login?message=error")
    }

    #[tokio::test]
    async fn test_login_handler_should_redirect_if_credentials_are_valid() {
        // TODO: Define separate test DB
        let pool = SqlitePoolOptions::new().connect("sqlite.db").await.unwrap();
        let salt = "1234567890123456".as_bytes(); // TODO: Use app secret
        let password_hash = hash_with_salt(
            "password",
            12,
            <[u8; 16]>::try_from(salt).unwrap(),
        ).unwrap().to_string();
        let query = query!("INSERT INTO users (name, password_hash) VALUES (?, ?);", "valid_user", password_hash);
        query.execute(&pool).await.unwrap();
        let server = TestServer::new(app().await.into_make_service()).unwrap();

        let response = server.post("/login")
            .form(&Credentials { username: "valid_user".into(), password: "password".into() })
            .await;

        assert_eq!(response.header("Location"), "/greet")
    }
}