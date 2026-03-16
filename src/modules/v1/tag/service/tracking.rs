use axum::{body::Bytes, http::StatusCode};

pub async fn push(body: Bytes) -> StatusCode {
    let body = String::from_utf8_lossy(&body);
    println!("tracking push body: {body}");

    StatusCode::OK
}
