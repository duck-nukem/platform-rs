use axum::{Extension, Form};
use axum::response::{IntoResponse, Redirect, Response};
use bcrypt::verify;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
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

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
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

        assert_eq!(response.header("Location"), "/signup?reason=invalid")
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

        assert_eq!(response.header("Location"), "/signup?reason=error")
    }
}