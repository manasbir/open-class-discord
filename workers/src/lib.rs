use std::str::FromStr;
use axum::{body::Body, http::{HeaderMap, StatusCode}, response::Response, routing::{get,post}, Router, extract::State};
use anyhow::{Result, anyhow};
use constants::types::Interaction;
use serde_json::{json, Value};
use tower_service::Service;
use worker::*;
use ed25519_dalek::{Verifier, VerifyingKey};

#[derive(Debug, Clone)]
pub struct Vars {
    public_key: ed25519_dalek::VerifyingKey,
}


fn router(state: Vars) -> Router {
    Router::new()
    .route("/", post(parse_event))
    .route("/", get(parse_event))
    .with_state(state)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let key: String = _env.var("DISCORD_PUBLIC_KEY").unwrap().to_string();
    let state = Vars {
        public_key: ed25519_dalek::VerifyingKey::from_bytes(
            &hex::decode(key).unwrap().try_into().unwrap(),
        ).unwrap(),
    };
    Ok(router(state).call(req).await?)
}


pub async fn parse_event(State(state): State<Vars>, headers: HeaderMap, bytes: axum::body::Bytes) -> Response {
    match serde_json::from_slice::<Interaction>(&bytes) {
        Ok(interaction) => {
            if verify_sig(state.public_key, headers, bytes).is_err() {
                return make_res(StatusCode::BAD_REQUEST, json!({ "type": 4, "data": {"content": "bad signature"}}));
            }
    
            match res(state, interaction).await {
                Ok(res) => res,
                Err(e) => {
                    console_log!("Failed to respond to interaction: {:?}", e);
                    make_res(StatusCode::INTERNAL_SERVER_ERROR, json!({ "type": 4, "data": {"content": format!("failed to respond to interaction: {e}")}}))
                }
            }
        },
        Err(e) => {
            console_log!("Failed to parse interaction: {:?}", e);
            return make_res(StatusCode::BAD_REQUEST, json!({ "type": 4, "data": {"content": "failed to parse interaction"}}))
        }
    }
}

pub async fn res(state: Vars, interaction: Interaction) -> Result<Response> {
    match interaction.r#type {
         // PING
        1 => Ok(make_res(StatusCode::OK, json!({ "type": 1 }))),
        2 => parse_commands(state, interaction.data).await,
        _ => Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json;charset=UTF-8")
        .body(Body::from(json!({ "type": 4, "data": {"content": "unknown interaction type"}}).to_string())).unwrap())
    }

}

async fn parse_commands(state: Vars, data: Option<Data>) -> Result<Response> {
    match data {
        Some(data) => {
            match data.r#type {
                1 => Ok(make_res(StatusCode::OK, json!({ "type": 1 }))),
                2 => Ok(make_res(StatusCode::OK, json!({ "type": 1 }))),
                3 => Ok(make_res(StatusCode::OK, json!({ "type": 1 }))),
                _ => Ok(make_res(StatusCode::OK, json!({ "type": 1 })))
            }
        },
        None => Ok(make_res(StatusCode::OK, json!({ "type": 1, "data": {"content": "unknown interaction type"}})))
    }
}

fn verify_sig(public_key: VerifyingKey, headers: HeaderMap, bytes: axum::body::Bytes) -> Result<()> {
    let timestamp = headers.get("x-signature-timestamp").ok_or_else(|| anyhow!("could not get signature"))?.to_str()?;
    let signature_bytes = hex::decode(headers.get("X-Signature-Ed25519").ok_or_else(|| anyhow!("could not get signature"))?)?;

    let mut verify_data = timestamp.as_bytes().to_vec();
    verify_data.extend(&bytes);

    let signature = ed25519_dalek::Signature::from_str(&hex::encode(&signature_bytes))?;
    Ok(public_key.verify(&verify_data, &signature)?)
}

fn make_res(code: StatusCode, body: Value) -> Response {
    Response::builder()
        .status(code)
        .header("content-type", "application/json;charset=UTF-8")
        .body(Body::from(body.to_string())).unwrap()
}