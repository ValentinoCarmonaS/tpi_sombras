#[derive(Debug)]
pub enum ShadowError {
    // el angulo es incorrecto !!!
    InvalidAngleError { value: i32 },

    // error en la lectura de stdin
    ReadLineError,

    // error en el parseo de str a int
    ParseIntError,

    // la linea actual tiene >< a 2 elementos
    InvalidLenLineError,

    // no hay la cantidad de flatlanders que se indican
    InvalidNumFlatlandersError { value_expected: i32, value_got: i32 },

    // Error en flatlander
    InvalidPositionOrHeightError { value: i32 },
}

impl ShadowError {
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
