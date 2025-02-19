pub mod find_class;
mod helpers;

use anyhow::{anyhow, Result};
use chrono::{Local, Timelike};
use constants::{
    commands::CommandNames,
    interaction::Interaction,
};
use ed25519_dalek::{Verifier, VerifyingKey};
use find_class::init_db;
use helpers::{build_query, make_res};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::{
    io::Read,
    str::FromStr,
    sync::Arc,
};
use worker::*;

#[derive(Clone)]
pub struct Vars {
    public_key: ed25519_dalek::VerifyingKey,
    db: Arc<D1Database>,
}

unsafe impl Send for Vars {}
unsafe impl Sync for Vars {}

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
                json!({ "type": 4, "data": {"content": format!("failed to respond to interaction: \n ```{e}```")}}),
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
            CommandNames::FindClass => find_class(env, interaction).await,
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
    if interaction.member.user.id != env.var("ADMIN_DISCORD_ID")?.to_string() {
        return make_res(
            StatusCode::OK,
            json!({ "type": 4, "data": {"content": "unauthorized"}}),
        );
    }

    init_db(&env.d1("DB")?).await?;

    make_res(
        StatusCode::OK,
        json!({ "type": 4, "data": {"content": "success"}}),
    )
}

async fn find_class(env: Env, interaction: Interaction) -> Result<Response> {
    let db = env.d1("DB")?;

    let options = interaction.data.unwrap().options;

    let building = options.get("building").map(|building| building.value.clone());
    let floor = options.get("floor").map(|floor| floor.value.clone());
    let room = options.get("room").map(|room| room.value.clone());
    let start_time = match options.get("start") {
        Some(time) => time.value.clone(),
        None => {
            let time = Local::now().time();
            if time.minute() < 10 {
                format!("{}:0{}", time.hour(), time.minute())
            } else {
                format!("{}:{}", time.hour(), time.minute())
            }
        }
    };

    let end_time = options.get("end").map(|end_time| end_time.value.clone());

    let (query, params) = build_query(building, floor, room, start_time, end_time);
    let mut stmt = db.prepare(&query);

    for param in params.iter() {
        console_log!("{:?}", param);
        stmt = stmt.bind(&[param.into()])?;
    }

    console_log!("{:?}", stmt.inner());

    // Execute query
    let results = stmt.all().await?;
    let res: Vec<Value> = results.results()?;

    make_res(
        StatusCode::OK,
        json!({ "type": 4, "data": {"content": format!("{:?}", res[0])}}),
    )
}
