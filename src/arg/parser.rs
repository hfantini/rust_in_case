use std::{collections::HashMap};

use crate::{debug, trace, error};

#[derive(Debug)]
pub struct CmdLineArgs {
    pub command: Option<String>,
    pub args: Option<HashMap<String, Vec<String>>>
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

        let mut cmd: Option<String> = None;
        let mut flag: Option<String> = None;
        let mut param: Vec<String> = Vec::new();

        let mut params: HashMap<String, Vec<String>> = HashMap::new();

        // PARSING CMD-LINE VALUES

        while let Some(arg) = iter.next() {
            if(!Self::is_switch(&arg)) {
                if cmd.is_none() {
                    cmd = Some(arg);
                } else {

                    if flag.is_none() {
                        error!("Parameter {} found before a command or switch. 
                            \n\n type 'rustincase help' for more info.", &arg);
                        std::process::exit(2);
                    }
                    
                    param.push(arg);
                }
            } else {

                if cmd.is_none() {
                    error!("Switch {} found before a command. 
                            \n\n type 'rustincase help' for more info.", &arg);
                    std::process::exit(2);
                }

                // PUSHING COMMAND BEFORE THE NEXT ONE

                if flag.is_some() {
                    trace!("Pushing command '{}' to the structure", flag.as_ref().unwrap());
                    params.insert(flag.unwrap().clone(), param.clone());

                    flag = None;
                    param.clear();
                }

                flag = Some(arg);
            }
        }

        // PROCESSING THE LAST RESULT
        
        if flag.is_some() {
            trace!("Pushing final command '{}' to the structure", flag.as_ref().unwrap());
            params.insert(flag.unwrap().clone(), param.clone());
        }

        // PROCESSING PARSED RESULTS

        ret.command = cmd;
        if(params.len() > 0) {
            ret.args = Some(params.clone());
        }

        debug!("CmdLineArgs::parse() finished");
        ret

    }

    fn is_switch(value: &str) -> bool {
        value.starts_with('-')
    }

}