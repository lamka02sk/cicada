use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use log::{error, info, warn};
use serde_json::Value;

#[derive(Debug)]
pub enum CicadaErrorKind {
    Hidden(CicadaError),
    Public(CicadaError)
}

impl CicadaErrorKind {

    pub fn get_http_code(&self) -> Option<u16> {
        match self {
            CicadaErrorKind::Hidden(value) => value.get_http_code(),
            CicadaErrorKind::Public(value) => value.get_http_code(),
        }
    }

    pub fn get_http_message(&self) -> Option<String> {
        match self {
            CicadaErrorKind::Hidden(value) => value.get_http_message(),
            CicadaErrorKind::Public(value) => value.get_http_message(),
        }
    }

}

#[derive(Debug)]
pub enum CicadaErrorLog {
    None,
    Info,
    Warn,
    Error
}

#[derive(Debug)]
pub enum CicadaHttpLog {
    Default,
    Custom(String)
}

#[derive(Debug)]
pub struct CicadaCustomError {
    pub identifier: String,
    pub description: String
}

#[derive(Debug)]
pub struct CicadaHttpError {
    pub code: u16,
    pub message: CicadaHttpLog
}

#[derive(Debug)]
pub struct CicadaError {
    pub log: CicadaErrorLog,
    pub http: Option<CicadaHttpError>,
    pub custom: Option<CicadaCustomError>,
    pub source: Option<Box<dyn Error>>
}

impl CicadaError {

    pub fn log(&self, message: &str) {
        match &self.log {
            CicadaErrorLog::Info => info!("{}", message),
            CicadaErrorLog::Warn => warn!("{}", message),
            CicadaErrorLog::Error => error!("{}", message),
            _ => {}
        }
    }

    pub fn get_http_code(&self) -> Option<u16> {

        if let None = self.http {
            return None;
        }

        Some(self.http.as_ref().unwrap().code)

    }

    pub fn get_http_message(&self) -> Option<String> {

        if let None = self.http {
            return None;
        }

        match &self.http.as_ref().unwrap().message {
            CicadaHttpLog::Default => None,
            CicadaHttpLog::Custom(message) => Some(message.to_string())
        }

    }

    pub fn make_public<T>(result: CicadaResult<T>) ->  CicadaResult<T> {

        if let Err(error) = result {
            return Err(match error {
                CicadaErrorKind::Hidden(error) => CicadaErrorKind::Public(error),
                CicadaErrorKind::Public(error) => CicadaErrorKind::Public(error)
            });
        }

        result

    }

    pub fn with_log<T>(log: CicadaErrorLog, result: CicadaResult<T>) ->  CicadaResult<T> {

        if let Err(error) = result {
            return Err(match error {
                CicadaErrorKind::Hidden(error) => {
                    let mut error = error;
                    error.log = log;
                    CicadaErrorKind::Hidden(error)
                },
                CicadaErrorKind::Public(error) => {
                    let mut error = error;
                    error.log = log;
                    CicadaErrorKind::Public(error)
                }
            });
        }

        result

    }

    pub fn new<T>(log: CicadaErrorLog, http: Option<CicadaHttpError>, custom: Option<CicadaCustomError>, source: Option<Box<dyn Error>>) -> CicadaResult<T> {
        Err(CicadaErrorKind::Hidden(CicadaError {
            log, http, custom, source
        }))
    }

    pub fn log_http<T>(log: CicadaErrorLog, code: u16, message: CicadaHttpLog) -> CicadaResult<T> {
        CicadaError::new(log, Some(CicadaHttpError {
            code, message
        }), None, None)
    }

    pub fn http<T>(code: u16, message: CicadaHttpLog) -> CicadaResult<T> {
        CicadaError::log_http(CicadaErrorLog::None, code, message)
    }

    pub fn log_custom<T>(log: CicadaErrorLog, identifier: &str, description: &str) -> CicadaResult<T> {
        CicadaError::new(log, None, Some(CicadaCustomError {
            identifier: identifier.to_string(),
            description: description.to_string()
        }), None)
    }

    pub fn custom<T>(identifier: &str, description: &str) -> CicadaResult<T> {
        CicadaError::log_custom(CicadaErrorLog::None, identifier, description)
    }

    pub fn log_real<T>(log: CicadaErrorLog, error: Box<dyn Error>) -> CicadaResult<T> {
        Err(CicadaErrorKind::Hidden(CicadaError::real_raw(log, error)))
    }

    pub fn real<T>(error: Box<dyn Error>) -> CicadaResult<T> {
        CicadaError::log_real(CicadaErrorLog::None, error)
    }

    pub fn real_raw(log: CicadaErrorLog, error: Box<dyn Error>) -> Self {
        CicadaError {
            log, http: None, custom: None, source: Some(error)
        }
    }

    pub fn too_many_requests<T>(message: CicadaHttpLog) -> CicadaResult<T> {
        CicadaError::log_http(CicadaErrorLog::Info, 429, message)
    }

    pub fn internal<T>(message: CicadaHttpLog) -> CicadaResult<T> {
        CicadaError::log_http(CicadaErrorLog::Error, 500, message)
    }

    pub fn forbidden<T>(message: CicadaHttpLog) -> CicadaResult<T> {
        CicadaError::log_http(CicadaErrorLog::Warn, 403, message)
    }

}

impl Display for CicadaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        if let Some(error) = &self.custom {
            write!(f, "Application error [{}]: {}", error.identifier, error.description)?;
        }

        if let Some(error) = &self.source {
            write!(f, "Internal error: {}", error)?;
        }

        Ok(())

    }
}

impl Display for CicadaErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let value = match self {
            CicadaErrorKind::Hidden(value) => value,
            CicadaErrorKind::Public(value) => value,
        };

        if let Some(error) = &value.custom {
            write!(f, "Application error [{}]: {}", error.identifier, error.description)?;
        }

        if let Some(error) = &value.source {
            write!(f, "Internal error: {}", error)?;
        }

        Ok(())

    }
}

pub use CicadaError as AppError;

pub type CicadaResponse = Result<Value, CicadaErrorKind>;
pub type CicadaResult<T> = Result<T, CicadaErrorKind>;

impl From<Box<dyn Error>> for CicadaError {
    fn from(error: Box<dyn Error>) -> Self {
        CicadaError::real_raw(CicadaErrorLog::None, error)
    }
}

impl Into<CicadaResponse> for CicadaError {
    fn into(self) -> CicadaResponse {
        Err(CicadaErrorKind::Hidden(self))
    }
}

impl From<CicadaErrorKind> for CicadaError {
    fn from(error_kind: CicadaErrorKind) -> Self {
        match error_kind {
            CicadaErrorKind::Hidden(value) => value,
            CicadaErrorKind::Public(value) => value,
        }
    }
}

impl<T> Into<CicadaErrorKind> for CicadaResult<T> {
    fn into(self) -> CicadaErrorKind {

        if self.is_ok() {
            return CicadaErrorKind::Hidden(CicadaError {
                log: CicadaErrorLog::Error,
                http: Some(CicadaHttpError {
                    code: 500,
                    message: "Cannot convert Ok variant of CicadaResult into CicadaErrorKind".into()
                }),
                custom: None,
                source: None
            });
        }

        self.err().unwrap()

    }
}

impl Into<CicadaHttpLog> for &str {
    fn into(self) -> CicadaHttpLog {
        CicadaHttpLog::Custom(self.to_string())
    }
}

impl Into<CicadaHttpLog> for String {
    fn into(self) -> CicadaHttpLog {
        CicadaHttpLog::Custom(self)
    }
}