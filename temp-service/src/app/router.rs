use http_body_util::{Full};
use hyper::{Method, Request, Response, StatusCode};
use hyper::body::Bytes;
use hyper::http::HeaderValue;
use serde_json::json;
use crate::app::BoxError;

pub async fn router(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError>
{
    let mut response = match (req.method(), req.uri().path()) {
        (&Method::POST, "/api/v1/link") => crate::web::link::create(req).await?,
        (&Method::GET, "/api/v1/link") => crate::web::link::get(req).await?,
        (&Method::PUT, "/api/v1/link") => crate::web::link::update(req).await?,
        (&Method::DELETE, "/api/v1/link") => crate::web::link::delete(req).await?,
        _ => {
            let mut response = Response::new(Full::new(
                Bytes::from(json!({
                    "status": "error",
                    "message": "the given route does not exist"
                }).to_string())
            ));
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    };

    response.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_static("application/json")
    );
    
    Ok(response)
}
