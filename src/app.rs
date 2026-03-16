use axum::http::StatusCode;
use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn connect_database() -> DatabaseConnection {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set before starting the server");

    Database::connect(&database_url)
        .await
        .unwrap_or_else(|error| panic!("failed to connect to database: {error}"))
}

pub fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
