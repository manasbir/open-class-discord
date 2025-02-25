pub const FIND_CLASS_URL: &str = "https://portalapi2.uwaterloo.ca/v2/map/OpenClassrooms";

use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};


#[derive(Debug, Deserialize, Clone)]
pub struct SQLRes {
    pub building_code: String,
    pub floor_number: u32,
    pub room_number: u32,
    pub start_time: String,
    pub end_time: String,

}

#[derive(Debug, Deserialize)]
pub struct FindClassRes {
    pub data: FindClassResData,
}

// TODO custom parsing to avoid nesting structs
#[derive(Debug)]
pub struct FindClassResData {
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct Property {
    pub building_name: String,
    pub building_code: String,
    pub open_classroom_slots: Option<OpenClassroomSlots>,
}


impl<'de> serde::Deserialize<'de> for FindClassResData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // First deserialize into a temporary struct to handle the nested JSON string
        #[derive(Deserialize)]
        struct Outer {
            features: Vec<Feature>,
        } 

        #[derive(Deserialize)]
        struct Feature {
            properties: Properties,
        }


        #[derive(Deserialize)]
        struct Properties {
            building_name: String,
            building_code: String,
            #[serde(rename = "openClassroomSlots")]
            open_classroom_slots: Option<String>,  // The data field contains a JSON string
        }

        let outer = Outer::deserialize(deserializer)?.features;

        let mut vec = Vec::new();

        for feature in outer {
            vec.push( match feature.properties.open_classroom_slots {
                None => Property { building_name: feature.properties.building_name, building_code: feature.properties.building_code, open_classroom_slots: None },
                Some(open_classroom_slots) => {
                    // Parse the inner JSONstring
                    let inner_value: Value = serde_json::from_str(&open_classroom_slots)
                        .map_err(serde::de::Error::custom)?;

                    let data = Some(OpenClassroomSlots::deserialize(inner_value)
                        .map_err(serde::de::Error::custom)?);


                    Property { building_name: feature.properties.building_name, building_code: feature.properties.building_code, open_classroom_slots: data}
                }
            })
        }
        Ok(FindClassResData { properties: vec })
    }
}


#[derive(Debug, Deserialize)]
pub struct OpenClassroomSlots {
    pub data: Vec<OpenClassroomData>,
}



#[derive(Debug, Deserialize, Clone)]
pub struct OpenClassroomData {
    #[serde(rename = "roomNumber")]
    pub room_number: String,
    #[serde(rename = "Schedule")]
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Schedule {
    #[serde(rename = "Weekday")]
    pub weekday: String,
    #[serde(rename = "Slots")]
    pub slots: Vec<Slot>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Slot {
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
}