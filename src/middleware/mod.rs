use std::{net::SocketAddr, time::Instant};

use axum::{
    body::Body,
    extract::connect_info::ConnectInfo,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use chrono::Utc;

pub async fn request_logger(request: Request<Body>, next: Next) -> Response {
    let started_at = Instant::now();
    let timestamp = Utc::now().to_rfc3339();
    let method = request.method().to_string();
    let path = request.uri().path().to_owned();
    let headers = request.headers();
    let socket_ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.0.ip().to_string())
        .unwrap_or_else(|| "-".to_string());
    let forwarded_ip = forwarded_ip(headers);
    let user_agent = header_value(headers, "user-agent");

    let response = next.run(request).await;
    let status = response.status().as_u16();
    let elapsed_ms = started_at.elapsed().as_millis();

    println!(
        "[request] ts={timestamp} method={method} path={path} status={status} socket_ip={socket_ip} forwarded_ip={forwarded_ip} user_agent=\"{user_agent}\" elapsed_ms={elapsed_ms}"
    );

    response
}

fn forwarded_ip(headers: &HeaderMap) -> String {
    let forwarded_for = header_value(headers, "x-forwarded-for");
    if forwarded_for != "-" {
        return forwarded_for;
    }

    let forwarded = header_value(headers, "forwarded");
    if forwarded != "-" {
        return forwarded;
    }

    "-".to_string()
}

fn header_value(headers: &HeaderMap, key: &str) -> String {
    headers
        .get(key)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "-".to_string())
}
