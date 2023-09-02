use crate::authn::models::{Credentials, User};
use crate::database::get_connection;
use crate::deserialization::empty_string_as_none;
use crate::templates::render;
use crate::AuthContext;
use axum::body::Body;
use axum::extract::Query;
use axum::http::Request;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use sailfish::TemplateOnce;
use serde::Deserialize;
use tower_request_id::RequestId;

use super::models::NewUser;
use super::repository::create_user;

pub mod login;

#[derive(Deserialize)]
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    message: Option<String>,
}

pub async fn signup_view(Query(params): Query<Params>) -> impl IntoResponse {
    render(SignupTemplate { params })
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

pub async fn logged_in_view(auth: AuthContext, req: Request<Body>) -> impl IntoResponse {
    let request_id = &*req
        .extensions()
        .get::<RequestId>()
        .map(ToString::to_string)
        .unwrap();
    render(GreetingsTemplate {
        user: auth.current_user.unwrap().to_owned(),
        nonce: request_id.to_string(),
    })
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
    nonce: String,
}
