use chrono::{Datelike, Local, Timelike};
use d1::{get_open_classes, query::build_query, Params, SQLRes};
use reqwest::StatusCode;
use serde_json::{json, Value};
use worker::{console_log, D1Database, Response};
use anyhow::Result;
use crate::{interactions::Interaction, make_res};



pub async fn find_class(db: D1Database, interaction: Interaction) -> Result<Response> {
    let options = interaction.data.unwrap().options;

    let building = options.get("building").map(|building| building.value.clone());
    let floor = options.get("floor").map(|floor| floor.value.clone());
    let room = options.get("room").map(|room| room.value.clone());
    let end_time = options.get("end_time").map(|end_time| end_time.value.clone());
    let start_time = match options.get("start_time") {
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
    let day = Local::now().weekday().to_string();

    let params = Params {
        start_time,
        day,
        building_code: building,
        floor_number: floor,
        room_number: room,
        end_time,
    };

    let res = get_open_classes(db, params).await?;

    let res = res.iter().take(5).collect::<Vec<_>>();

    make_res(StatusCode::OK,json!({ "type": 4, "data": {"content": format!("```{:?}```", res)}}))
}