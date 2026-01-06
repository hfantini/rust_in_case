use crate::{command::{command::{Command, Runnable}, version}, trace};

pub struct CmdVersion {
    pub command:Command
}

impl CmdVersion {
    pub fn create() -> CmdVersion {
            CmdVersion {
            command: Command {
                name: "Version",
                description: "Displays the current version.",
                trigger: "version",
                args: None
            }
        }
    }
}

impl Runnable for CmdVersion {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Version command triggered");
        println!(env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}