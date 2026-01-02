use std::{collections::HashMap};

use crate::{
    command::command::{Command, Runnable},
    trace
};

pub struct CmdBackup {
    pub command:Command
}

impl<> CmdBackup<> {
    pub fn create(args: Option<HashMap<String, Vec<String>>>) -> CmdBackup {
        CmdBackup {
            command: Command {
                name: "Backup",
                description: "Executes a compressed backup (.zip) 
                using an input file that lists all source file paths.",
                trigger: "help",
                args: args
            }
        }
    }
}

impl Runnable for CmdBackup {
    fn run(&self) {
        trace!("Backup command triggered");
        println!("Processing....");
    }
}