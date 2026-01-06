use crate::{
    command::{backup::CmdBackup, command::Runnable, help::CmdHelp, version::CmdVersion},
    globals::CMD_LINE_ARGS,
    util::help::get_help_message,
};

mod arg;
mod command;
mod globals;
mod log;
mod util;

fn main() {
    debug!("Program started");

    if CMD_LINE_ARGS.command.is_some() {
        if CMD_LINE_ARGS.command.is_none() {
            error!("No sub-command found; Type 'rustincase help' for support");
            std::process::exit(1);
        }

        let command: &String = CMD_LINE_ARGS.command.as_ref().unwrap();

        print_header();

        let mut err: Option<Box<dyn std::error::Error>> = None;

        match command.as_str() {
            "version" => {
                CmdVersion::create().run().unwrap_or_else(|e| err = Some(e));
            }
            "help" => {
                CmdHelp::create().run().unwrap_or_else(|e| err = Some(e));
            }
            "backup" => {
                CmdBackup::create(
                    Some(CMD_LINE_ARGS.args.as_ref().unwrap().clone())
                )
                .run()
                .unwrap_or_else(|e| err = Some(e));;
            }
            _ => {
                error!(
                    "Unrecognized sub-command '{}'; {}",
                    command,
                    get_help_message()
                );
            }
        }

        if err.is_some() {
            error!("Process finished with error:\n {}", err.expect("Error message must be set"));
            std::process::exit(1);
        }

    } else {
        error!("Sub-command not found; Type 'rustincase help' for support");
        std::process::exit(1);
    }
}

fn print_header() {
    println!(
        r#"
    ░█▀▄░█░█░█▀▀░▀█▀░░░░░▀█▀░█▀█░░░░░█▀▀░█▀█░█▀▀░█▀▀
    ░█▀▄░█░█░▀▀█░░█░░▄▄▄░░█░░█░█░▄▄▄░█░░░█▀█░▀▀█░█▀▀
    ░▀░▀░▀▀▀░▀▀▀░░▀░░░░░░▀▀▀░▀░▀░░░░░▀▀▀░▀░▀░▀▀▀░▀▀▀
    
    Author(s): {}
    Version: {}
"#,
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_VERSION")
    );
}
