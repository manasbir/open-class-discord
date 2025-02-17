use axum::{body::Body, http::StatusCode, response::Response};
use serde_json::Value;

pub(crate) fn make_res(code: StatusCode, body: Value) -> Response {
    Response::builder()
        .status(code)
        .header("content-type", "application/json;charset=UTF-8")
        .body(Body::from(body.to_string())).unwrap()
}