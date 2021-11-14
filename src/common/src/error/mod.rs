use std::error::Error;
use std::fmt::{Display, Formatter};
use serde_json::Value;

pub type CicadaResponse = Result<Value, CicadaError>;

#[derive(Debug)]
pub struct CicadaError {
    pub code: u16,
    pub message: String
}

impl Display for CicadaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl Error for CicadaError {}
