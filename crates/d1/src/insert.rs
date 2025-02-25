use std::{collections::HashSet, ops::Deref};

use crate::types::{BuildingInfo, RoomInfo, TimeSlots, ToID};
use anyhow::Result;
use worker::{console_log, D1Database, D1PreparedStatement};

pub(crate) fn insert_buildings(
    db: &D1Database,
    statements: &mut Vec<D1PreparedStatement>,
    buildings: &[BuildingInfo],
) -> Result<()> {
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

    console_log!("Inserted buildings");

    Ok(())
}

pub(crate) fn insert_rooms_and_floors(
    db: &D1Database,
    statements: &mut Vec<D1PreparedStatement>,
    rooms: &[RoomInfo],
) -> Result<()> {
    let mut floors = HashSet::new();
    for room in rooms {
        let floor_id = room.floor_id.deref();
        let room_number = room.room_number.as_str();
        let building_code = room.building_code.as_str();
        let floor_number = room
            .room_number
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap();

        if !floors.contains(floor_id) {
            floors.insert(floor_id);

            let stmt = db.prepare(
                "INSERT INTO floors (floor_id, building_code, floor_number)
                VALUES (?1, ?2, ?3)
                ON CONFLICT (building_code, floor_number) DO NOTHING",
            );

            let stmt = stmt.bind(&[floor_id.into(), building_code.into(), floor_number.into()])?;

            statements.push(stmt);
        }

        let stmt = db.prepare(
            "INSERT INTO rooms (room_id, building_code, floor_id, room_number)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT (building_code, room_number) DO NOTHING",
        );

        let stmt = stmt.bind(&[
            room.to_id().into(),
            building_code.into(),
            floor_id.into(),
            room_number.into(),
        ])?;

        statements.push(stmt);
    }

    Ok(())
}

pub(crate) fn insert_time_slots(
    db: &D1Database,
    statements: &mut Vec<D1PreparedStatement>,
    slots: &[TimeSlots],
) -> Result<()> {
    for slot in slots {
        let stmt = db.prepare(
            "INSERT INTO time_slots (slot_id, room_id, day, start_time, end_time)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT (slot_id) DO UPDATE 
            SET start_time = excluded.start_time, end_time = excluded.end_time, day = excluded.day",
        );

        let mut start_time = slot.start_time.clone();

        start_time.pop();
        start_time.pop();
        start_time.pop();

        let mut end_time = slot.end_time.clone();

        end_time.pop();
        end_time.pop();
        end_time.pop();

        let stmt = stmt.bind(&[
            slot.to_id().into(),
            slot.room_id.as_str().into(),
            slot.day.as_str().into(),
            start_time.as_str().into(),
            end_time.as_str().into(),
        ])?;

        statements.push(stmt);
    }

    console_log!("Inserted time slots");

    Ok(())
}
