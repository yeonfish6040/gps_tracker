mod entities;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use dotenvy::dotenv;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, DbBackend, Set, Statement,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use std::env;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[derive(Deserialize)]
struct CreateDeviceRequest {
    name: String,
}

#[derive(Serialize)]
struct DeviceResponse {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = connect_database().await;
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/devices", post(create_device))
        .with_state(AppState { db });
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|error| panic!("failed to bind to {bind_addr}: {error}"));

    println!("listening on http://{bind_addr}");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn root() -> &'static str {
    "axum server is running"
}

async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, (StatusCode, String)> {
    state
        .db
        .execute(Statement::from_string(
            DbBackend::Postgres,
            "SELECT 1".to_owned(),
        ))
        .await
        .map_err(internal_error)?;

    Ok(Json(HealthResponse { status: "ok" }))
}

async fn create_device(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>), (StatusCode, String)> {
    let txn = state.db.begin().await.map_err(internal_error)?;

    let device = entities::device::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(internal_error)?;

    txn.commit().await.map_err(internal_error)?;

    Ok((
        StatusCode::CREATED,
        Json(DeviceResponse {
            id: device.id,
            name: device.name,
        }),
    ))
}

async fn connect_database() -> DatabaseConnection {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set before starting the server");

    Database::connect(&database_url)
        .await
        .unwrap_or_else(|error| panic!("failed to connect to database: {error}"))
}

fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
