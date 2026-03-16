use axum::{
    Router,
    routing::post,
};
use crate::{app::{AppState}, modules};
use modules::v1::tag::service::auth;


pub fn router() -> Router<AppState> {
    Router::new().route("/", post(auth::register_tag))
}