use std::any::Any;
use std::error::Error;
use crate::{Configuration, FileManager, JsonFile, implement_configuration};
use serde::{Deserialize, Serialize};
use serde_json::error::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub port: i16,
    pub username: String,
    pub password: String,
    pub database: String
}

impl DatabaseConfiguration {

    pub fn get_database_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database)
    }

}

implement_configuration!(DatabaseConfiguration);
