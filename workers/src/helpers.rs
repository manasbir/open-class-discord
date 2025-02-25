use anyhow::Result;
use reqwest::StatusCode;
use serde_json::Value;
use worker::Response;

use discord::embed::{OpenBuildings, OpenFloors, OpenRooms, OpenTimes};
use portal::types::SQLRes;

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


pub fn sql_res_to_open_buildings(res: Vec<SQLRes>) -> Vec<OpenBuildings> {
    let mut building_info: Vec<OpenBuildings> = Vec::new();
    for r in res {
        match building_info.iter().position(|x| x.building_code == r.building_code) {
            Some(i) => {
                match building_info[i].floors.iter().position(|x| x.floor_number == r.floor_number) {
                    Some(j) => {
                        match building_info[i].floors[j].rooms.iter().position(|x| x.room_number == r.room_number) {
                            Some(k) => {
                                building_info[i].floors[j].rooms[k].time_slots.push(OpenTimes {
                                    start_time: r.start_time,
                                    end_time: r.end_time,
                                });
                            }
                            None => {
                                building_info[i].floors[j].rooms.push(OpenRooms {
                                    room_number: r.room_number,
                                    time_slots: vec![OpenTimes {
                                        start_time: r.start_time,
                                        end_time: r.end_time,
                                    }],
                                });
                            }
                        }
                    },
                    None => {
                        building_info[i].floors.push(OpenFloors {
                            floor_number: r.floor_number,
                            rooms: vec![OpenRooms {
                                room_number: r.room_number,
                                time_slots: vec![OpenTimes {
                                    start_time: r.start_time,
                                    end_time: r.end_time,
                                }],
                            }],
                        });
                    }
                }
            }
            None => {
                building_info.push(OpenBuildings {
                    building_code: r.building_code,
                    floors: vec![OpenFloors {
                        floor_number: r.floor_number,
                        rooms: vec![OpenRooms {
                            room_number: r.room_number,
                            time_slots: vec![OpenTimes {
                                start_time: r.start_time,
                                end_time: r.end_time,
                            }],
                        }],
                    }],
                });
            }
        }
    }

    building_info
}