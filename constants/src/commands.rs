use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Command {
    pub name: CommandNames,
    pub description: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<&'static [CommandOptions]>,
}

#[derive(Debug, Serialize)]
pub struct CommandOptions {
    pub r#type: u8,
    name: &'static str,
    pub autocomplete: bool,
    description: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum CommandNames {
    FindClass,
}

pub const COMMANDS: [Command; 1] = [
    Command {
        name: CommandNames::FindClass,
        description: "Find a class",
        options: Some(&[
            CommandOptions {
                r#type: 3,
                name: "building",
                autocomplete: true,
                description: "building (concat) + optional floor nums",
            },
            CommandOptions {
                r#type: 3,
                name: "time",
                autocomplete: false,
                description: "The class to find + optional floor nums",
            },
        ]),
    },
];
