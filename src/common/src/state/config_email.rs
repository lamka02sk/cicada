use std::any::Any;
use std::error::Error;
use crate::{Configuration, FileManager, JsonFile, implement_configuration};
use serde::{Deserialize, Serialize};
use serde_json::error::Result as SerdeResult;
use crate::state::enums::email_encryption::EmailEncryption;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfiguration {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub port: i16,
    pub encryption: EmailEncryption,
    pub utf8: bool
}

implement_configuration!(EmailConfiguration);
