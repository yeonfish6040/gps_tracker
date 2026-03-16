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
    let status_color = status_color(status);
    let method_color = method_color(&method);

    println!(
        "{dim}[{timestamp}]{reset} {method_color}{method}{reset} {path} {dim}|{reset} {status_color}{status}{reset} {cyan}+{elapsed_ms}ms{reset} {dim}from{reset} {yellow}{forwarded_ip} > {socket_ip}{reset} {dim}(\"{user_agent}\"){reset}",
        dim = "\x1b[2m",
        reset = "\x1b[0m",
        cyan = "\x1b[36m",
        yellow = "\x1b[33m",
    );

    response
}

fn forwarded_ip(headers: &HeaderMap) -> String {
    let forwarded_for = header_chain(headers, "x-forwarded-for");
    if !forwarded_for.is_empty() {
        return forwarded_for.join(" > ");
    }

    let forwarded = forwarded_header_chain(headers);
    if !forwarded.is_empty() {
        return forwarded.join(" > ");
    }

    "-".to_string()
}

fn header_chain(headers: &HeaderMap, key: &str) -> Vec<String> {
    headers
        .get(key)
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(normalize_forwarded_node)
                .filter(|part| !part.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn forwarded_header_chain(headers: &HeaderMap) -> Vec<String> {
    headers
        .get("forwarded")
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            value
                .split(',')
                .flat_map(|entry| entry.split(';'))
                .filter_map(|part| {
                    let part = part.trim();
                    let (_, value) = part.split_once('=')?;
                    if !part[..part.find('=').unwrap_or(0)].trim().eq_ignore_ascii_case("for") {
                        return None;
                    }
                    let normalized = normalize_forwarded_node(value.trim());
                    if normalized.is_empty() {
                        None
                    } else {
                        Some(normalized)
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn normalize_forwarded_node(value: &str) -> String {
    let trimmed = value.trim().trim_matches('"');
    let without_brackets = trimmed
        .strip_prefix('[')
        .and_then(|inner| inner.strip_suffix(']'))
        .unwrap_or(trimmed);

    if let Some((host, port)) = without_brackets.rsplit_once(':') {
        if !host.contains(':') && port.chars().all(|ch| ch.is_ascii_digit()) {
            return host.to_string();
        }
    }

    without_brackets.to_string()
}

fn header_value(headers: &HeaderMap, key: &str) -> String {
    headers
        .get(key)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "-".to_string())
}

fn method_color(method: &str) -> &'static str {
    match method {
        "GET" => "\x1b[34m",
        "POST" => "\x1b[32m",
        "PUT" | "PATCH" => "\x1b[35m",
        "DELETE" => "\x1b[31m",
        _ => "\x1b[37m",
    }
}

fn status_color(status: u16) -> &'static str {
    match status {
        200..=299 => "\x1b[32m",
        300..=399 => "\x1b[36m",
        400..=499 => "\x1b[33m",
        _ => "\x1b[31m",
    }
}
