use crate::{flatlander::Flatlander, shadow_error::ShadowError};

#[derive(Debug)]
pub struct Data {
    theta: i32,
    flatlanders: Vec<Flatlander>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            theta: 0,
            flatlanders: Vec::new(),
        }
    }

    pub fn set_degrees(&mut self, theta: i32) -> Result<(), ShadowError> {
        if theta < 10 || theta > 80 {
            return Err(ShadowError::InvalidAngleError { value: theta });
        }

        self.theta = theta;
        Ok(())
    }

    pub fn set_flatlander(&mut self, x: i32, h: i32) -> Result<(), ShadowError> {
        let flatlander = Flatlander::new(x, h)?;
        self.flatlanders.push(flatlander);

        Ok(())
    }

    pub fn sort(&mut self) {
        // Sort the flatlander by x
        self.flatlanders
            .sort_by_key(|flatlander| flatlander.get_x());
    }

    pub fn calculate_total_shadow_length(self) -> Result<f64, ShadowError> {
        !unimplemented!()
    }
}
