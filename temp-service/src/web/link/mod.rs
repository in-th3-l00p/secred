use hyper::body::Buf;
use http_body_util::{BodyExt, Full};
use hyper::{Request, Response, StatusCode};
use hyper::body::Bytes;
use crate::app::BoxError;

pub async fn create(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError> {
    let body = req.collect().await?.aggregate();
    let parsed: serde_json::Value = serde_json::from_reader(body.reader())?;

    println!("{:#?}", parsed);

    let mut response = Response::new(Full::new(Bytes::from("{}")));
    *response.status_mut() = StatusCode::CREATED;
    Ok(response)
}

pub async fn get(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError> {
    let response = Response::new(Full::new(Bytes::from("{}")));
    Ok(response)
}

pub async fn update(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError> {
    let body = req.collect().await?.aggregate();
    let parsed: serde_json::Value = serde_json::from_reader(body.reader())?;

    println!("{:#?}", parsed);

    let response = Response::new(Full::new(Bytes::from("{}")));
    Ok(response)
}

pub async fn delete(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError> {
    let response = Response::new(Full::new(Bytes::from("{}")));
    Ok(response)
}
