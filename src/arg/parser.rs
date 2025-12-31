use std::process::Command;

use crate::{debug, trace, error, critical};

#[derive(Debug)]
pub struct CmdLineArgs {
    pub command: Option<String>,
    pub args: Option<Vec<String>>
}

impl Default for CmdLineArgs {

    fn default() -> Self {
        CmdLineArgs { command: None, args: None }
    }
}

impl CmdLineArgs {

    pub fn parse() -> Self {
        debug!("CmdLineArgs::parse() called");
        
        let mut ret = Self::default();
        let mut iter = std::env::args().skip(1);
        
        debug!("std::env::args() taken with skip(1)");
        trace!("Parsing command-line arguments");

        while let Some(arg) = iter.next() {
            if Self::is_command(&arg) {
                trace!("Found command: {}", &arg);
                ret.command = Some(arg);
            }
        }

        debug!("CmdLineArgs::parse() finished");
        ret

    }

    fn is_command(value: &str) -> bool {
        !value.starts_with('-')
    }
}