pub mod init;
pub mod insert;
pub mod refresh;
pub mod types;
use anyhow::Result;
use serde::{Deserialize, Deserializer};
use worker::{console_log, D1Database};

#[derive(Debug, Deserialize, Clone)]
pub struct SQLRes {
    pub building_code: String,
    pub floor_number: u32,
    pub room_number: u32,
    pub start_time: String,
    pub end_time: String,
}

pub struct Params {
    pub start_time: String,
    pub day: String,
    pub building_code: Option<String>,
    pub floor_number: Option<String>,
    pub room_number: Option<String>,
    pub end_time: Option<String>,
}

pub async fn get_open_classes(db: D1Database, params: Params) -> Result<Vec<SQLRes>> {
    let mut query_string = Vec::new();
    let mut param_vec = Vec::new();

    add_param(
        &mut query_string,
        &mut param_vec,
        Some(params.day),
        "AND t.day = ?",
    );
    add_param(
        &mut query_string,
        &mut param_vec,
        Some(params.start_time),
        "AND t.start_time >= ?",
    );
    add_param(
        &mut query_string,
        &mut param_vec,
        params.building_code,
        "AND b.building_code = ?",
    );
    add_param(
        &mut query_string,
        &mut param_vec,
        params.floor_number,
        "AND f.floor_number = ?",
    );
    add_param(
        &mut query_string,
        &mut param_vec,
        params.room_number,
        "AND r.room_number = ?",
    );
    add_param(
        &mut query_string,
        &mut param_vec,
        params.end_time,
        "AND t.end_time >= ?",
    );

    let query = format!(
        "SELECT DISTINCT r.room_id, r.floor_id, f.floor_number, r.building_code, r.room_number, 
        t.start_time, t.end_time, t.day
         FROM rooms r
         JOIN time_slots t ON r.room_id = t.room_id
         JOIN floors f ON r.floor_id = f.floor_id
         JOIN buildings b ON b.building_code = r.building_code
         WHERE 1=1 {}
         ORDER BY t.start_time ASC, t.end_time DESC",
        query_string.join(" ")
    );

    let params = param_vec
        .into_iter()
        .map(|param| param.into())
        .collect::<Vec<_>>();

    Ok(db
        .prepare(query)
        .bind(&params)?
        .all()
        .await?
        .results::<SQLRes>()?)
}

fn add_param<'a, T: ToString>(
    query_string: &mut Vec<&'a str>,
    params: &mut Vec<String>,
    value: Option<T>,
    string: &'a str,
) {
    if let Some(value) = value {
        query_string.push(string);
        params.push(value.to_string());
    }
}
