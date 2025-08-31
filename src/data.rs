use std::f64::consts::PI;

use crate::{flatlander::Flatlander, shadow_error::ShadowError};

/// Stores the input data: angle and flatlanders.
#[derive(Debug)]
pub struct Data {
    theta: i32,
    flatlanders: Vec<Flatlander>,
}

impl Data {
    /// Creates a new Data struct with default values.
    pub fn new() -> Self {
        Self {
            theta: 0,
            flatlanders: Vec::new(),
        }
    }

    /// Sets the angle in degrees. Returns error if out of range.
    pub fn set_degrees(&mut self, theta: i32) -> Result<(), ShadowError> {
        if !(10..=80).contains(&theta) {
            return Err(ShadowError::InvalidAngle { value: theta });
        }

        self.theta = theta;
        Ok(())
    }

    /// Adds a flatlander to the list. Returns error if invalid.
    pub fn set_flatlander(&mut self, x: i32, h: i32) -> Result<(), ShadowError> {
        let flatlander = Flatlander::new(x, h)?;
        self.flatlanders.push(flatlander);

        Ok(())
    }

    /// Sorts the flatlanders by their x position.
    pub fn sort(&mut self) {
        self.flatlanders
            .sort_by_key(|flatlander| flatlander.get_x());
    }

    /// Calculates the total shadow length for all flatlanders.
    pub fn calculate_total_shadow_length(self) -> f64 {
        let theta_rad = (self.theta as f64) * PI / 180.0;
        let tan = theta_rad.tan();

        let mut ans = 0.0;
        let mut current_start = -1.0;
        let mut current_end = -1.0;

        for flatlander in self.flatlanders {
            let x = flatlander.get_x() as f64;
            let l = x + flatlander.calculate_shadow_length(tan);

            if current_start <= x && current_end >= x {
                current_end = current_end.max(l);
            }

            if x > current_end && current_start <= current_end {
                ans += current_end - current_start;
                current_start = x;
                current_end = l;
            }
        }

        ans + current_end - current_start
    }

    /// Returns the angle in degrees.
    #[allow(dead_code)]
    pub fn get_theta(&self) -> i32 {
        self.theta
    }

    /// Returns a reference to the flatlanders vector.
    #[allow(dead_code)]
    pub fn get_flatlanders(&self) -> &Vec<Flatlander> {
        &self.flatlanders
    }
}

#[test]
fn test_new() {
    let data = Data::new();
    assert_eq!(data.get_theta(), 0);
    assert!(data.get_flatlanders().is_empty());
}

#[test]
fn test_set_degrees() {
    let mut data = Data::new();
    let theta = 45;

    data.set_degrees(theta).unwrap();

    assert_eq!(
        data.get_theta(),
        theta,
        "Error in set degrees assert equals"
    );
}

#[test]
fn test_set_degrees_error_less_than_10() {
    let mut data = Data::new();
    let theta = 9;
    match data.set_degrees(theta) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidAngle { value } => assert_eq!(value, theta),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_set_degrees_error_greater_than_80() {
    let mut data = Data::new();
    let theta = 81;
    match data.set_degrees(theta) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidAngle { value } => assert_eq!(value, theta),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_set_flatlander() {
    let mut data = Data::new();
    let x = 0;
    let h = 10;

    match data.set_flatlander(x, h) {
        Ok(_) => (),
        Err(_) => panic!("Error in set flatlander"),
    }

    assert_eq!(
        data.flatlanders[0].get_x(),
        x,
        "Error in set flatlander assert equals x"
    );
    assert_eq!(
        data.flatlanders[0].get_h(),
        h,
        "Error in set flatlander assert equals h"
    );
}

#[test]
fn test_set_flatlander_error() {
    let mut data = Data::new();
    let x = -1;
    let h = 10;

    match data.set_flatlander(x, h) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidPositionOrHeight { value } => assert_eq!(value, x),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_sort() {
    let mut data = Data::new();
    if data.set_flatlander(10, 10).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(0, 10).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(5, 10).is_err() {
        panic!("Should not fail")
    }

    data.sort();

    assert_eq!(data.get_flatlanders()[0].get_x(), 0);
    assert_eq!(data.get_flatlanders()[1].get_x(), 5);
    assert_eq!(data.get_flatlanders()[2].get_x(), 10);
}

#[test]
fn test_get_flatlanders() {
    let mut data = Data::new();
    if data.set_flatlander(10, 10).is_err() {
        panic!("Should not fail")
    }
    assert_eq!(data.get_flatlanders().len(), 1);
}

#[test]
fn test_calculate_total_shadow_length_no_flatlanders() {
    let data = Data::new();
    let epsilon = 1e-4;
    let expected = 0.0;
    let actual = data.calculate_total_shadow_length();
    assert!(
        (expected - actual).abs() < epsilon,
        "Error, expected: {}, got: {}",
        expected,
        actual
    );
}

#[test]
fn test_calculate_total_shadow_length_one_flatlander() {
    let mut data = Data::new();
    if data.set_degrees(45).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(0, 10).is_err() {
        panic!("Should not fail")
    }

    let epsilon = 1e-4;
    let expected = 10.0;
    let actual = data.calculate_total_shadow_length();
    assert!(
        (expected - actual).abs() < epsilon,
        "Error, expected: {}, got: {}",
        expected,
        actual
    );
}

#[test]
fn test01_calculate_total_shadow_length() {
    let mut data = Data::new();
    if data.set_degrees(45).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(0, 10).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(5, 10).is_err() {
        panic!("Should not fail")
    }

    data.sort();
    let epsilon = 1e-4;
    let expected = 15.0;
    let actual = data.calculate_total_shadow_length();

    assert!(
        (expected - actual).abs() < epsilon,
        "Error in calculate total shadow length, expected: {}, got: {}",
        expected,
        actual
    );
}

#[test]
fn test02_calculate_total_shadow_length() {
    let mut data = Data::new();
    if data.set_degrees(30).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(0, 100).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(50, 150).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(100, 200).is_err() {
        panic!("Should not fail")
    }

    data.sort();
    let epsilon = 1e-4;
    let expected = 446.4101615137755;
    let actual = data.calculate_total_shadow_length();

    assert!(
        (expected - actual).abs() < epsilon,
        "Error in calculate total shadow length, expected: {}, got: {}",
        expected,
        actual
    );
}

#[test]
fn test03_calculate_total_shadow_length() {
    let mut data = Data::new();
    if data.set_degrees(45).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(0, 100).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(50, 150).is_err() {
        panic!("Should not fail")
    }
    if data.set_flatlander(100, 200).is_err() {
        panic!("Should not fail")
    }

    data.sort();
    let epsilon = 1e-4;
    let expected = 300.0;
    let actual = data.calculate_total_shadow_length();

    assert!(
        (expected - actual).abs() < epsilon,
        "Error in calculate total shadow length, expected: {}, got: {}",
        expected,
        actual
    );
}
