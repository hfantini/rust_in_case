use core::fmt;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    command::{
        command::{Command, Runnable},
        validation::{Validatable, Validation},
    },
    debug, info,
    log::loggable::Loggable,
    trace,
};

pub struct CmdBackup {
    pub command: Command,
}

#[derive(Debug)]
pub enum CmdBackupErrors {
    NoArgumentsFound,
    ValidationFailed,
}

impl Display for CmdBackupErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = "CmdBackup:";
        match self {
            CmdBackupErrors::NoArgumentsFound => {
                write!(f, "{} No arguments found", prefix)
            }
            CmdBackupErrors::ValidationFailed => {
                write!(f, "{} Argument validation failed", prefix)
            }
        }
    }
}

impl std::error::Error for CmdBackupErrors {}

impl CmdBackup {
    pub fn create(args: Option<HashMap<String, Vec<String>>>) -> CmdBackup {
        CmdBackup {
            command: Command {
                name: "Backup",
                description: "Executes a compressed backup (.zip) 
                using an input file that lists all source file paths.",
                trigger: "help",
                args: args,
            },
        }
    }
}

impl Runnable for CmdBackup {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("CmdBackup: command triggered");

        // ARG PARSING

        if self.command.args.is_none() {
            return Err(Box::new(CmdBackupErrors::NoArgumentsFound));
        }

        let args = self
            .command
            .args
            .as_ref()
            .ok_or_else(|| Box::new(CmdBackupErrors::NoArgumentsFound))?;

        // VALIDATION

        let validation = self.validate(args);
        validation.log("backup");

        if !validation.status {
            return Err(Box::new(CmdBackupErrors::ValidationFailed));
        }

        let path: &String = ["-i", "--input"]
            .iter()
            .find_map(|k| args.get(*k))
            .expect("validated input argument must exist")
            .get(0)
            .expect("validated input [0] argument must exist");

        info!("Processing file: {}", path);

        let file = File::open(path)?;

        let lock = file.lock_shared()?;

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| format!("{}", e))?;
            println!("Linha: {}", line);
        }

        Ok(())
    }
}

impl Validatable for CmdBackup {
    fn validate(&self, args: &HashMap<String, Vec<String>>) -> Validation {
        debug!("CmdBackup: validation triggered");
        let mut ret = Validation::default();

        // PARAM: input (-i --input)

        let options = ["-i", "--input"].iter().find_map(|k| args.get(*k));

        match options {
            Some(options) => match options.get(0) {
                Some(input) => {
                    let path = Path::new(input);

                    if !path.is_file() {
                        ret.errors
                            .push("'input' argument is not a valid file.".to_string());
                    } else {
                        trace!("CmdBackup: Validation status was set to true");
                        ret.status = true;
                    }
                }
                None => {
                    ret.errors
                        .push("'input' argument with invalid number os parameters.".to_string());
                }
            },
            None => {
                ret.errors
                    .push("No input arguments found for option 'backup'".to_string());
            }
        }

        ret
    }
}
