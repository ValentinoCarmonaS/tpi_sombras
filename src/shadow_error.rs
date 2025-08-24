/// Represents all possible errors in the shadow calculation program.
#[derive(Debug)]
pub enum ShadowError {
    InvalidAngleError { value: i32 },
    ReadLineError,
    ParseIntError,
    InvalidLenLineError,
    InvalidNumFlatlandersError { value_expected: i32, value_got: i32 },
    InvalidPositionOrHeightError { value: i32 },
}

impl ShadowError {
    /// Prints the error message to stderr.
    pub fn show_error(&self) {
        match self {
            Self::InvalidPositionOrHeightError { value } => {
                eprintln!("Invalid position or height: {}", value)
            }
            Self::InvalidAngleError { value } => {
                eprintln!("Invalid angle: {}", value)
            }
            Self::InvalidLenLineError => {
                eprintln!("Invalid length line, the lenght is not 2")
            }
            Self::ParseIntError => {
                eprintln!("Error in parse int")
            }
            Self::InvalidNumFlatlandersError {
                value_expected,
                value_got,
            } => {
                eprintln!(
                    "Invalid number of flatlanders, expected {} but got: {}",
                    value_expected, value_got
                )
            }
            Self::ReadLineError => {
                eprintln!("Error in read line")
            }
        }
    }
}
