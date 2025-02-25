use chrono::{Datelike, Local, Timelike};
use d1::{get_open_classes, Params, SQLRes};
use reqwest::StatusCode;
use serde_json::{json, Value};
use worker::{console_log, D1Database, Response};
use anyhow::Result;
use crate::{embed::{builder::EmbedBuilder, types::{Embed, EmbedAuthor, EmbedField, EmbedFooter}}, interactions::Interaction, make_res};



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

    make_res(StatusCode::OK,json!({ "type": 4, "data": {"embeds": [build_embed(res)]}}))
}

fn build_embed(res: Vec<&SQLRes>) -> Embed {
    let mut embed = EmbedBuilder::new()
        .title("Open Classes")
        .color(0x150578)
        .footer(EmbedFooter {
            text: "manas manas masas".to_string(),
            icon_url: Some("https://pbs.twimg.com/profile_images/1467714157680070663/HYty_41-_400x400.jpg".to_string()),
            proxy_icon_url: None
        })
        .description("allegedly a description".to_string());

    for (i,class) in res.iter().enumerate() {
        embed = embed.field(EmbedField {
            name: format!("{}", class.room_number),
            value: format!("{} - {}", class.start_time, class.end_time),
            inline: (i % 2) == 1
        });
    }

    embed.build()
}  

