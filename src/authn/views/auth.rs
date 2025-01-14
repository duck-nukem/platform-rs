use axum::extract::Query;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Extension, Form};
use bcrypt::verify;
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::instrument;

use crate::authn::models::{Credentials, User};
use crate::authn::repository::find_by_username;
use crate::authn::routes::AuthRoute;
use crate::dashboard::routes::DashboardRoute;
use crate::deserialization::empty_string_as_none;
use crate::routing::{build_url, Prefix, QueryParams};
use crate::templates::render;
use crate::AuthContext;

#[instrument]
pub async fn login_view(Query(params): Query<Params>) -> Html<String> {
    render(LoginTemplate {
        message: params.message.unwrap_or_else(String::new),
    })
}

async fn perform_login(mut auth: AuthContext, is_correct_password: bool, user: &User) -> Response {
    let error_response = Redirect::to(
        build_url(
            Prefix::Nested("auth"),
            AuthRoute::Login,
            QueryParams::From(vec![("message".to_string(), "invalid".to_string())]),
        )
        .as_str(),
    )
    .into_response();

    if !is_correct_password {
        return error_response;
    }

    match auth.login(user).await {
        Ok(()) => Redirect::to(
            build_url(Prefix::Root, DashboardRoute::Greetings, QueryParams::None).as_str(),
        )
        .into_response(),
        Err(_) => error_response,
    }
}

#[instrument]
pub async fn login_handler(
    auth: AuthContext,
    Extension(pool): Extension<PgPool>,
    Form(login): Form<Credentials>,
) -> impl IntoResponse {
    let Ok(connection) = pool.acquire().await else {
        return Redirect::to(
            build_url(
                Prefix::Nested("auth"),
                AuthRoute::Login,
                QueryParams::From(vec![("message".to_string(), "db_error".to_string())]),
            )
            .as_str(),
        )
        .into_response();
    };
    let user_query = find_by_username(connection, login.username.as_str()).await;
    let Ok(user) = user_query else {
        /*
        If the user doesn't exist we simulate a password verification
        to get the same response time as if there was a match.

        This is required to avoid account enumeration by inspecting the response times.
         */
        let _ = verify(
            login.password.clone().as_str(),
            "$2y$10$tfFECZbEbCSq1.xBBK5nrOUWbpR2bQig/5T0/SjuEvpY5Diaonk9u", // "password" with cost 10
        );
        return Redirect::to(
            build_url(
                Prefix::Nested("auth"),
                AuthRoute::Login,
                QueryParams::From(vec![("message".to_string(), "invalid".to_string())]),
            )
            .as_str(),
        )
        .into_response();
    };

    let verified_password = verify(
        login.password.clone().as_str(),
        user.password_hash.clone().as_str(),
    );
    match verified_password {
        Ok(is_valid_password_for_user) => {
            perform_login(auth, is_valid_password_for_user, &user).await
        }
        Err(_password_error) => Redirect::to(
            build_url(
                Prefix::Nested("auth"),
                AuthRoute::Login,
                QueryParams::From(vec![("message".to_string(), "error".to_string())]),
            )
            .as_str(),
        )
        .into_response(),
    }
}

pub async fn logout_handler(mut auth: AuthContext) -> Response {
    auth.logout().await;
    Redirect::to(
        build_url(
            Prefix::Nested("auth"),
            AuthRoute::Login,
            QueryParams::From(vec![("message".to_string(), "logged_out".to_string())]),
        )
        .as_str(),
    )
    .into_response()
}

#[derive(Deserialize, Debug)]
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    message: Option<String>,
}

#[derive(TemplateOnce)]
#[template(path = "login.html")]
struct LoginTemplate {
    message: String,
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::authn::models::{Credentials, NewUser};
    use crate::authn::repository::create_user;
    use crate::authn::routes::AuthRoute;
    use crate::routing::{build_url, Prefix, QueryParams};
    use crate::tests::make_server;

    #[sqlx::test]
    async fn test_login_handler_should_redirect_if_user_is_not_found(pool: PgPool) {
        let server = make_server(pool.clone()).await;
        let url = build_url(Prefix::Nested("auth"), AuthRoute::Login, QueryParams::None);

        let response = server
            .post(url.as_str())
            .form(&Credentials {
                username: "".into(),
                password: "".into(),
            })
            .await;

        let expected_redirection_url = build_url(
            Prefix::Nested("auth"),
            AuthRoute::Login,
            QueryParams::From(vec![("message".to_string(), "invalid".to_string())]),
        );
        assert_eq!(
            response.header("Location"),
            expected_redirection_url.as_str()
        );
    }

    #[sqlx::test]
    async fn test_login_handler_should_redirect_if_password_is_invalid(pool: PgPool) {
        create_user(
            pool.acquire().await.unwrap(),
            NewUser {
                name: "user".into(),
                raw_password: "password".into(),
            },
        )
        .await
        .unwrap();
        let server = make_server(pool.clone()).await;
        let url = build_url(Prefix::Nested("auth"), AuthRoute::Login, QueryParams::None);

        let response = server
            .post(url.as_str())
            .form(&Credentials {
                username: "user".into(),
                password: "wrong_password".into(),
            })
            .await;

        let expected_redirection_url = build_url(
            Prefix::Nested("auth"),
            AuthRoute::Login,
            QueryParams::From(vec![("message".to_string(), "invalid".to_string())]),
        );
        assert_eq!(response.header("Location"), expected_redirection_url)
    }

    #[sqlx::test]
    async fn test_login_handler_should_redirect_if_credentials_are_valid(pool: PgPool) {
        create_user(
            pool.acquire().await.unwrap(),
            NewUser {
                name: "valid_user".into(),
                raw_password: "password".into(),
            },
        )
        .await
        .unwrap();
        let server = make_server(pool.clone()).await;
        let url = build_url(Prefix::Nested("auth"), AuthRoute::Login, QueryParams::None);

        let response = server
            .post(url.as_str())
            .form(&Credentials {
                username: "valid_user".into(),
                password: "password".into(),
            })
            .await;

        assert_eq!(response.header("Location"), "/greet")
    }
}
