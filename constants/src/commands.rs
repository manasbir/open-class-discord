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
    Init
}

pub const COMMANDS: [Command; 2] = [
    Command {
        name: CommandNames::FindClass,
        description: "Find a class",
        options: Some(&[
            CommandOptions {
                r#type: 3,
                name: "building",
                autocomplete: true,
                description: "building (concat)",
            },
            CommandOptions {
                r#type: 3,
                name: "floor number",
                autocomplete: false,
                description: "floor num",
            },
            CommandOptions {
                r#type: 3,
                name: "room number",
                autocomplete: false,
                description: "room num",
            },
            CommandOptions {
                r#type: 3,
                name: "time",
                autocomplete: false,
                description: "time",
            },
            
        ]),
    },
    Command {
        name: CommandNames::Init,
        description: "Initialize the bot",
        options: None,
    },
];
