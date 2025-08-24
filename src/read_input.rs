use crate::{data::Data, shadow_error::ShadowError};
use std::io::{self, BufRead};

pub fn read_line(line: &str) -> Result<(i32, i32), ShadowError> {
    if line.is_empty() {
        return Err(ShadowError::InvalidLenLineError);
    }

    let line_parsed: Vec<&str> = line.split_whitespace().collect();

    if line_parsed.len() != 2 {
        return Err(ShadowError::InvalidLenLineError);
    }

    let first: i32 = match line_parsed[0].parse() {
        Ok(value) => value,
        Err(_) => return Err(ShadowError::ParseIntError),
    };

    let second: i32 = match line_parsed[1].parse() {
        Ok(value) => value,
        Err(_) => return Err(ShadowError::ParseIntError),
    };

    Ok((first, second))
}

pub fn read_input() -> Result<Data, ShadowError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut data = Data::new();

    // Leer primera línea: theta N
    let (theta, num_flatlanders) = match lines.next() {
        Some(Ok(line)) => read_line(&line)?,
        Some(Err(_)) => return Err(ShadowError::ReadLineError),
        None => return Err(ShadowError::InvalidLenLineError),
    };

    data.set_degrees(theta)?;

    // Leer exactamente N líneas de flatlanders
    for i in 0..num_flatlanders {
        let (x, h) = match lines.next() {
            Some(Ok(line)) => read_line(&line)?,
            Some(Err(_)) => return Err(ShadowError::ReadLineError),
            None => return Err(ShadowError::InvalidNumFlatlandersError {
                value_expected: num_flatlanders,
                value_got: i,
            }),
        };

        data.set_flatlander(x, h)?;
    }

    Ok(data)
}