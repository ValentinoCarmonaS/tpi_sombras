use crate::flatlanders::Flatlanders;

#[derive(Debug)]
pub struct Data {
    pub degrees: i32,
    pub flatlanders: Vec<Flatlanders>,
}