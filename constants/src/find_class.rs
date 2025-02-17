pub const FIND_CLASS_URL: &str = "https://portalapi2.uwaterloo.ca/v2/map/OpenClassrooms";

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FindClassRes {
    pub data: FindClassResData,
}

// TODO custom parsing to avoid nesting structs
#[derive(Debug, Deserialize)]
pub struct FindClassResData {
    pub features: Vec<Features>,
}

#[derive(Debug, Deserialize)]
pub struct Features {
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub building_name: String,
    pub building_code: String,
    pub open_classroom_slots: Option<OpenClassroomSlots>,
}

#[derive(Debug, Deserialize)]
pub struct OpenClassroomSlots {
    pub data: Vec<OpenClassroomData>,
}

#[derive(Debug, Deserialize)]
pub struct OpenClassroomData {
    pub room_number: String,
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub weekday: String,
    pub slots: Vec<Slot>,
}

#[derive(Debug, Deserialize)]
pub struct Slot {
    pub start_time: String,
    pub end_time: String,
}