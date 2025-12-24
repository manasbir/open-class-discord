use std::str::FromStr;

use anyhow::{anyhow, Result};
use commands::{find_class::find_class, init::init_command, CommandNames};
use ed25519_dalek::{Verifier, VerifyingKey};
use interactions::Interaction;
use reqwest::StatusCode;
use serde_json::{json, Value};
use worker::{Env, Headers, Request, Response};

pub mod commands;
mod embed;
mod interactions;

pub fn make_res(code: StatusCode, body: Value) -> Result<Response> {
    Ok(Response::builder()
        .with_status(code.as_u16())
        .with_header("content-type", "application/json;charset=UTF-8")?
        .from_json(&body)?)
}

pub async fn parse_event(req: Request, env: Env) -> Result<Response> {
    let key = VerifyingKey::from_bytes(
        &hex::decode(env.secret("DISCORD_PUBLIC_KEY")?.to_string())
            .unwrap()
            .try_into()
            .unwrap(),
    )
    .unwrap();

    let mut req = req.clone()?;
    let bytes = req.bytes().await?;
    let headers = req.headers();

    verify_sig(key, headers, bytes)?;

    let timestamp = req
        .headers()
        .get("X-Signature-Timestamp")?
        .ok_or_else(|| anyhow!("no timestamp"))?;

    let mut interaction = req.json::<Interaction>().await?;
    interaction.timestamp = Some(timestamp);

    res(env, interaction).await
}

pub async fn res(env: Env, interaction: Interaction) -> Result<Response> {
    match interaction.r#type {
        // PING
        1 => Ok(make_res(StatusCode::OK, json!({ "type": 1 }))?),
        // slash command
        2 => parse_commands(env, interaction).await,
        // unknown
        _ => make_res(
            StatusCode::OK,
            json!({ "type": 4, "data": {"content": "unknown interaction type"}}),
        ),
    }
}

async fn parse_commands(env: Env, interaction: Interaction) -> Result<Response> {
    match &interaction.data {
        Some(data) => match data.name {
            CommandNames::Init => init_command(env, &interaction).await,
            CommandNames::FindClass => find_class(env.d1("DB")?, interaction).await,
        },
        None => make_res(
            StatusCode::OK,
            json!({ "type": 4, "data": {"content": "unknown interaction type"}}),
        ),
    }
}

fn verify_sig(public_key: VerifyingKey, headers: &Headers, bytes: Vec<u8>) -> Result<()> {
    let timestamp = headers
        .get("X-Signature-Timestamp")?
        .ok_or_else(|| anyhow!("could not get signature"))?
        .to_string();

    let signature_bytes = hex::decode(
        headers
            .get("X-Signature-Ed25519")?
            .ok_or_else(|| anyhow!("could not get signature"))?,
    )?;

    let mut verify_data = timestamp.as_bytes().to_vec();
    verify_data.extend(bytes);

    let signature = ed25519_dalek::Signature::from_str(&hex::encode(&signature_bytes))?;
    Ok(public_key.verify(&verify_data, &signature)?)
}
