use std::{collections::HashMap, path::Path};

use crate::{
    command::{command::{Command, Runnable},
    validation::{Validatable, Validation}}, debug, info, log::loggable::Loggable, trace, util::help::get_help_message
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

        trace!("CmdBackup: command triggered");

        let validation = self.validate();
        validation.log("backup");

        if !validation.status {
            info!("{}", get_help_message());
            std::process::exit(4);
        }

        println!("Processing....");
    }
}

impl Validatable for CmdBackup {
    fn validate(&self) -> Validation {
        debug!("CmdBackup: validation triggered");
        let mut ret = Validation::default();

        if self.command.args.is_some() {

            let args = self.command.args.as_ref().unwrap();
            
            // PARAM: input (-i --input)

            let switches = vec!("-i", "--input");
            let mut options: Option<&Vec<String>> = None;

            for value in switches {
                if args.contains_key(value)  {
                    options = args.get(value);
                    break;
                }
            }

            match options {
                Some(options) => {
                    match options.get(0) {
                        Some(input) => {
                            let path = Path::new(input);

                            if !path.is_file() {
                                ret.errors.push("'input' argument is not a valid file.".to_string());
                            } else {
                                trace!("CmdBackup: Validation status was set to true");
                                ret.status = true;   
                            }
                        },
                        None => {
                            ret.errors.push("'input' argument with invalid number os parameters.".to_string());
                        }
                    }
                }
                None => {
                    ret.errors.push("No input arguments found for option 'backup'".to_string());
                }
            }

        } else {
            ret.errors.push("No arguments found for option 'backup'".to_string());
        }

        ret
    }
}