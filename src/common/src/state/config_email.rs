use std::error::Error;
use crate::{Configuration, FileManager, JsonFile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfiguration {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub port: i16,
    pub utf8: bool
}

impl Configuration for EmailConfiguration {

    fn new(filename: &str) -> Result<Self, Box<dyn Error>> where Self: Sized {
        match serde_json::from_reader(JsonFile::new(filename).get_reader()?) {
            Ok(config) => Ok(config),
            Err(error) => Err(Box::new(error))
        }
    }

}
