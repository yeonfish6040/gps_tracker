pub mod v1;

use axum::{Router, middleware, routing::get};

pub fn router() -> Router<crate::app::AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/v1", v1::router())
        .layer(middleware::from_fn(crate::middleware::request_logger))
}

async fn root() -> &'static str {
    "axum server is running"
}
