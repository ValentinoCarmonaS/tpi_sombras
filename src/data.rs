use std::f64::consts::PI;

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

    #[allow(dead_code)]
    pub fn get_theta(&self) -> i32 {
        self.theta
    }

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

    match data.set_degrees(theta) {
        Ok(_) => (),
        Err(_) => return assert!(false, "Error in set degrees"),
    }

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
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidAngleError { value } => assert_eq!(value, theta),
            _ => assert!(false, "Incorrect error type"),
        },
    }
}

#[test]
fn test_set_degrees_error_greater_than_80() {
    let mut data = Data::new();
    let theta = 81;
    match data.set_degrees(theta) {
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidAngleError { value } => assert_eq!(value, theta),
            _ => assert!(false, "Incorrect error type"),
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
        Err(_) => return assert!(false, "Error in set flatlander"),
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
        Ok(_) => assert!(false, "Should have returned an error"),
        Err(e) => match e {
            ShadowError::InvalidPositionOrHeightError { value } => assert_eq!(value, x),
            _ => assert!(false, "Incorrect error type"),
        },
    }
}

#[test]
fn test_sort() {
    let mut data = Data::new();
    if let Err(_) = data.set_flatlander(10, 10) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(0, 10) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(5, 10) {
        assert!(false, "Should not fail")
    }

    data.sort();

    assert_eq!(data.get_flatlanders()[0].get_x(), 0);
    assert_eq!(data.get_flatlanders()[1].get_x(), 5);
    assert_eq!(data.get_flatlanders()[2].get_x(), 10);
}

#[test]
fn test_get_flatlanders() {
    let mut data = Data::new();
    if let Err(_) = data.set_flatlander(10, 10) {
        assert!(false, "Should not fail")
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
    if let Err(_) = data.set_degrees(45) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(0, 10) {
        assert!(false, "Should not fail")
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
    if let Err(_) = data.set_degrees(45) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(0, 10) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(5, 10) {
        assert!(false, "Should not fail")
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
    if let Err(_) = data.set_degrees(30) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(0, 100) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(50, 150) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(100, 200) {
        assert!(false, "Should not fail")
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
    if let Err(_) = data.set_degrees(45) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(0, 100) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(50, 150) {
        assert!(false, "Should not fail")
    }
    if let Err(_) = data.set_flatlander(100, 200) {
        assert!(false, "Should not fail")
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
