use crate::{interactions::Interaction, make_res};
use anyhow::{anyhow, Result};
use d1::init::init_db;
use reqwest::StatusCode;
use serde_json::json;
use worker::{Env, Response};

// TODO: kill init command for a more secure way to initialize the database

pub async fn init_command(env: Env, interaction: &Interaction) -> Result<Response> {
    let member = interaction
        .user
        .as_ref()
        .ok_or_else(|| anyhow!("No member???"))?;
    if member.id != env.secret("ADMIN_DISCORD_ID")?.to_string() {
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
