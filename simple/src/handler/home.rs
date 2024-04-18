use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::{settings::Site, HtmlTemplate};

/// Handler for the home page. The template is rendered using the `Home` struct
/// which is passed to the [`HtmlTemplate`] struct. The template is defined in
/// `templates/index.html`.
pub async fn home(State(site): State<Arc<Site>>) -> impl IntoResponse {
    let template = Home { site };
    HtmlTemplate::new(template)
}

/// The `Home` struct is used to render the home page template.
#[derive(Template)]
#[template(path = "index.html")]
struct Home {
    site: Arc<Site>,
}
