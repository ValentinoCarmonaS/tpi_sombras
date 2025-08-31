use crate::{data::Data, shadow_error::ShadowError};
use std::io::{self, BufRead};

/// Parses a line into a tuple of two i32 values.
/// Returns error if the line is invalid or cannot be parsed.
pub fn parse_line(line: &str) -> Result<(i32, i32), ShadowError> {
    if line.is_empty() {
        return Err(ShadowError::ValorFaltante);
    }

    let line_parsed: Vec<&str> = line.split_whitespace().collect();

    if line_parsed.len() != 2 {
        return Err(ShadowError::ValorFaltante);
    }

    let first: i32 = match line_parsed[0].parse() {
        Ok(value) => value,
        Err(_) => return Err(ShadowError::NumeroInvalido),
    };

    let second: i32 = match line_parsed[1].parse() {
        Ok(value) => value,
        Err(_) => return Err(ShadowError::NumeroInvalido),
    };

    Ok((first, second))
}

/// Reads a line from stdin and returns a tuple of two i32 values.
fn read_line(
    lines: &mut dyn Iterator<Item = io::Result<String>>,
) -> Result<(i32, i32), ShadowError> {
    match lines.next() {
        Some(Ok(line)) => parse_line(&line),
        Some(Err(_)) => Err(ShadowError::LineaFaltante),
        None => Err(ShadowError::LineaFaltante),
    }
}

/// Reads all input from stdin and returns a Data struct.
/// Returns error if the input is invalid.
pub fn read_input() -> Result<Data, ShadowError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut data = Data::new();

    // Read the first line of the input
    let (theta, num_flatlanders) = read_line(&mut lines)?;
    if !(1..=100000).contains(&num_flatlanders) {
        return Err(ShadowError::FueraDeRango);
    }
    data.set_degrees(theta)?;

    // Read exactly N lines of the flatlanders
    for _ in 0..num_flatlanders {
        let (x, h) = read_line(&mut lines)?;
        data.set_flatlander(x, h)?;
    }
    Ok(data)
}

#[test]
fn test_read_line_success() {
    let line = "10 20";
    match parse_line(line) {
        Ok((first, second)) => {
            assert_eq!(first, 10);
            assert_eq!(second, 20);
        }
        Err(_) => panic!("Should not have returned an error"),
    }
}

#[test]
fn test_read_line_empty() {
    let line = "";
    match parse_line(line) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::ValorFaltante => (),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_read_line_invalid_len_greater() {
    let line = "10 20 30";
    match parse_line(line) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::ValorFaltante => (),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_read_line_invalid_len_less() {
    let line = "10";
    match parse_line(line) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::ValorFaltante => (),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_read_line_parse_int_error_first() {
    let line = "a 20";
    match parse_line(line) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::NumeroInvalido => (),
            _ => panic!("Incorrect error type"),
        },
    }
}

#[test]
fn test_read_line_parse_int_error_second() {
    let line = "10 b";
    match parse_line(line) {
        Ok(_) => panic!("Should have returned an error"),
        Err(e) => match e {
            ShadowError::NumeroInvalido => (),
            _ => panic!("Incorrect error type"),
        },
    }
}
