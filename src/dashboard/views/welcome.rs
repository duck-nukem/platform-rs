use crate::authn::models::{User, ERROR_USER};
use crate::templates::render;
use crate::AuthContext;
use axum::body::Body;
use axum::http::Request;
use axum::response::Html;
use sailfish::TemplateOnce;
use tower_request_id::RequestId;
use tracing::instrument;

#[instrument]
pub async fn greet_user(auth: AuthContext, req: Request<Body>) -> Html<String> {
    let request_id = &*req
        .extensions()
        .get::<RequestId>()
        .map_or_else(|| "INVALID REQUEST ID!".to_owned(), ToString::to_string);
    render(GreetingsTemplate {
        user: auth.current_user.unwrap_or_else(|| ERROR_USER.clone()),
        nonce: request_id.to_string(),
    })
}

#[derive(TemplateOnce)]
#[template(path = "logged_in.html")]
struct GreetingsTemplate {
    user: User,
    nonce: String,
}
