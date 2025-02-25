mod helpers;
use anyhow::{anyhow, Result};
use chrono::{Datelike, Local, Timelike};
use ::d1::init::init_db;
use ::d1::query::build_query;
use discord::commands::{find_class::find_class, CommandNames};
use discord::interactions::Interaction;
use discord::make_res;
use ed25519_dalek::{Verifier, VerifyingKey};
use ::d1::SQLRes;
use discord::embed::{make_embed, OpenBuildings, OpenFloors, OpenRooms, OpenTimes};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::{
    collections::HashSet, io::Read, mem, str::FromStr, sync::Arc
};
use worker::*;
use crate::wasm_bindgen::JsValue;


#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();
    console_log!("scheduled event");
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let mut req = req.clone()?;
    // console_log!("{}", req.text().await?);
    match parse_event(&mut req, env).await {
        Ok(res) => Ok(res),
        Err(e) => {
            console_log!("Failed to respond to interaction: {:?}", e);
            make_res(
                StatusCode::OK,
                json!({ "type": 4, "data": {"content": format!("failed to respond to interaction:```{e}```")}}),
            )
        }
    }
}

pub async fn parse_event(req: &mut Request, env: Env) -> Result<Response> {
    let key = ed25519_dalek::VerifyingKey::from_bytes(
        &hex::decode(env.var("DISCORD_PUBLIC_KEY")?.to_string())
            .unwrap()
            .try_into()
            .unwrap(),
    )
    .unwrap();

    let mut req2 = req.clone()?;
    let bytes = req2.bytes().await?;
    let headers = req.headers();

    verify_sig(key, headers, bytes)?;

    let timestamp = req
        .headers()
        .get("X-Signature-Timestamp")?
        .ok_or_else(|| anyhow!("no signature"))?;

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

async fn init_command(env: Env, interaction: &Interaction) -> Result<Response> {
    let member = interaction.user.as_ref().ok_or_else(|| anyhow!("No member???"))?;
    if member.id != env.var("ADMIN_DISCORD_ID")?.to_string() {
        return make_res(
            StatusCode::OK,
            json!({ "type": 4, "data": {"content": "unauthorized"}}),
        );
    }

    JsValue::null();

    init_db(&env.d1("DB")?).await?;

    make_res(
        StatusCode::OK,
        json!({ "type": 4, "data": {"content": "success"}}),
    )
}


