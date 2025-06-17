mod service;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use service::main_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = 
                http1::Builder::new()
                    .serve_connection(io, service_fn(main_service))
                    .await
            {
                eprintln!("error serving: {:?}", err);
            }
        });
    }
}
