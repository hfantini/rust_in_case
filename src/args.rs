#[derive(Debug)]
pub struct CmdLineArgs {
    pub input: Option<String>,
    pub output: Option<String>,
    pub version: bool,
    pub help: bool
}

impl Default for CmdLineArgs {

    fn default() -> Self {
        CmdLineArgs { input: None, output: None, version: false, help: false }
    }
}

impl CmdLineArgs {

    pub fn is_switch(value: &str) -> bool {
        value.starts_with('-')
    }

    pub fn parse() -> Self {
        let mut ret = Self::default();
        let mut iter = std::env::args().skip(1);

        while let Some(arg) = iter.next() {
            if Self::is_switch(&arg) {
                Self::parse_switch(&arg, &mut ret, &mut iter);
            } else {

                if ret.input.is_some() {
                    panic!("Duplicated input value");
                }

                ret.input = Some(arg);
            }
        }

        ret
    }

    fn parse_switch(
        switch: &str,
        args: &mut Self,
        iter: &mut impl Iterator<Item = String>) {

        match switch {
            "-h" | "--help" => {
                args.help = true;
            }
            "-v" | "--version" => {
                args.version = true;
            }
            "-o" | "--output" => {
                args.output = iter.next()
            }
            _ => {
                println!("Warning: Unrecognized switch '{}'", switch);
            }
        }
    }
}