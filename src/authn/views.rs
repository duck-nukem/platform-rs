use axum::body::Body;
use axum::http::Request;
use axum::response::IntoResponse;
use sailfish::TemplateOnce;
use tower_request_id::RequestId;

use crate::authn::models::User;
use crate::templates::render;
use crate::AuthContext;

pub mod auth;
pub mod signup;

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

#[derive(TemplateOnce)]
#[template(path = "logged_in.html")]
struct GreetingsTemplate {
    user: User,
    nonce: String,
}
