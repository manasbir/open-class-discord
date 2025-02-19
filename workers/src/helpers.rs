use anyhow::Result;
use reqwest::StatusCode;
use serde_json::Value;
use worker::{Response, ResponseBody};

pub(crate) fn make_res(code: StatusCode, body: Value) -> Result<Response> {
    Ok(Response::builder()
        .with_status(code.as_u16())
        .with_header("content-type", "application/json;charset=UTF-8")?
        .from_json(&body)?)
}

pub(crate) fn ordinal(n: i32) -> String {
    let suffix = match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}

pub(crate) fn build_query(
    building: Option<String>,
    floor: Option<String>,
    room: Option<String>,
    start_time: String,
    end_time: Option<String>,
) -> (String, Vec<String>) {
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    if let Some(building) = building {
        conditions.push("AND b.building_code = ?");
        params.push(building);
    }

    if let Some(floor) = floor {
        conditions.push("AND f.floor_number = ?");
        params.push(floor);
    }

    if let Some(room) = room {
        conditions.push("AND r.room_number = ?");
        params.push(room);
    }

    conditions.push("AND t.start_time >= ? ");
    params.push(start_time);

    if let Some(end_time) = end_time {
        conditions.push("AND t.end_time >= ? ");
        params.push(end_time);
    }

    let query = format!(
        "SELECT DISTINCT r.room_number, t.start_time, t.end_time, t.day
         FROM rooms r
         JOIN time_slots t ON r.room_id = t.room_id
         JOIN floors f ON r.floor_id = f.floor_id
         JOIN buildings b ON b.building_code = r.building_code
         WHERE 1=1 {}
         ORDER BY t.end_time DESC, t.start_time DESC",
        conditions.join(" ")
    );

    (query, params)
}
