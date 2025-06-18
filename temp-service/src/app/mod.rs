use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes};
use hyper::{Request, Response};
use crate::app::router::router;

mod router;

// main service
// some sort of context provider
pub async fn service(req: Request<hyper::body::Incoming>)
    -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
{
    router(req).await
}