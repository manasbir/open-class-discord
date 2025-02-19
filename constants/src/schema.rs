pub struct BuildingInfo {
    pub building_code: String,
    pub primary_name: String,
}

pub struct FloorInfo {
    pub building_code: String,
    pub floor_number: u32,
}

pub struct RoomInfo {
    pub building_code: String,
    pub floor_id: String,
    pub room_number: String,
}

pub struct TimeSlots {
    pub idx: i32,
    pub start_time: String,
    pub end_time: String,
    pub day: String,
    pub room_id: String,
}

pub trait ToID {
    fn to_id(&self) -> String;
}

impl ToID for BuildingInfo {
    fn to_id(&self) -> String {
        self.building_code.clone()
    }
}

impl ToID for FloorInfo {
    fn to_id(&self) -> String {
        format!("{}-{}", self.building_code, self.floor_number)
    }
}

impl ToID for RoomInfo {
    fn to_id(&self) -> String {
        format!("{}-{}", self.floor_id, self.room_number)
    }
}

impl ToID for TimeSlots {
    fn to_id(&self) -> String {
        format!("{}-{}-{}", self.room_id, self.day, self.idx)
    }
}