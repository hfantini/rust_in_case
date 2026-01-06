use std::collections::HashMap;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub trigger: &'static str,
    pub args: Option<HashMap<String, Vec<String>>>
}

pub trait Runnable {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
