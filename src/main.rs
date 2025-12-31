use crate::{
    command::{help::CommandHelp, version::CommandVersion,}, 
    globals::CMD_LINE_ARGS
};

mod arg;
mod log;
mod command;
mod globals;

fn main() {

    debug!("Program started");

    if CMD_LINE_ARGS.version {
        CommandVersion::create().run();
        std::process::exit(0);
    }

    cmd_print_header();

    if CMD_LINE_ARGS.help {
        CommandHelp::create().run();
        std::process::exit(0);
    }

    if CMD_LINE_ARGS.input.is_none() {
        trace!("The input from command-line was not specified.");
        println!("No input file detected: type 'rustincase -h' for help");
        std::process::exit(1);
    }
}

fn cmd_print_header()
{
    println!(
        r#"
    ░█▀▄░█░█░█▀▀░▀█▀░░░░░▀█▀░█▀█░░░░░█▀▀░█▀█░█▀▀░█▀▀
    ░█▀▄░█░█░▀▀█░░█░░▄▄▄░░█░░█░█░▄▄▄░█░░░█▀█░▀▀█░█▀▀
    ░▀░▀░▀▀▀░▀▀▀░░▀░░░░░░▀▀▀░▀░▀░░░░░▀▀▀░▀░▀░▀▀▀░▀▀▀
    
    Author(s): {}
    Version: {}
"#,env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_VERSION"));
}