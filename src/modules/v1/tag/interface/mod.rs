pub mod auth;
pub mod tracking;

use axum::Router;

pub fn router() -> Router<crate::app::AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/tracking", tracking::router())
}