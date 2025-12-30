use crate::args::CmdLineArgs;

mod args;

fn main() {
    let value = CmdLineArgs::parse();
    println!("{:#?}", value);
}
