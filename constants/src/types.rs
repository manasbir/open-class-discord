use std::str;

use serde::Deserialize;

use crate::commands::CommandNames;

#[derive(Deserialize)]
 pub struct Interaction {
    // TODO make types enum
    pub r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub id: String,
    pub name: CommandNames,
    pub options: Vec<Options>,
    // TODO make types enum
    pub r#type: i32,
}

#[derive(Deserialize, Debug)]
pub struct Options {
    // pub focused: bool,
    pub name: String,
    // TODO make types enum
    pub r#type: i32,
    pub value: String,
}