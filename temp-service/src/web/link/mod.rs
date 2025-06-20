use crate::app::{App, BoxError};
use http_body_util::{BodyExt, Full};
use hyper::body::Buf;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};
use serde_json::json;

pub async fn create(
    req: Request<hyper::body::Incoming>
) -> Result<Response<Full<Bytes>>, BoxError> {
    let app = req
        .extensions()
        .get::<App>()
        .expect("extensions missing from request extensions")
        .clone();
    let body = req.collect().await?.aggregate();
    let link_fields = serde_json::from_reader(body.reader());
    Ok(match link_fields {
        Ok(link_fields) => {
            let id = app.link_service.create(link_fields).await;
            match id {
                Ok(id) => {
                    let mut response = Response::new(Full::new(Bytes::from(json!({
                        "status": "ok",
                        "message": "the link was created successfully",
                        "payload": {
                            "id": id
                        }
                    }).to_string())));
                    *response.status_mut() = StatusCode::CREATED;
                    response
                },
                Err(e) => {
                    println!("error creating link: {:?}", e);
                    let mut response = Response::new(Full::new(Bytes::from(json!({
                        "status": "error",
                        "message": "internal server error",
                    }).to_string())));
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    response
                }
            }
        },
        _ => {
            let mut response = Response::new(Full::new(Bytes::from(json!({
                "status": "error",
                "message": "invalid request body"
            }).to_string())));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response
        }
    })
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
