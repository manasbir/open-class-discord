use reqwest::StatusCode;
use serde_json::Value;
use worker::Response;
use anyhow::Result;

pub mod commands;
pub mod types;
pub mod embed;
pub mod interactions;

pub fn make_res(code: StatusCode, body: Value) -> Result<Response> {
    Ok(Response::builder()
        .with_status(code.as_u16())
        .with_header("content-type", "application/json;charset=UTF-8")?
        .from_json(&body)?)
}