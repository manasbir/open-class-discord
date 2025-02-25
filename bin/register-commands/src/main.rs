use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use discord::commands::COMMANDS;



#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let client = Client::new();
    // TODO use wranger.toml to get token and kill .env
    let discord_token = std::env::var("DISCORD_TOKEN")?;
    let discord_application_id = std::env::var("DISCORD_APPLICATION_ID")?;
    let url = format!(
        "https://discord.com/api/v10/applications/{}/commands",
        discord_application_id
    );

    let mut commands = json!(COMMANDS);
    for command in commands.as_array_mut().unwrap() {
        command["integration_types"] = json!([0,1]);
        command["dm_permission"] = json!(true);
        command["contexts"] = json!([0,1,2]);
    }

    let res = client
        .put(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {}", discord_token))
        .body(json!(commands).to_string()).send().await?;

    match res.status().as_u16() {
        200..=299 => {
            println!("Commands registered successfully!");
            println!("registered commands:");
            for info in res.json::<Vec<Value>>().await? {
                println!("{}", info["name"]);
            }
        }
        _ => {
            println!("Failed to register commands: {:?}", res.text().await?);
        }
    }

    Ok(())
}