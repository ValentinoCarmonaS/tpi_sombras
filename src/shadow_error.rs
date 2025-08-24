/// Represents all possible errors in the shadow calculation program.
#[derive(Debug)]
pub enum ShadowError {
    InvalidAngle { value: i32 },
    ReadLine,
    ParseInt,
    InvalidLenLine,
    InvalidNumFlatlanders { value_expected: i32, value_got: i32 },
    InvalidPositionOrHeight { value: i32 },
}

impl ShadowError {
    /// Prints the error message to stderr.
    pub fn show_error(&self) {
        match self {
            Self::InvalidPositionOrHeight { value } => {
                eprintln!("Invalid position or height: {}", value)
            }
            Self::InvalidAngle { value } => {
                eprintln!("Invalid angle: {}", value)
            }
            Self::InvalidLenLine => {
                eprintln!("Invalid length line, the lenght is not 2")
            }
            Self::ParseInt => {
                eprintln!("Error in parse int")
            }
            Self::InvalidNumFlatlanders {
                value_expected,
                value_got,
            } => {
                eprintln!(
                    "Invalid number of flatlanders, expected {} but got: {}",
                    value_expected, value_got
                )
            }
            Self::ReadLine => {
                eprintln!("Error in read line")
            }
        }
    }

}
