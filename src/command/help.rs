use colored::*;
use textwrap::{fill, Options};

use crate::{command::command::{Command, Runnable}, trace};

pub struct CmdHelp {
    pub command:Command
}

impl CmdHelp {
    pub fn create() -> CmdHelp {
        CmdHelp {
            command: Command {
                name: "Help",
                args: None
            }
        }
    }
}

impl Runnable for CmdHelp {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Help command triggered");

        let width = 80;

        let default_indent: &str = "    ";
        let subsequent_indent = format!("{default_indent}   ");

        let options = Options::new(width)
            .initial_indent(&default_indent)
            .subsequent_indent(&subsequent_indent);

        let divisor = fill("------------------", &options);

        let title = fill(
            "rust_in_case - a simple backup and compression tool", &options
        );

        let description = fill(
      "rust_in_case is a tool that bundles and compresses multiple folders and \
            files from your filesystem quickly and easily. It was created both as a \
            Rust learning project and as a practical way to generate backups that can \
            be packaged and copied to other destinations.",
            &options,
        );

        let usage = fill(
            "Usage: rust_in_case <command> [options]",
            &options,
        );

        let commands = [
            ("help", "Displays the available commands."),
            ("version", "Shows the current application version."),
            ("backup", "Performs a backup operation using an input file."),
        ];

        let commands_title = fill("Available commands:", &options);

        let mut commands_text = String::new();
        for (cmd, desc) in commands {
            let wrapped = fill(desc, 
                Options::new(width).initial_indent("    ").subsequent_indent("        "));
            commands_text.push_str(&format!("{}{:12}{}\n", default_indent, cmd, wrapped));
        }

        let options_title = fill("Global options:", &options);

        let options_section = [
            ("-h, --help", "Shows the help for a specific command."),
        ];

        let mut options_text = String::new();
        for (opt, desc) in options_section {
            let wrapped = fill(desc, Options::new(width).initial_indent("    ").subsequent_indent("        "));
            options_text.push_str(&format!("{}{:12}{}\n", default_indent, opt, wrapped));
        }

        println!("{}\n", title.bright_yellow().to_string());
        println!("{description}\n");
        println!("{}\n", usage.bright_green().to_string());
        
        println!("{}", commands_title.bright_cyan().to_string());
        println!("{}\n", divisor.bright_cyan().to_string());
        print!("{commands_text}\n");

        println!("{}", options_title.bright_cyan().to_string());
        println!("{}\n", divisor.bright_cyan().to_string());
        print!("{options_text}\n");

        Ok(())
    }
}