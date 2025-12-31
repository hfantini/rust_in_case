use crate::{command::command_switch::CommandSwitch, trace};

pub struct CommandVersion {
}

impl CommandVersion {
    pub fn create() -> CommandSwitch {
        CommandSwitch {
            name: "Version",
            description: "Displays the current version.",
            short: Some("-v"),
            long: Some("--version"),
            param: None,
            func: |_value: &CommandSwitch| {
                trace!("Version command triggered");
                println!(env!("CARGO_PKG_VERSION"));
            }
        }
    }
}