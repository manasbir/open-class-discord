use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    // #[serde(skip_serializing)]
}

pub const COMMANDS: [Command; 2] = [
    Command {
        name: "root",
        description: "Root command",
    },
    Command {
        name: "add",
        description: "Add command",
    },
];
