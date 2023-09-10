use axum::response::Html;
use sailfish::TemplateOnce;

pub fn render(template: impl TemplateOnce) -> Html<String> {
    Html(template.render_once().expect("Failed to render template"))
}
