use crate::authn::models::{Credentials, NewUser};
use crate::database::get_connection;
use crate::templates::render;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use sailfish::TemplateOnce;
use crate::authn::repository::create_user;

pub async fn signup_view() -> impl IntoResponse {
    render(SignupTemplate {})
}

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
