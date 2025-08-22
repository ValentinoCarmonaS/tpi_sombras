use crate::flatlander::Flatlander;

#[derive(Debug)]
pub struct Data {
    pub degrees: i32,
    pub flatlanders: Vec<Flatlander>,
}