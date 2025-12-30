use crate::args::CmdLineArgs;

mod args;
mod log;

fn main() {

    log::log_trace("Rust_in_case @ 0.0.1");
    log::log_info("Rust_in_case @ 0.0.1");
    log::log_warn("Rust_in_case @ 0.0.1");
    log::log_error("Rust_in_case @ 0.0.1");
    log::log_critical("Rust_in_case @ 0.0.1");

    let value = CmdLineArgs::parse();
    println!("{:#?}", value);
}
