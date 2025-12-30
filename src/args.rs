use crate::log::{log_critical, log_debug, log_trace, log_warn};

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
        log_debug("CmdLineArgs::parse() called".to_string());
        
        let mut ret = Self::default();
        let mut iter = std::env::args().skip(1);
        
        log_debug("std::env::args() taken with skip(1)".to_string());
        log_trace("Parsing command-line arguments".to_string());

        while let Some(arg) = iter.next() {
            if Self::is_switch(&arg) {
                log_trace(format!("Found switch: {}", &arg));
                Self::parse_switch(&arg, &mut ret, &mut iter);
            } else {
                if ret.input.is_some() {
                    log_critical("Duplicated input value".to_string());
                    std::process::exit(1);
                }

                log_trace(format!("Input found: {}", arg));
                ret.input = Some(arg);
            }
        }

        log_debug("CmdLineArgs::parse() finished".to_string());
        ret
    }

    fn parse_switch(
        switch: &str,
        args: &mut Self,
        iter: &mut impl Iterator<Item = String>) {

        log_debug("CmdLineArgs::parse_switch() called".to_string());

        match switch {
            "-h" | "--help" => {
                args.help = true;
                log_trace("'help' flag set to true".to_string());
            }
            "-v" | "--version" => {
                args.version = true;
                log_trace("'version' flag set to true".to_string());
            }
            "-o" | "--output" => {
                args.output = iter.next();
                log_trace(
                    format!(
                        "'output' option set to {}", 
                        args.output.as_deref().unwrap_or("?")
                    )
                );
            }
            _ => {
                log_warn(format!("Unrecognized switch: '{}'", switch));
            }
        }

        log_debug("CmdLineArgs::parse_switch() finished".to_string());
    }
}