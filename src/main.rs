mod app;
mod middleware;
mod modules;

use app::{AppState, connect_database};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = connect_database().await;
    let app = modules::router().with_state(AppState { db });
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|error| panic!("failed to bind to {bind_addr}: {error}"));

    println!("listening on http://{bind_addr}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .expect("server error");
}
