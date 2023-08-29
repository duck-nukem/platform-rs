use axum::extract::Query;
use axum::Form;
use axum::response::{IntoResponse, Redirect};
use bcrypt::verify;
use sailfish::TemplateOnce;

use crate::AuthContext;
use crate::authn::models::Credentials;
use crate::authn::repository::find_by_username;
use crate::authn::views::Params;
use crate::database::get_connection;
use crate::templates::render;

pub async fn login_view(
    Query(params): Query<Params>,
) -> impl IntoResponse
{
    render(LoginTemplate { message: params.message.unwrap_or("".parse().unwrap()) })
}


pub async fn login_handler(
    mut auth: AuthContext,
    Form(login): Form<Credentials>,
) -> impl IntoResponse {
    let user_query = find_by_username(get_connection().await, login.username.as_str()).await;
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

#[derive(TemplateOnce)]
#[template(path = "login.html")]
struct LoginTemplate {
    message: String,
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use axum_test::TestServer;
    use bcrypt::hash_with_salt;

    use crate::app;
    use crate::authn::models::{Credentials, NewUser};
    use crate::authn::repository::create_user;
    use crate::database::get_connection;

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
        create_user(
            get_connection().await,
            NewUser { name: "user".into(), password_hash: "no_hash".into() },
        ).await.unwrap();
        let server = TestServer::new(app().await.into_make_service()).unwrap();

        let response = server.post("/login")
            .form(&Credentials { username: "user".into(), password: "no_hash".into() })
            .await;

        assert_eq!(response.header("Location"), "/login?message=error")
    }

    #[tokio::test]
    async fn test_login_handler_should_redirect_if_credentials_are_valid() {
        let salt = "1234567890123456".as_bytes(); // TODO: Use app secret
        let password_hash = hash_with_salt(
            "password",
            12,
            <[u8; 16]>::try_from(salt).unwrap(),
        ).unwrap().to_string();
        create_user(
            get_connection().await,
            NewUser { name: "user".into(), password_hash },
        ).await.unwrap();
        let server = TestServer::new(app().await.into_make_service()).unwrap();

        let response = server.post("/login")
            .form(&Credentials { username: "valid_user".into(), password: "password".into() })
            .await;

        assert_eq!(response.header("Location"), "/greet")
    }
}
