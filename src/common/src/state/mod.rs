use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::io::Read;
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use crate::JsonFile;
use crate::state::config_email::EmailConfiguration;

use super::{TomlFile, FileManager};

mod config_email;
mod config_system;

const CONFIG_FILES: [&str; 1] = [
    /*"system", */
    "email"
];

#[derive(Debug)]
pub struct Cicada {
    pub version: String,
    pub config: Arc<Mutex<HashMap<String, Box<dyn Configuration>>>>
}

impl Cicada {

    fn extract_version() -> String {

        let mut file = TomlFile::new("Cargo.toml");
        file.parse();

        file.query("package.version").unwrap().as_str().unwrap().to_string()

    }

    fn initialize_configuration() -> HashMap<String, Box<dyn Configuration>> {

        fn get_filename(filename: &str) -> String {
            String::from("config/") + filename + ".default.json"
        }

        CONFIG_FILES.iter().fold(HashMap::new(), |acc, &config| {

            let configuration = match config {
                "email" => EmailConfiguration::new(&get_filename(config)),
                _ => panic!("Handler for configuration file {} is not defined", config)
            };

            if let Err(error) = configuration {
                panic!("{}", error);
            }

            let mut acc = acc;
            acc.insert(config.to_string(), Box::new(configuration.unwrap()));
            acc

        })

    }

}

impl Cicada {

    pub fn new() -> Self {
        Cicada {
            version: Cicada::extract_version(),
            config: Arc::new(Mutex::new(
                Cicada::initialize_configuration()
            ))
        }
    }

}

pub trait Configuration: Debug {
    fn new(filename: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;
}