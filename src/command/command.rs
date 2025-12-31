pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub trigger: &'static str
}

pub trait Runnable {
    fn run(&self);
}

pub trait Validatable {
    fn validate(&self);
}