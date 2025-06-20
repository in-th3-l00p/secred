mod app;
mod data;
mod web;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    let app = app::App::new(data::pool::initialize_pool().await);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let app_clone = app.clone();
        tokio::task::spawn(async move {
            if let Err(err) =
                http1::Builder::new()
                    .serve_connection(io, app_clone)
                    .await
            {
                eprintln!("error serving: {:?}", err);
            }
        });
    }
}
