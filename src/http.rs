use crate::templates::render;
use axum::response::IntoResponse;
use axum::{
    http::{
        header::{CONTENT_SECURITY_POLICY, LOCATION},
        HeaderValue, Request, StatusCode,
    },
    middleware::Next,
    response::Response,
};
use sailfish::TemplateOnce;
use tower_request_id::RequestId;

#[allow(clippy::future_not_send)]
pub async fn route_auth_guard<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;
    let is_unauthenticated = response.status() == StatusCode::UNAUTHORIZED;
    let is_unauthorized = response.status() == StatusCode::FORBIDDEN;
    let is_required_auth_missing_or_invalid = is_unauthenticated || is_unauthorized;

    if is_required_auth_missing_or_invalid {
        *response.status_mut() = StatusCode::FOUND;
        response.headers_mut().insert(
            LOCATION,
            HeaderValue::from_static("/auth/login?message=auth_required"),
        );
    }

    Ok(response)
}

#[allow(clippy::future_not_send)]
#[allow(clippy::expect_used)]
pub async fn set_security_headers<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let request_id = &*req
        .extensions()
        .get::<RequestId>()
        .map(ToString::to_string)
        .expect("Can't set security headers due to a missing or invalid request id");
    let mut response = next.run(req).await;
    let header_value =
        format!("object-src 'none'; base-uri 'none'; script-src 'nonce-{request_id}'");
    response.headers_mut().insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_bytes(header_value.as_bytes())
            .expect("Unable to construct security headers"),
    );

    Ok(response)
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, render(NotFoundTemplate {}))
}

#[derive(TemplateOnce)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {}
