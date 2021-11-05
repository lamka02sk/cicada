use std::error::Error;
use crate::{Configuration, FileManager, JsonFile, implement_configuration};
use serde::{Deserialize, Serialize};
use serde_json::error::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfiguration {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub port: i16,
    pub utf8: bool
}

implement_configuration!(EmailConfiguration);
