#[derive(Debug)]
pub enum ShadowError {
    // no hay dos datos al principio !!!
    // error en el angulo !!!

    // error en la lectura de stdin
    // la linea actual tiene >< a 2 elementos
    // no hay la cantidad de flatlanders que se indican
    InvalidPositionOrHeightError { value: i32 },
}

impl ShadowError {
    fn fmt(&self) {
        match self {
            Self::InvalidPositionOrHeightError { value } => {
                eprintln!("Invalid position or height: {}", value)
            }
        }
    }
}
