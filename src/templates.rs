use axum::response::Html;
use sailfish::TemplateOnce;

pub fn render(template: impl TemplateOnce) -> Html<String> {
    Html(
        template
            .render_once()
            .unwrap_or_else(|_| "Failed to render template".to_owned()),
    )
}
