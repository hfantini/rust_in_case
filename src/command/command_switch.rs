pub struct CommandSwitch {
    pub name: &'static str,
    pub description: &'static str,
    pub short: Option<&'static str>,
    pub long: Option<&'static str>,
    pub param: Option<Vec<String>>,
    pub func: fn(&CommandSwitch)
}

impl CommandSwitch {
    pub fn run(&self) {
        (self.func)(&self);
    }
}