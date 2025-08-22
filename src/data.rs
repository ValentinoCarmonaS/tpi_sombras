use crate::{flatlander::Flatlander, shadow_error::ShadowError};

#[derive(Debug)]
pub struct Data {
    degrees: i32,
    flatlanders: Vec<Flatlander>,
}

impl Data {
    pub fn new(degrees: i32, flatlanders: Vec<Flatlander>) -> Self {
        !unimplemented!()
    }

    pub fn set_degrees(&mut self, degrees: i32) -> Result<(), ShadowError> {
        !unimplemented!()
    }

    pub fn set_flatlander(&mut self, x: i32, h: i32) -> Result<(), ShadowError> {
        !unimplemented!()
    }

    pub fn sort(&mut self) -> Result<(), ShadowError> {
        !unimplemented!()
    }

    pub fn calculate_total_shadow_length(&self) -> Result<f64, ShadowError> {
        !unimplemented!()
    }
}