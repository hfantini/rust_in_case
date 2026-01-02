
use colored::*;

use crate::util::datetime::now_formatted;

#[derive(PartialEq, Eq)]
pub enum Level {
    DEBUG,
    TRACE,
    INFO,
    WARN,
    ERROR,
    CRITICAL
}

pub struct Log {
    level: Level,
    message: String
}

pub fn log(value: Log) {

    // DATE & TIME

    let formatted_datetime_now = now_formatted();

    // LOG MESSAGE TO STDOUT

    let message = format!(
        "{} {}: {}",
        translate_level_prefix(&value.level),
        formatted_datetime_now,
        value.message
    );

    println!("{}", colorize_log(&message, &value.level))
}

pub fn log_trace(message: String) {
    if !cfg!(debug_assertions) {
        return;
    }

    log(Log {level: Level::TRACE, message: message});
}

pub fn log_debug(message: String) {
    if !cfg!(debug_assertions) {
        return;
    }

    log(Log {level: Level::DEBUG, message: message});
}


pub fn log_info(message: String) {
    log(Log {level: Level::INFO, message: message});
}

pub fn log_warn(message: String) {
    log(Log {level: Level::WARN, message: message});
}

pub fn log_error(message: String) {
    log(Log {level: Level::ERROR, message: message});
}

pub fn log_critical(message: String) {
    log(Log {level: Level::CRITICAL, message: message});
}

fn translate_level_prefix(level: &Level) -> &'static str {
    match level {
        Level::TRACE    => "[ TRCE ]",
        Level::DEBUG    => "[ DEBG ]",
        Level::INFO     => "[ INFO ]",
        Level::WARN     => "[ WARN ]",
        Level::ERROR    => "[ ERRO ]",
        Level::CRITICAL => "[ CRIT ]"
    }
}

fn colorize_log(msg: &String, level: &Level) -> String {
    match level {
        Level::TRACE => msg.white().to_string(),
        Level::DEBUG => msg.bright_magenta().to_string(),
        Level::WARN => msg.bright_yellow().to_string(),
        Level::ERROR => msg.bright_red().to_string(),
        Level::CRITICAL => msg.red().to_string(),
        _ => msg.normal().to_string()
    }
}