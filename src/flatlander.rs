use std::cmp::Ordering;
use crate::shadow_error::ShadowError;

#[derive(Debug)]
pub struct Flatlander {
    x: i32,
    h: i32,
}

impl Flatlander {
    pub fn new(x: i32, h: i32) -> Result<Self, ShadowError> {
        if x < 0 || x > 310000 {
            return Err(ShadowError::InvalidPositionOrHeightError { value: x });
        } else if h < 1 || h > 1000 {
            return Err(ShadowError::InvalidPositionOrHeightError { value: h });
        }

        Ok(Self { x, h })
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
}
