use http_body_util::{Full};
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use hyper::http::HeaderValue;
use serde_json::json;
use crate::app::BoxError;

pub async fn router(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError>
{
    match (req.method(), req.uri().path()) {
        _ => {
            let mut response = Response::new(Full::new(
                Bytes::from(json!({
                    "status": "error",
                    "message": "the given route does not exist"
                }).to_string())
            ));
            *response.status_mut() = StatusCode::NOT_FOUND;
            response.headers_mut().insert(
                "Content-Type",
                HeaderValue::from_static("application/json")
            );

            Ok(response)
        }
    }
}
