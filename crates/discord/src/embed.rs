use serde::Deserialize;
use serde_json::{json, Value};


#[derive(Debug, Deserialize)]
pub struct OpenBuildings {
    pub building_code: String,
    pub floors: Vec<OpenFloors>,
}

#[derive(Debug, Deserialize)]
pub struct OpenFloors {
    pub floor_number: u32,
    pub rooms: Vec<OpenRooms>,
}

#[derive(Debug, Deserialize)]
pub struct OpenRooms {
    pub room_number: u32,
    pub time_slots: Vec<OpenTimes>,
}

#[derive(Debug, Deserialize)]
pub struct OpenTimes {
    pub start_time: String,
    pub end_time: String,
}

pub fn make_embed(info: Vec<&OpenBuildings>) -> Value {
    let mut embeds: Vec<Value> = Vec::new();


    json!({
        "type": 2,
        "data": {
            "embeds": embeds,
        },
    })
}