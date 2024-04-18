use axum::{routing::get, Router};

use crate::AppState;

mod home;
pub use home::home;

mod not_found;
pub use not_found::not_found;

/// Initialize the routes for the application.
#[allow(clippy::needless_pass_by_value)]
pub fn init_routes(state: AppState) -> Router {
    let router = Router::new();

    router
        .route("/", get(home))
        .fallback(not_found)
        .with_state(state.site())
}
