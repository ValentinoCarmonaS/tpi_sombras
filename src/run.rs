use crate::read_input::read_input;
use crate::shadow_error::ShadowError;

pub fn run() -> Result<f64, ShadowError> {
    // Read the input from stdin
    let mut input_list = read_input()?;

    // Sort the flatlanders by x
    input_list.sort();

    // Calculate the total shadow length;
    let ans = input_list.calculate_total_shadow_length();

    Ok(ans)
}
