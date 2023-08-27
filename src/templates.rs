use axum::response::{Html, IntoResponse};
use sailfish::TemplateOnce;

pub fn render(template: impl TemplateOnce) -> impl IntoResponse {
    Html(template.render_once().expect("Failed to render template"))
}