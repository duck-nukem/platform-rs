use axum::{
    http::{header::LOCATION, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn route_auth_guard<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;
    let is_unauthenticated = response.status() == StatusCode::UNAUTHORIZED;
    let is_unauthorized = response.status() == StatusCode::FORBIDDEN;
    let is_required_auth_missing_or_invalid = is_unauthenticated || is_unauthorized;

    if is_required_auth_missing_or_invalid {
        *response.status_mut() = StatusCode::FOUND;
        response.headers_mut().insert(
            LOCATION,
            HeaderValue::from_static("login?message=auth_required"),
        );
    }

    Ok(response)
}
