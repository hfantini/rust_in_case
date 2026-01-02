use crate::{
    command::{backup::CmdBackup, command::Runnable, help::CmdHelp, version::CmdVersion}, globals::CMD_LINE_ARGS
};

mod arg;
mod log;
mod command;
mod globals;
mod util;

fn main() {

    debug!("Program started");

    if CMD_LINE_ARGS.command.is_some() {

        if CMD_LINE_ARGS.command.is_none() {
            error!("No sub-command found; Type 'rustincase help' for support");
            std::process::exit(1);
        }

        let command: &String = CMD_LINE_ARGS.command.as_ref().unwrap();

        if command == "version" {
            CmdVersion::create().run();
            std::process::exit(0);
        }

        print_header();

        match command.as_str() {
            "help" => {
                CmdHelp::create().run();
                std::process::exit(0);
            },
            "backup" => {
                CmdBackup::create(
                    if CMD_LINE_ARGS.args.is_some() {
                        Some(CMD_LINE_ARGS.args.as_ref().unwrap().clone())
                    } else {
                        None
                    }
                ).run();
            }
            _ => {
                error!("Unrecognized sub-command '{}'; Type 'rustincase help' for support", command);
                std::process::exit(1);
            }
        }
        
    } else {
        error!("Sub-command not found; Type 'rustincase help' for support");
        std::process::exit(1);
    }
}

fn print_header()
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