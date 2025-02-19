use serde::Deserialize;
use serde_json::{json, Value};


#[derive(Debug, Deserialize)]
pub struct OpenBuildings {
    pub building_code: String,
    pub floors: Vec<OpenFloors>,
}

#[derive(Debug, Deserialize)]
pub struct OpenFloors {
    pub floor_number: i32,
    pub rooms: Vec<OpenRooms>,
}

#[derive(Debug, Deserialize)]
pub struct OpenRooms {
    pub room_number: String,
    pub time_slots: Vec<OpenTimes>,
}

#[derive(Debug, Deserialize)]
pub struct OpenTimes {
    pub start_time: String,
    pub end_time: String,
}

pub fn make_embed(info: Vec<OpenBuildings>) -> Value {
    let mut embeds = Vec::new();

    for building in info {
        let mut floors = Vec::new();
        for floor in &building.floors {
            let mut rooms = Vec::new();
            for room in &floor.rooms {
                let mut time_slots = Vec::new();
                for time_slot in &room.time_slots {
                    time_slots.push(json!({
                        "start_time": time_slot.start_time,
                        "end_time": time_slot.end_time,
                    }));
                }
                rooms.push(json!({
                    "room_number": room.room_number,
                    "time_slots": time_slots,
                }));
            }
            floors.push(json!({
                "rooms": rooms,
            }));
        }
        embeds.push(json!({
            "title": building.building_code,
            "floors": floors,
        }));
    }

    json!({
        "type": 2,
        "embeds": embeds,
    })
}