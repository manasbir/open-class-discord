use std::str::FromStr;

use crate::{
    embed::{
        builder::EmbedBuilder,
        types::{Embed, EmbedField, EmbedFooter},
    },
    interactions::Interaction,
    make_res,
};
use anyhow::Result;
use chrono::{Datelike, Local, NaiveTime, Timelike};
use d1::{get_open_classes, Params, SQLRes};
use reqwest::StatusCode;
use serde_json::json;
use worker::{D1Database, Response};

pub async fn find_class(db: D1Database, interaction: Interaction) -> Result<Response> {
    let options = interaction.data.unwrap().options;
    let mut msg: Option<&str> = None;

    let building = options
        .get("building")
        .map(|building| building.value.clone());
    let floor = options.get("floor").map(|floor| floor.value.clone());
    let room = options.get("room").map(|room| room.value.clone());
    let end_time = match options.get("end_time") {
        Some(end_time) => match NaiveTime::from_str(&end_time.value) {
            Ok(end_time) => {
                if end_time.minute() < 10 {
                    Some(format!(
                        "{}:0{}",
                        (end_time.hour() % 12) + 12,
                        end_time.minute()
                    ))
                } else {
                    Some(format!(
                        "{}:{}",
                        (end_time.hour() % 12) + 12,
                        end_time.minute()
                    ))
                }
            }
            Err(_) => {
                msg = Some("End time did not work");
                None
            }
        },
        None => None,
    };
    let start_time = match options.get("start_time") {
        Some(time) => {
            let time = NaiveTime::from_str(&time.value)?;
            if time.minute() < 10 {
                format!("{}:0{}", (time.hour() % 12) + 12, time.minute())
            } else {
                format!("{}:{}", (time.hour() % 12) + 12, time.minute())
            }
        }
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

    make_res(
        StatusCode::OK,
        json!({ "type": 4, "data": { "message": msg, "embeds": [build_embed(res)]}}),
    )
}

fn build_embed(res: Vec<&SQLRes>) -> Embed {
    let mut embed = EmbedBuilder::new()
        .title(format!("Open Classes for {}", res[0].building_code))
        .color(0x150578)
        .footer(EmbedFooter {
            text: "manas manas manas".to_string(),
            icon_url: Some(
                "https://pbs.twimg.com/profile_images/1467714157680070663/HYty_41-_400x400.jpg"
                    .to_string(),
            ),
            proxy_icon_url: None,
        });

    for class in res {
        let start_time = class.start_time.split(":").collect::<Vec<_>>();
        let start_time = NaiveTime::from_hms_opt(
            start_time[0].parse().unwrap(),
            start_time[1].parse().unwrap(),
            0,
        )
        .unwrap();
        let end_time = class.end_time.split(":").collect::<Vec<_>>();
        let end_time = NaiveTime::from_hms_opt(
            end_time[0].parse().unwrap(),
            end_time[1].parse().unwrap(),
            0,
        )
        .unwrap();
        embed = embed.field(EmbedField {
            name: format!("Room {}", class.room_number),
            value: format!(
                "{} - {}",
                start_time.format("%-I:%M %p"),
                end_time.format("%-I:%M %p")
            ),
            inline: false,
        });
    }

    embed.build()
}
