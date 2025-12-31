use crate::{debug, trace, warn, critical};

#[derive(Debug)]
pub struct CmdLineArgs {
    pub input: Option<String>,
    pub output: Option<String>,
    pub version: bool,
    pub help: bool
}

impl Default for CmdLineArgs {

    fn default() -> Self {
        CmdLineArgs { input: None, output: None, version: false, help: false }
    }
}

impl CmdLineArgs {

    pub fn is_switch(value: &str) -> bool {
        value.starts_with('-')
    }

    pub fn parse() -> Self {
        debug!("CmdLineArgs::parse() called");
        
        let mut ret = Self::default();
        let mut iter = std::env::args().skip(1);
        
        debug!("std::env::args() taken with skip(1)");
        trace!("Parsing command-line arguments");

        while let Some(arg) = iter.next() {
            if Self::is_switch(&arg) {
                trace!("Found switch: {}", &arg);
                Self::parse_switch(&arg, &mut ret, &mut iter);
            } else {
                if ret.input.is_some() {
                    critical!("Duplicated input value");
                    std::process::exit(1);
                }

                trace!("Input found: {}", arg);
                ret.input = Some(arg);
            }
        }

        debug!("CmdLineArgs::parse() finished");
        ret
    }

    fn parse_switch(
        switch: &str,
        args: &mut Self,
        iter: &mut impl Iterator<Item = String>) {

        debug!("CmdLineArgs::parse_switch() called");

        match switch {
            "-h" | "--help" => {
                args.help = true;
                trace!("'help' flag set to true");
            }
            "-v" | "--version" => {
                args.version = true;
                trace!("'version' flag set to true");
            }
            "-o" | "--output" => {
                args.output = iter.next();
                trace!(
                    "'output' option set to {}", 
                    args.output.as_deref().unwrap_or("?")
                );
            }
            _ => {
                warn!("Unrecognized switch: '{}'", switch);
            }
        }

        debug!("CmdLineArgs::parse_switch() finished");
    }
}