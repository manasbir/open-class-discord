use anyhow::{anyhow, Result};
use discord::{make_res, parse_event};
use reqwest::StatusCode;
use serde_json::{json, Value};
use worker::*;


#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();
    console_log!("scheduled event");
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    
    let mut req = req.clone()?;

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




