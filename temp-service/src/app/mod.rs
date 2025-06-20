mod router;

use crate::app::router::router;
use crate::data::service::link_service::LinkService;
use crate::data;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::net::TcpListener;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

// main app structure
// sets up the hyper service
#[derive(Clone)]
pub struct App {
    pg_pool: PgPool,
    link_service: LinkService
}

impl App {
    pub async fn new() -> Self {
        let pg_pool = data::pool::initialize_pool().await;
        let link_service = LinkService::new(pg_pool.clone());
        Self { pg_pool, link_service }
    }

    pub async fn serve(
        &self, addr: SocketAddr
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            let app_clone = self.clone();
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
}

impl Service<Request<hyper::body::Incoming>> for App {
    type Response = Response<Full<Bytes>>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<hyper::body::Incoming>) -> Self::Future {
        Box::pin(router(req))
    }
}
