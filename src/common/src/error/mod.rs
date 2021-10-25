use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CicadaError(pub String);

impl Display for CicadaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CicadaError {}