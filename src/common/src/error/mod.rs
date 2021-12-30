use std::error::Error;
use std::fmt::{Display, Formatter};
use log::error;
use serde_json::Value;

pub type CicadaResponse = Result<Value, CicadaError>;

// CicadaError for handling HTTP errors
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

// AppError for handling application errors
#[derive(Debug)]
pub struct AppError(String, String);

impl AppError {
    pub fn new(identifier: &str, description: &str) -> Self {
        error!("[{}] {}", identifier, description);
        Self(identifier.to_string(), description.to_string())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.0, self.1)
    }
}

impl Error for AppError {}
