mod router;

use std::pin::Pin;
use http_body_util::combinators::BoxBody;
use http_body_util::Full;
use hyper::body::{Bytes};
use hyper::{Request, Response};
use hyper::service::Service;
use sqlx::PgPool;
use crate::app::router::router;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

// main app structure
// sets up the hyper service
#[derive(Clone)]
pub struct App {
    pg_pool: PgPool,
}

impl App {
    pub(crate) fn new(pg_pool: PgPool) -> Self {
        App { pg_pool }
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
