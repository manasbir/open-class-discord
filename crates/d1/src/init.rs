use anyhow::{Context, Result};
use portal::get_classes;
use std::{collections::HashSet, ops::Deref};
use worker::{console_log, D1Database, D1PreparedStatement};

use crate::{insert::{insert_buildings, insert_rooms_and_floors, insert_time_slots}, types::{BuildingInfo, FloorInfo, RoomInfo, TimeSlots, ToID}};

pub async fn init_db(db: &D1Database) -> Result<()> {
    let features = get_classes().await?;

    let mut statements = Vec::new();
    let mut buildings = Vec::new();
    let mut rooms = Vec::new();
    let mut time_slots = Vec::new();

    for props in features {

        // Process building
        let building = BuildingInfo {
            building_code: props.building_code,
            primary_name: props.building_name,
        };
        let building_id = building.to_id();
        // Process rooms and floors
        if let Some(slots) = props.open_classroom_slots {
            for room_data in slots.data {
                let floor_number = room_data
                    .room_number
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();

                let floor_id = FloorInfo {
                    building_code: building_id.clone(),
                    floor_number,
                }
                .to_id();
                let room = RoomInfo {
                    building_code: building_id.clone(),
                    floor_id,
                    room_number: room_data.room_number,
                };
                let room_id = room.to_id();

                // Process time slots
                for (idx, schedule) in room_data.schedule.iter().enumerate() {
                    let day = schedule.weekday.clone()[..3].to_string();

                    for slot in schedule.slots.clone() {
                        let start_time = slot.start_time;
                        let end_time = slot.end_time;
                        let slot = TimeSlots {
                            idx: idx.try_into().unwrap(),
                            room_id: room_id.clone(),
                            day: day.clone(),
                            start_time,
                            end_time,
                        };
                        time_slots.push(slot);
                    }
                }
                rooms.push(room);
            }
        }
        buildings.push(building);
    }

    console_log!("Inserting data");

    // Insert data
    insert_buildings(db, &mut statements, &buildings)?;
    insert_rooms_and_floors(db, &mut statements, &rooms)?;
    insert_time_slots(db, &mut statements, &time_slots)?;

    db.batch(statements).await?;

    Ok(())
}

