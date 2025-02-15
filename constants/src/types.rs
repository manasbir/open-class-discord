use std::str;

use serde::Deserialize;

#[derive(Deserialize)]
 pub struct Interaction {
    // TODO make types enum
    pub r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub options: Vec<Options>,
    // TODO make types enum
    pub r#type: i32,
}

#[derive(Deserialize)]
pub struct Options {
    pub focused: bool,
    pub name: String,
    // TODO make types enum
    pub r#type: i32,
    pub value: String,
}