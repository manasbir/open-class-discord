mod helpers;
pub mod find_class;
use core::time;
use std::{ops::Deref, str::FromStr, u64};
use axum::{body::Body, extract::State, http::{HeaderMap, HeaderValue, StatusCode}, response::Response, routing::{get,post}, Router};
use anyhow::{Result, anyhow};
use constants::{commands::CommandNames, types::{Data, Interaction}};
use helpers::make_res;
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

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();
    console_log!("scheduled event");
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let db= _env.d1("DB")?;
    console_log!("{:?}", db.dump().await);

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
            if verify_sig(state.public_key, &headers, bytes).is_err() {
                return make_res(StatusCode::BAD_REQUEST, json!({ "type": 4, "data": {"content": "bad signature"}}));
            }

            let timestamp: u64 = headers.get("X-Signature-Timestamp").unwrap().to_str().unwrap().parse().unwrap();
    
            // todo, generalize error handling
            // probably with a macro
            match res(state, interaction, timestamp).await {
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

pub async fn res(state: Vars, interaction: Interaction, timestamp: u64) -> Result<Response> {
    match interaction.r#type {
         // PING
        1 => Ok(make_res(StatusCode::OK, json!({ "type": 1 }))),
        // slash command
        2 => parse_commands(state, interaction.data, timestamp).await,
        _ => Ok(make_res(StatusCode::OK, json!({ "type": 4, "data": {"content": "unknown interaction type"}})))
    }

}

async fn parse_commands(state: Vars, data: Option<Data>, timestamp: u64) -> Result<Response> {
    match data {
        Some(data) => {
            match data.name {
                CommandNames::FindClass => {
                    console_log!("{data:?}");
                    return Ok(make_res(StatusCode::OK, json!({ "type": 4, "data": {"content": format!("{data:?}, {timestamp:?}")}})));
                },
            }
        },
        None => Ok(make_res(StatusCode::OK, json!({ "type": 4, "data": {"content": "unknown interaction type"}})))
    }
}

fn verify_sig(public_key: VerifyingKey, headers: &HeaderMap, bytes: axum::body::Bytes) -> Result<()> {
    let timestamp = headers.get("X-Signature-Timestamp").ok_or_else(|| anyhow!("could not get signature"))?.to_str()?;
    let signature_bytes = hex::decode(headers.get("X-Signature-Ed25519").ok_or_else(|| anyhow!("could not get signature"))?)?;

    let mut verify_data = timestamp.as_bytes().to_vec();
    verify_data.extend(&bytes);

    let signature = ed25519_dalek::Signature::from_str(&hex::encode(&signature_bytes))?;
    Ok(public_key.verify(&verify_data, &signature)?)
}
