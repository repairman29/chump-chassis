use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(json!({"status": "healthy", "version": "0.1.0"}))
}