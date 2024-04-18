use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::IntoResponse,
};

use crate::{settings::Site, HtmlTemplate};

/// Handler for the 404 page. The template is rendered using the `NotFound`
/// struct which is passed to the [`HtmlTemplate`] struct. The template is
/// defined in `templates/404.html`.
pub async fn not_found(uri: Uri, State(site): State<Arc<Site>>) -> impl IntoResponse {
    let template = NotFound { uri, site };
    (StatusCode::NOT_FOUND, HtmlTemplate::new(template))
}

/// The `NotFound` struct is used to render the 404 page template.
#[derive(Template)]
#[template(path = "404.html")]
struct NotFound {
    uri: Uri,
    site: Arc<Site>,
}
