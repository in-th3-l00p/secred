use std::convert::Infallible;

use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Bytes;

pub async fn main_service(req: Request<hyper::body::Incoming>) 
    -> Result<Response<Full<Bytes>>, Infallible>
{
    Ok(Response::new(Full::new(Bytes::from("hello"))))
}
