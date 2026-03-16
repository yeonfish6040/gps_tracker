use axum::{
    Router,
    routing::post,
};
use crate::{app::{AppState}, modules};
use modules::v1::tag::service::tracking;


pub fn router() -> Router<AppState> {
    Router::new().route("/push", post(tracking::push))
}