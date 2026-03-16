pub mod v1;

use axum::{Router, routing::get};

pub fn router() -> Router<crate::app::AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/v1", v1::router())
}

async fn root() -> &'static str {
    "axum server is running"
}
