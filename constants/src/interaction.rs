use std::str;
use serde::Deserialize;

use crate::commands::CommandNames;

#[derive(Deserialize)]
 pub struct Interaction {
    // TODO make `type`` enum
    pub r#type: i32,
    pub data: Option<Data>,
    pub timestamp: Option<String>,
    pub member: Member,
}


#[derive(Deserialize, Debug)]
pub struct Member{
    pub user: User,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub id: String,
    pub name: CommandNames,
    pub options: Option<Vec<Options>>,
    // TODO make `type` enum
    pub r#type: i32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Options {
    // pub focused: bool,
    pub name: String,
    // TODO make `type` enum
    pub r#type: i32,
    pub value: String,
}