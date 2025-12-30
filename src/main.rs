use crate::args::CmdLineArgs;

mod args;
mod log;

fn main() {
    let value = CmdLineArgs::parse();
    println!("{:#?}", value);
}
