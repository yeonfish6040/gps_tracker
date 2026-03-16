pub mod tag;

use axum::Router;

pub fn router() -> Router<crate::app::AppState> {
    Router::new()
        .nest("/tag", tag::interface::router())
}
