use crate::{
    command::{command::Runnable, help::CmdHelp, version::CmdVersion}, globals::CMD_LINE_ARGS
};

mod arg;
mod log;
mod command;
mod globals;

fn main() {

    debug!("Program started");

    if CMD_LINE_ARGS.command.is_some() {

        let command = CMD_LINE_ARGS.command.as_ref().unwrap();

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