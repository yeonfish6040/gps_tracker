use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, Set, TransactionTrait, prelude::Uuid};
use serde::{Deserialize, Serialize};

use crate::{
    app::{AppState, internal_error},
    modules::v1::tag::domain::entities,
};

#[derive(Deserialize)]
pub struct CreateDeviceRequest {
    name: String,
}

#[derive(Serialize)]
pub struct DeviceResponse {
    id: Uuid,
    name: String,
}

pub async fn register_tag(
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