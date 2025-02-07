use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use workers::constants::commands::COMMANDS;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let client = Client::new();
    let discord_token = std::env::var("DISCORD_TOKEN")?;
    let discord_application_id = std::env::var("DISCORD_APPLICATION_ID")?;
    let url = format!(
        "https://discord.com/api/v10/applications/{}/commands",
        discord_application_id
    );

    let res = client
        .put(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {}", discord_token))
        .body(json!(COMMANDS).to_string()).send().await?;

    match res.status().as_u16() {
        200..=299 => {
            println!("Commands registered successfully!");
            println!("registered commands:");
            for info in res.json::<Vec<serde_json::Value>>().await? {
                println!("{}", info["name"]);
            }
        }
        _ => {
            println!("Failed to register commands: {:?}", res.text().await?);
        }
    }
    Ok(())
}
