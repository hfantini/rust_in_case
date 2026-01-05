pub trait Loggable {
    fn log(&self, label: &'static str);
}