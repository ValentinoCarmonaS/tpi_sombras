/// Represents all possible errors in the shadow calculation program.
#[derive(Debug)]
pub enum ShadowError {
    FueraDeRango,
    LineaFaltante,
    NumeroInvalido,
    ValorFaltante,
}

impl ShadowError {
    /// Prints the error message to stderr.
    pub fn show_error(&self) {
        match self {
            Self::FueraDeRango => eprintln!("Error: \"Fuera de rango\""),
            Self::LineaFaltante => eprintln!("Error: \"Linea faltante\""),
            Self::NumeroInvalido => eprintln!("Error: \"Numero invalido\""),
            Self::ValorFaltante => eprintln!("Error: \"Valor faltante\""),
        }
    }
}
