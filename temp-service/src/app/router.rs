use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty};
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;

pub async fn router(req: Request<hyper::body::Incoming>)
                    -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
{
    match (req.method(), req.uri().path()) {
        _ => {
            let mut response = Response::new(
                Empty::new()
                    .map_err(|never| match never {})
                    .boxed()
            );
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}
