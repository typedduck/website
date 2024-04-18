use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

/// A wrapper around a template that can be converted into a response.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct HtmlTemplate<T>(T);

impl<T> HtmlTemplate<T> {
    /// Create a new `HtmlTemplate` instance.
    #[inline]
    #[must_use]
    pub const fn new(template: T) -> Self {
        Self(template)
    }
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
