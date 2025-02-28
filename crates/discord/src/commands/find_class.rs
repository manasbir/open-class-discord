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
use chrono_tz::America::Toronto;
use d1::{get_open_classes, Params, SQLRes};
use reqwest::StatusCode;
use serde_json::json;
use std::{fmt::format, str::FromStr};
use worker::{D1Database, Response};

pub async fn find_class(db: D1Database, interaction: Interaction) -> Result<Response> {
    let options = interaction.data.unwrap().options;
    let mut msg: Option<&str> = None;

    let building = match options.get("building") {
        Some(building) => building.value.clone().to_ascii_uppercase(),
        None => "MC".to_string(),
    };
    let floor_number = options.get("floor").map(|floor| floor.value.clone());
    let room_number = options.get("room").map(|room| room.value.clone());
    let end_time = match options.get("end_time") {
        Some(end_time) => match NaiveTime::from_str(&end_time.value) {
            Ok(end_time) => {
                let time = end_time.with_hour(end_time.hour() % 12 + 12).unwrap();
                Some(time.format("%H:%M").to_string())
            },
            Err(_) => {
                msg = Some("End time did not work");
                None
            }
        },
        None => None,
    };
    let start_time = match options.get("start_time") {
        Some(time) => {
            // TODO make nicer
            let time = NaiveTime::from_str(&time.value)?;
            let time = time.with_hour(time.hour() % 12 + 12).unwrap();
            let time = round_up(time);
            time.format("%H:%M").to_string()
        }
        None => {
            let time = round_up(Local::now().with_timezone(&Toronto).time());
            time.format("%H:%M").to_string()
        }
    };
    let day = Local::now().with_timezone(&Toronto).weekday().to_string();

    let params = Params {
        start_time,
        day,
        building_code: Some(building),
        floor_number,
        room_number,
        end_time,
    };

    let res = get_open_classes(db, params).await?;

    make_res(
        StatusCode::OK,
        json!({ "type": 4, "data": { "message": msg, "embeds": [build_embed(res)]}}),
    )
}

fn build_embed(res: Vec<SQLRes>) -> Embed {
    if res.len() == 0 {
        let embed = EmbedBuilder::new()
            .title("No open classes :(((((")
            .color(0x150578)
            .footer(EmbedFooter {
                text: "manas manas manas".to_string(),
                icon_url: Some(
                    "https://pbs.twimg.com/profile_images/1467714157680070663/HYty_41-_400x400.jpg"
                        .to_string(),
                ),
                proxy_icon_url: None,
            });

        return embed.build();
    }

    let len = res.len();

    let classes = res.iter().take(6).collect::<Vec<_>>();

    let mut embed = EmbedBuilder::new()
        .title(format!("Open Classes for {}", res[0].building_code))
        .color(0x150578)
        .footer(EmbedFooter {
            text: format!("Showing {} of {} classes", classes.len(), len),
            icon_url: Some(
                "https://pbs.twimg.com/profile_images/1467714157680070663/HYty_41-_400x400.jpg"
                    .to_string(),
            ),
            proxy_icon_url: None,
        });

    for class in classes {
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
            inline: true,
        });
    }

    embed.build()
}


fn round_up(time: NaiveTime) -> NaiveTime {
    if time.minute() <= 20 {
        NaiveTime::from_hms_opt(time.hour(), 20, 0).unwrap()
    } else if  time.minute() <= 50 {
        NaiveTime::from_hms_opt(time.hour(), 50, 0).unwrap()
    } else {
        time
    }
}