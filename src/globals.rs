use std::sync::LazyLock;
use crate::arg::parser::CmdLineArgs;

pub static CMD_LINE_ARGS: LazyLock<CmdLineArgs> = LazyLock::new(|| {
    CmdLineArgs::parse()
});