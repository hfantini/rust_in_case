use crate::{command::command_switch::CommandSwitch, trace};

pub struct CommandHelp {
}

impl CommandHelp {
    pub fn create() -> CommandSwitch {
        CommandSwitch {
            name: "Help",
            description: "Displays a help with all possible commands for this program.",
            short: Some("-h"),
            long: Some("--help"),
            param: None,
            func: |_value: &CommandSwitch| {
                trace!("Help command triggered");
                println!("Printing help...")
            }
        }
    }
}