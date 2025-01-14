use crate::authn::models::{Credentials, NewUser};
use crate::authn::repository::create_user;
use crate::templates::render;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{Extension, Form};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use tracing::instrument;

#[instrument]
pub async fn signup_view() -> Html<String> {
    render(SignupTemplate {})
}

#[instrument]
pub async fn signup_handler(
    Extension(pool): Extension<PgPool>,
    Form(signup): Form<Credentials>,
) -> impl IntoResponse {
    let Ok(dbpool) = pool.acquire().await else {
        return Redirect::to("/auth/signup?message=error");
    };
    let created_user = create_user(
        dbpool,
        NewUser {
            name: signup.username,
            raw_password: signup.password,
        },
    )
    .await;
    match created_user {
        Ok(_) => Redirect::to("/auth/login?message=success"),
        Err(_) => Redirect::to("/auth/signup?message=error"),
    }
}

#[derive(TemplateOnce)]
#[template(path = "signup.html")]
pub struct SignupTemplate {}
