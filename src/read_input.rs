use crate::{data::Data, shadow_error::ShadowError};
use std::io::{self, BufRead};

pub fn read_input() -> Result<Data, ShadowError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut num_flatlanders = 0;
    let mut act_num_flatlanders = 0;
    let mut data = Data::new();

    // Leer línea por línea
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let line_parsed: Vec<&str> = line.split(' ').collect();

        if line_parsed.len() != 2 {
            return Err(ShadowError::InvalidLenLineError);
        }

        let first: i32;
        let second: i32;

        // Parsing the first element
        match line_parsed[0].parse::<i32>() {
            Ok(value) => first = value,
            Err(_) => return Err(ShadowError::ParseIntError),
        }

        // Parsing the second element
        match line_parsed[1].parse::<i32>() {
            Ok(value) => second = value,
            Err(_) => return Err(ShadowError::ParseIntError),
        }

        if num_flatlanders == 0 {
            // If it is the first line
            data.set_degrees(first)?;
            num_flatlanders = second;
        } else {
            // If it is a flatlander
            data.set_flatlander(first, second)?;
            act_num_flatlanders += 1;
        }
    }

    if act_num_flatlanders != num_flatlanders {
        return Err(ShadowError::InvalidNumFlatlandersError {
            value_expected: num_flatlanders,
            value_got: act_num_flatlanders,
        });
    }

    Ok(data)
}
