use std::{collections::HashMap, str};
use serde::Deserialize;
use super::commands::CommandNames;


#[derive(Deserialize)]
 pub struct Interaction {
    // TODO make `type`` enum
    pub r#type: i32,
    pub data: Option<Data>,
    pub timestamp: Option<String>,
    pub user: Option<User>,
}


#[derive(Deserialize, Debug)]
pub struct User{
    pub id: String,
}

#[derive(Debug)]
pub struct Data {
    pub id: String,
    pub name: CommandNames,
    pub options: HashMap<String, Options>,
    // TODO make `type` enum
    pub r#type: i32,
}

impl <'de> serde::Deserialize<'de> for Data {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // First deserialize into a temporary struct to handle the nested JSON string
        #[derive(Deserialize)]
        struct Outer {
            id: String,
            name: CommandNames,
            options: Option<Vec<Options>>,
            r#type: i32,
        }

        let outer = Outer::deserialize(deserializer)?;

        let mut options = HashMap::new();
        if let Some(options_vec) = outer.options {
            for option in options_vec {
                options.insert(option.name.clone(), option);
            }
        }

        Ok(Data { id: outer.id, name: outer.name, options: options, r#type: outer.r#type })
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Options {
    // pub focused: bool,
    pub name: String,
    // TODO make `type` enum
    pub r#type: i32,
    pub value: String,
}