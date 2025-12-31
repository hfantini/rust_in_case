use crate::{command::command::{Command, Runnable}, trace};

pub struct CmdHelp {
    pub command:Command
}

impl CmdHelp {
    pub fn create() -> CmdHelp {
        CmdHelp {
            command: Command {
                name: "Help",
                description: "Displays a help with all possible commands for this program.",
                trigger: "help"
            }
        }
    }
}

impl Runnable for CmdHelp {
    fn run(&self) {
        trace!("Help command triggered");
        println!("Printing help...");
    }
}