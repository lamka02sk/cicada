use std::any::Any;
use std::error::Error;
use crate::{Configuration, FileManager, JsonFile, implement_configuration, AppError};
use serde::{Deserialize, Serialize};
use serde_json::error::Result as SerdeResult;
use crate::state::enums::email_encryption::EmailEncryption;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailConfiguration {
    _filename: Option<String>,
    pub domain: String,
    pub username: String,
    pub password: String,
    pub port: i16,
    pub encryption: EmailEncryption,
    pub utf8: bool
}

implement_configuration!(EmailConfiguration);
