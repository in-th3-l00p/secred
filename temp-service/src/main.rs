mod app;
mod data;
mod web;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = app::App::new().await;
    app.serve(SocketAddr::from(([127, 0, 0, 1], 3000))).await
}
