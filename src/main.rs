use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

async fn health_check() -> impl IntoResponse {
    "Json(json!({"status": "healthy", "version": "0.1.0"}))
}