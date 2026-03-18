use axum::{routing::get, Router};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl axum::response::IntoResponse {
    axum::response::Json(json!({"status": "healthy", "version": "0.1.0"}))
}