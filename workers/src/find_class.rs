

use constants::find_class::{FindClassRes, FIND_CLASS_URL};
use worker::{console_log, D1Database, D1PreparedStatement};
use anyhow::Result;


pub async fn init_db(db: D1Database) -> Result<()> {
    let mut statements  = Vec::new();
    let features = reqwest::get(FIND_CLASS_URL).await?.json::<FindClassRes>().await?.data.features;

    let mut buildings = Vec::new();
    let mut floors = Vec::new();
    let mut rooms = Vec::new();
    let mut time_slots = Vec::new();
    features.iter().for_each( |feat| {
        let code = &feat.properties.building_code;
        let name = &feat.properties.building_name;
        buildings.push(format!("({},{})", code, name));
        let slots = match &feat.properties.open_classroom_slots {
            Some(class_info) => &class_info.data,
            None => return,
        };


        for slot in slots {
            let room_number = &slot.room_number;
            let schedule = &slot.schedule;
            let floor = match room_number.chars().next() {
                Some(c) => c.to_string(),
                None => "-1".to_string(),
            };

            let floor_id = format!("{}_{}", code, floor);
            let room_id = format!("{}_{}", code, room_number);

            floors.push(format!("({},{},{},{})", floor_id, code, floor, floor));
            rooms.push(format!("({},{},{},{})", room_id, code, floor_id, room_number));

            for sched in schedule{
                let day = &sched.weekday;
                let slots = &sched.slots;
                slots.iter().enumerate().for_each( |(idx, slot)| {
                    let slot_id = format!("{}_{}_{}", room_id, day, idx);
                    time_slots.push(format!("({},{},{},{},{})", slot_id, room_id, day, slot.start_time,  slot.end_time));
                });
            }
        }
    });

    insert_buildings(&db, &mut statements, buildings)?;
    insert_floors(&db, &mut statements, floors)?;
    insert_rooms(&db, &mut statements, rooms)?;
    insert_time_slots(&db, &mut statements, time_slots)?;

    let res = db.batch(statements).await?;

    res.iter().for_each( |res| {
        console_log!("{:?}", res.results::<String>());
    });

    Ok(())
}

pub fn insert_buildings(db: &D1Database, statements: &mut Vec<D1PreparedStatement>, buildings: Vec<String>) -> Result<()> {
    let buildings = buildings.join("\n");

    console_log!("{:?}", buildings);
    
    let insert_buildings = db.prepare(format!(r#"
        INSERT INTO buildings (building_code, primary_name) 
        VALUES 
        {}
        ON CONFLICT (building_code) DO UPDATE 
        SET primary_name = EXCLUDED.primary_name;
    "#, buildings));

    statements.push(insert_buildings);

    Ok(())
}

pub fn insert_floors(db: &D1Database, statements: &mut Vec<D1PreparedStatement>, floors: Vec<String>) -> Result<()> {
    let floors = floors.join("\n");

    console_log!("{:?}", floors);
    
    let insert_floors = db.prepare(format!(r#"
        INSERT INTO floors (floor_id, building_code, floor_number, floor_name) 
        VALUES 
        {}
        ON CONFLICT (floor_id) DO UPDATE 
        SET floor_name = EXCLUDED.floor_name;
    "#, floors));

    statements.push(insert_floors);

    Ok(())
}

pub fn insert_rooms(db: &D1Database, statements: &mut Vec<D1PreparedStatement>, rooms: Vec<String>) -> Result<()> {
    let rooms = rooms.join("\n");

    console_log!("{:?}", rooms);
    
    let insert_rooms = db.prepare(format!(r#"
        INSERT INTO rooms (room_id, building_code, floor_id, room_number) 
        VALUES 
        {}
        ON CONFLICT (room_id) DO UPDATE 
        SET room_number = EXCLUDED.room_number;
    "#, rooms));

    statements.push(insert_rooms);

    Ok(())
}

pub fn insert_time_slots(db: &D1Database, statements: &mut Vec<D1PreparedStatement>, time_slots:Vec<String>) -> Result<()> {
    let time_slots = time_slots.join("\n");
    console_log!("{:?}", time_slots);
    
    let insert_time_slots = db.prepare(format!(r#"
        INSERT INTO time_slots (time_slot_id, room_id, day, start_time, end_time) 
        VALUES 
        {}
        ON CONFLICT (time_slot_id) DO UPDATE;
    "#, time_slots));

    statements.push(insert_time_slots);

    Ok(())
}