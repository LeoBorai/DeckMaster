use std::str::FromStr;

use axum::{
    body::Body,
    http::{HeaderName, Method, Uri},
};
use futures_util::TryStreamExt;
use worker::{Headers, Request, Response};

pub async fn to_axum_request(
    mut worker_request: Request,
) -> anyhow::Result<axum::extract::Request<Body>> {
    let method = Method::from_bytes(worker_request.method().to_string().as_bytes())?;
    let uri = Uri::from_str(worker_request.url()?.to_string().as_str())?;
    let body = worker_request.bytes().await?;

    let mut http_request = axum::extract::Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::from(body))?;

    for (header_name, header_value) in worker_request.headers() {
        http_request.headers_mut().insert(
            HeaderName::from_str(header_name.as_str())?,
            header_value.parse()?,
        );
    }

    Ok(http_request)
}

pub async fn to_worker_response(
    response: axum::response::Response<Body>,
) -> anyhow::Result<Response> {
    let mut bytes: Vec<u8> = Vec::<u8>::new();

    let (parts, body) = response.into_parts();

    let mut stream = body.into_data_stream();
    while let Some(chunk) = stream.try_next().await? {
        bytes.extend_from_slice(&chunk);
    }

    let code = parts.status.as_u16();

    let mut worker_response = Response::from_bytes(bytes)?;
    worker_response = worker_response.with_status(code);

    let headers = Headers::new();

    for (key, value) in parts.headers.iter() {
        headers.set(key.as_str(), value.to_str()?).unwrap()
    }
    worker_response = worker_response.with_headers(headers);

    Ok(worker_response)
}
