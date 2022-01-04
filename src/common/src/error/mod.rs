use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Mutex;
use log::error;
use serde_json::Value;

#[derive(Debug)]
pub enum CicadaErrorKind {
    Hidden(CicadaError),
    Public(CicadaError)
}

#[derive(Debug)]
pub struct CicadaCustomError {
    pub identifier: String,
    pub description: String
}

#[derive(Debug)]
pub struct CicadaHttpError {
    pub code: u16,
    pub message: Option<String>
}

#[derive(Debug)]
pub struct CicadaError {
    // pub kind: CicadaErrorKind,
    pub http: Option<CicadaHttpError>,
    pub custom: Option<CicadaCustomError>,
    pub source: Option<Box<dyn Error>>
}

impl CicadaError {

    pub fn new<T>(identifier: &str, description: &str) -> CicadaResult<T> {
        Err(CicadaErrorKind::Hidden(CicadaError {
            // kind: CicadaErrorKind::Hidden,
            http: Some(CicadaHttpError {
                code: 500,
                message: None
            }),
            custom: Some(CicadaCustomError {
                identifier: identifier.to_string(),
                description: description.to_string()
            }),
            source: None
        }))
    }

    // fn _http<T>(kind: CicadaErrorKind, code: u16, message: Option<&str>) -> CicadaResult<T> {
    //     Err(CicadaError {
    //         kind,
    //         http: Some(CicadaHttpError {
    //             code,
    //             message: match message {
    //                 Some(message) => Some(message.to_string()),
    //                 _ => None
    //             }
    //         }),
    //         custom: None,
    //         source: None
    //     })
    // }

    // pub fn pub_http<T>(code: u16, message: Option<&str>) -> CicadaResult<T> {
    //     CicadaError::_http(CicadaErrorKind::Public, code, message)
    // }

    pub fn http<T>(code: u16, message: Option<&str>) -> CicadaResult<T> {
        Err(CicadaErrorKind::Public(CicadaError {
            // kind: CicadaErrorKind::Public,
            http: Some(CicadaHttpError {
                code,
                message: match message {
                    Some(message) => Some(message.to_string()),
                    _ => None
                }
            }),
            custom: None,
            source: None
        }))
    }

}

impl Display for CicadaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        if let Some(error) = &self.custom {
            write!(f, "Application error [{}]: {}", error.identifier, error.description);
        }

        if let Some(error) = &self.source {
            write!(f, "Internal error: {}", error);
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
            write!(f, "Application error [{}]: {}", error.identifier, error.description);
        }

        if let Some(error) = &value.source {
            write!(f, "Internal error: {}", error);
        }

        Ok(())

    }
}

impl Into<CicadaResponse> for AppError {
    fn into(self) -> CicadaResponse {
        Err(CicadaErrorKind::Hidden(self))
    }
}

pub use CicadaError as AppError;

pub type CicadaResponse = Result<Value, CicadaErrorKind>;
pub type CicadaResult<T> = Result<T, CicadaErrorKind>;

impl From<CicadaErrorKind> for CicadaError {
    fn from(error_kind: CicadaErrorKind) -> Self {
        match error_kind {
            CicadaErrorKind::Hidden(value) => value,
            CicadaErrorKind::Public(value) => value,
        }
    }
}

// pub type CicadaResponse = Result<Value, CicadaError>;
// pub type CicadaResult<T> = Result<T, AppError>;
//
// // CicadaError for handling HTTP errors
// #[derive(Debug)]
// pub struct CicadaError {
//     pub code: u16,
//     pub message: String
// }
//
// impl CicadaError {
//
//     pub fn new(code: u16, message: &str) -> CicadaResponse {
//         Err(CicadaError {
//             code, message: message.to_string()
//         })
//     }
//
//     pub fn internal(message: &str) -> CicadaResponse {
//         Self::new(500, message)
//     }
//
//     pub fn forbidden(message: &str) -> CicadaResponse {
//         Self::new(403, message)
//     }
//
// }
//
// impl Display for CicadaError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Error {}: {}", self.code, self.message)
//     }
// }
//
// impl Error for CicadaError {}
//
// impl From<AppError> for CicadaError {
//     fn from(error: AppError) -> Self {
//         CicadaError {
//             code: 500,
//             message: format!("{}", error)
//         }
//     }
// }
//
// // AppError for handling application errors
// #[derive(Debug)]
// pub struct AppError(String, String, Mutex<bool>);
//
// impl AppError {
//     pub fn new<T>(identifier: &str, description: &str) -> CicadaResult<T> {
//         Err(Self(identifier.to_string(), description.to_string(), Mutex::new(false)))
//     }
// }
//
// impl Display for AppError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         *self.2.lock().unwrap() = true;
//         write!(f, "[{}] {}", self.0, self.1)
//     }
// }
//
// impl Drop for AppError {
//     fn drop(&mut self) {
//         if !*self.2.lock().unwrap() {
//             error!("[{}] {}", self.0, self.1);
//         }
//     }
// }
//
// impl Error for AppError {}
//
// impl Into<CicadaResponse> for AppError {
//     fn into(self) -> CicadaResponse {
//         CicadaError::internal(&format!("{}", self))
//     }
// }