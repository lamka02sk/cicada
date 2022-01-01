use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Mutex;
use log::error;
use serde_json::Value;

pub type CicadaResponse = Result<Value, CicadaError>;
pub type CicadaResult<T> = Result<T, AppError>;

// CicadaError for handling HTTP errors
#[derive(Debug)]
pub struct CicadaError {
    pub code: u16,
    pub message: String
}

impl CicadaError {

    pub fn new(code: u16, message: &str) -> CicadaResponse {
        Err(CicadaError {
            code, message: message.to_string()
        })
    }

    pub fn internal(message: &str) -> CicadaResponse {
        Self::new(500, message)
    }

    pub fn forbidden(message: &str) -> CicadaResponse {
        Self::new(403, message)
    }

}

impl Display for CicadaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl Error for CicadaError {}

impl From<AppError> for CicadaError {
    fn from(error: AppError) -> Self {
        CicadaError {
            code: 500,
            message: format!("{}", error)
        }
    }
}

// AppError for handling application errors
#[derive(Debug)]
pub struct AppError(String, String, Mutex<bool>);

impl AppError {
    pub fn new<T>(identifier: &str, description: &str) -> CicadaResult<T> {
        Err(Self(identifier.to_string(), description.to_string(), Mutex::new(false)))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        *self.2.lock().unwrap() = true;
        write!(f, "[{}] {}", self.0, self.1)
    }
}

impl Drop for AppError {
    fn drop(&mut self) {
        if !*self.2.lock().unwrap() {
            error!("[{}] {}", self.0, self.1);
        }
    }
}

impl Error for AppError {}

impl Into<CicadaResponse> for AppError {
    fn into(self) -> CicadaResponse {
        CicadaError::internal(&format!("{}", self))
    }
}