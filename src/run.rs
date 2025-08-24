use crate::read_input::read_input;
use crate::shadow_error::ShadowError;

/// Runs the main logic: reads stdin input, sorts, and calculates the answer.
/// Returns the total shadow length or an error.
pub fn run() -> Result<f64, ShadowError> {
    // Read the input from stdin
    let mut input_list = read_input()?;

    // Sort the flatlanders by x
    input_list.sort();

    // Calculate the total shadow length;
    let ans = input_list.calculate_total_shadow_length();

    Ok(ans)
}
