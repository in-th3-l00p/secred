use std::net::SocketAddr;
use std::convert::Infallible;

use http_body_util::Full;
use tokio::net::TcpListener;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

async fn service(req: Request<hyper::body::Incoming>) ->
    Result<Response<Full<Bytes>>, Infallible>
{
    Ok(Response::new(Full::new(Bytes::from("hello"))))
}

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
                    .serve_connection(io, service_fn(service))
                    .await
            {
                eprintln!("error serving: {:?}", err);
            }
        });
    }
}
