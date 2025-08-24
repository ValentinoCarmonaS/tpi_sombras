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

    pub fn calculate_shadow_length(&self, tan: f64) -> f64 {
        (self.h as f64) / tan
    }

    #[allow(dead_code)]
    pub fn get_x(&self) -> i32 {
        self.x
    }

    #[allow(dead_code)]
    pub fn get_h(&self) -> i32 {
        self.h
    }
}

#[test]
fn test_new() {
    let x = 0;
    let h = 10;

    match Flatlander::new(x, h) {
        Ok(flatlander) => {
            assert_eq!(
                flatlander.get_x(),
                x,
                "Error in new flatlander assert equals x"
            );
            assert_eq!(
                flatlander.get_h(),
                h,
                "Error in new flatlander assert equals h"
            )
        }
        Err(_) => return assert!(false, "Error in new flatlander"),
    }
}

#[test]
fn test_calculate_shadow_length1() {
    let x = 0;
    let h = 10;

    match Flatlander::new(x, h) {
        Ok(flatlander) => {
            let epsilon = 1e-4;

            // Angulo de 45
            let tan1 = 1.0;
            let expected1 = 10.0;
            let actual1 = flatlander.calculate_shadow_length(tan1);
            assert!(
                (expected1 - actual1).abs() < epsilon,
                "Error in calculate shadow length expected {}, got {}",
                expected1,
                actual1
            );
        }
        Err(_) => return assert!(false, "Error in new flatlander"),
    }
}

#[test]
fn test_calculate_shadow_length2() {
    let x = 0;
    let h = 10;

    match Flatlander::new(x, h) {
        Ok(flatlander) => {
            let epsilon = 1e-4;

            // Angulo de 30
            let tan2 = 0.57735;
            let expected2 = 17.32050;
            let actual2 = flatlander.calculate_shadow_length(tan2);
            assert!(
                (expected2 - actual2).abs() < epsilon,
                "Error in calculate shadow length expected {}, got {}",
                expected2,
                actual2
            );
        }
        Err(_) => return assert!(false, "Error in new flatlander"),
    }
}

#[test]
fn test_calculate_shadow_length3() {
    let x = 0;
    let h = 10;

    match Flatlander::new(x, h) {
        Ok(flatlander) => {
            let epsilon = 1e-4;

            // Angulo de 70
            let tan3 = 2.74747;
            let expected3 = 3.63970;
            let actual3 = flatlander.calculate_shadow_length(tan3);
            assert!(
                (expected3 - actual3).abs() < epsilon,
                "Error in calculate shadow length expected {}, got {}",
                expected3,
                actual3
            )
        }
        Err(_) => return assert!(false, "Error in new flatlander"),
    }
}

#[test]
fn test_new_invalid_x_less_than_0() {
    let x = -1;
    let h = 10;

    match Flatlander::new(x, h) {
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidPositionOrHeightError { value } => assert_eq!(value, x),
            _ => assert!(false, "Incorrect error type"),
        },
    }
}

#[test]
fn test_new_invalid_x_greater_than_310000() {
    let x = 310001;
    let h = 10;

    match Flatlander::new(x, h) {
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidPositionOrHeightError { value } => assert_eq!(value, x),
            _ => assert!(false, "Incorrect error type"),
        },
    }
}

#[test]
fn test_new_invalid_h_less_than_1() {
    let x = 10;
    let h = 0;

    match Flatlander::new(x, h) {
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidPositionOrHeightError { value } => assert_eq!(value, h),
            _ => assert!(false, "Incorrect error type"),
        },
    }
}

#[test]
fn test_new_invalid_h_greater_than_1000() {
    let x = 10;
    let h = 1001;

    match Flatlander::new(x, h) {
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidPositionOrHeightError { value } => assert_eq!(value, h),
            _ => assert!(false, "Incorrect error type"),
        },
    }
}
