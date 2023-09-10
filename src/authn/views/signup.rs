use crate::authn::models::{Credentials, NewUser};
use crate::authn::repository::create_user;
use crate::database::get_connection;
use crate::templates::render;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use sailfish::TemplateOnce;
use tracing::instrument;

#[instrument]
pub async fn signup_view() -> Html<String> {
    render(SignupTemplate {})
}

#[instrument]
pub async fn signup_handler(Form(signup): Form<Credentials>) -> impl IntoResponse {
    let created_user = create_user(
        get_connection().await,
        NewUser {
            name: signup.username,
            raw_password: signup.password,
        },
    )
    .await;
    match created_user {
        Ok(_) => Redirect::to("/login?message=success"),
        Err(_) => Redirect::to("/signup?message=error"),
    }
}

#[derive(TemplateOnce)]
#[template(path = "signup.html")]
pub struct SignupTemplate {}
