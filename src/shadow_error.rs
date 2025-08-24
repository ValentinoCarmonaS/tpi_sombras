/// Represents all possible errors in the shadow calculation program.
#[derive(Debug)]
pub enum ShadowError {
    InvalidAngle { value: i32 },
    ReadLine,
    ParseInt,
    InvalidLenLine,
    InvalidNumFlatlanders,
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
            Self::InvalidNumFlatlanders => {
                eprintln!("Invalid number of flatlanders")
            }
            Self::ReadLine => {
                eprintln!("Error in read line")
            }
        }
    }
}
