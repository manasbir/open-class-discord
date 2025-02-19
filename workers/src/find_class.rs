use std::{collections::HashSet, ops::Deref};
use constants::{find_class::{FindClassRes, FIND_CLASS_URL}, schema::{BuildingInfo, FloorInfo, RoomInfo, TimeSlots, ToID}};
use worker::{console_log, D1Database, D1PreparedStatement};
use anyhow::{Context, Result};

pub async fn init_db(db: &D1Database) -> Result<()> {
    console_log!("Initializing database");
    let features = reqwest::get(FIND_CLASS_URL)
        .await
        .context("Failed to fetch data")?
        .json::<FindClassRes>()
        .await
        .context("Failed to parse JSON")?
        .data.features;


    let mut statements = Vec::new();
    let mut buildings = Vec::new();
    let mut rooms = Vec::new();
    let mut time_slots = Vec::new();

    for feature in features {
        let props = feature.properties;
        
        // Process building
        let building = BuildingInfo { building_code: props.building_code, primary_name: props.building_name };
        let building_id = building.to_id();
        // Process rooms and floors
        if let Some(slots) = props.open_classroom_slots {
            for room_data in slots.data {
                let floor_number = room_data.room_number.chars()
                    .next().unwrap().to_digit(10).unwrap();
                
                let floor_id = FloorInfo { building_code: building_id.clone(), floor_number }.to_id();
                let room = RoomInfo { building_code: building_id.clone(), floor_id, room_number: room_data.room_number };
                let room_id = room.to_id();

                // Process time slots
                for (idx, schedule) in room_data.schedule.iter().enumerate() {
                    let day = schedule.weekday.clone();
                    
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

    // Insert data
    insert_buildings(&db, &mut statements, &buildings).await?;
    insert_rooms_and_floors(&db, &mut statements, &rooms).await?;
    insert_time_slots(&db, &mut statements, &time_slots).await?;

    console_log!("Executing batch");
    for tx in db.batch(statements).await? {
        console_log!("Result: {:?}", tx.success());
    }

    Ok(())
}


async fn insert_buildings(db: &D1Database, statements: &mut Vec<D1PreparedStatement>, buildings: &[BuildingInfo]) -> Result<()> {
    for building in buildings {
        let stmt = db.prepare(
            "INSERT INTO buildings (building_code, primary_name)
            VALUES (?1, ?2)
            ON CONFLICT (building_code) DO UPDATE
            SET primary_name = excluded.primary_name",
        );

        let code = building.building_code.as_str();
        let name = building.primary_name.as_str();
        let stmt = stmt.bind(&[code.into(), name.into()])?;
        statements.push(stmt);
    }
    Ok(())
}

async fn insert_rooms_and_floors(db: &D1Database,statements: &mut Vec<D1PreparedStatement>, rooms: &[RoomInfo]) -> Result<()> {
    let mut floors = HashSet::new();
    for room in rooms {
        let stmt = db.prepare(
            "INSERT INTO rooms (room_id, building_code, floor_id, room_number)
            VALUES (?1, (SELECT id FROM floors WHERE building_code = ?1 AND floor_number = ?2), ?3)
            ON CONFLICT (building_code, room_number) DO UPDATE
            SET floor_id = excluded.floor_id",
        );

        let floor_id = room.floor_id.deref();
        let room_number = room.room_number.as_str();
        let building_code = room.building_code.as_str();
        
        let stmt = stmt.bind(&[
            room.to_id().into(),
            building_code.into(),
            floor_id.into(),
            room_number.into()
        ])?;

        if floors.contains(floor_id) {
            floors.insert(floor_id);
        } else {
            let stmt = db.prepare(
                "INSERT INTO floors (floor_id, building_code, floor_number)
                VALUES (?1, ?2, ?3)
                ON CONFLICT (building_code, floor_number) DO NOTHING",
            );
            let stmt = stmt.bind(&[
                floor_id.into(),
                building_code.into(),
                room.floor_id.chars().next().unwrap().to_digit(10).unwrap().into()
            ])?;
            statements.push(stmt);
        }

        statements.push(stmt);
    }
    Ok(())
}

async fn insert_time_slots(db: &D1Database, statements: &mut Vec<D1PreparedStatement>,slots: &[TimeSlots]) -> Result<()> {
    for slot in slots {

        let stmt = db.prepare(
            "INSERT INTO time_slots (slot_id, room_id, day, start_time, end_time)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT (room_id, day, start_time, end_time) DO UPDATE",
        );

        let stmt = stmt.bind(&[
            slot.room_id.as_str().into(),
            slot.day.as_str().into(),
            slot.start_time.as_str().into(),
            slot.end_time.as_str().into(),
        ])?;

        statements.push(stmt);
    }
    Ok(())
}

