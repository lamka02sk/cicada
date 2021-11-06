use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::copy;
use std::path::Path;
use std::sync::{Arc, Mutex};
use super::{TomlFile, FileManager};

mod config_database;
mod config_email;
mod config_system;
mod config_notifications;

pub use config_database::*;
pub use config_email::*;
pub use config_system::*;
pub use config_notifications::*;
pub mod enums;

const CONFIG_FILES: [&str; 4] = [
    "system",
    "email",
    "notifications",
    "database"
];

#[derive(Debug)]
pub struct Cicada {
    pub version: String,
    pub config: Arc<Mutex<HashMap<String, Box<dyn Configuration>>>>
}

impl Cicada {

    fn extract_version() -> String {

        let mut file = TomlFile::new("Cargo.toml");

        if let Err(error) = file.parse() {
            panic!("Version number extraction has failed: {}", error);
        }

        file.query("package.version").unwrap().as_str().unwrap().to_string()

    }

    fn initialize_configuration() -> HashMap<String, Box<dyn Configuration>> {

        fn get_filename(filename: &str) -> String {
            String::from("config/") + filename + ".json"
        }

        fn prepare_configuration_file(filename: &str) {

            let filename = Path::new(filename);

            // Check if configuration file exists
            if filename.is_file() {
                return;
            }

            // Check if default configuration file exists
            let default_filename = filename.to_string_lossy().replace(".json", ".default.json");
            let default_filename = Path::new(&default_filename);

            if !default_filename.is_file() {
               panic!("Could not create configuration file \"{}\" from default, because the default configuration file doesn't exist", filename.display());
            }

            // Copy default configuration
            if let Err(error) = copy(default_filename, filename) {
                panic!("Could not create configuration file \"{}\" from default: {}", filename.display(), error);
            }

        }

        CONFIG_FILES.iter().fold(HashMap::new(), |acc, &config| {

            let filename = get_filename(config);
            prepare_configuration_file(&filename);

            let configuration: Result<Box<dyn Configuration>, Box<dyn Error>> = match config {
                "email" => EmailConfiguration::new(&filename),
                "system" => SystemConfiguration::new(&filename),
                "notifications" => NotificationsConfiguration::new(&filename),
                "database" => DatabaseConfiguration::new(&filename),
                _ => panic!("Handler for configuration file \"{}\" is not defined", config)
            };

            if let Err(error) = configuration {
                panic!("Loading configuration file \"{}\" failed: {}", config, error);
            }

            let mut acc = acc;
            acc.insert(config.to_string(), configuration.unwrap());
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
    fn new(filename: &str) -> Result<Box<dyn Configuration>, Box<dyn Error>> where Self: Sized;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! implement_configuration { ($type:ty) => {

    impl Configuration for $type {

        fn new(filename: &str) -> Result<Box<dyn Configuration>, Box<dyn Error>> where Self: Sized {
            let immediate: SerdeResult<Self> = serde_json::from_reader(JsonFile::new(filename).get_reader()?);
            match immediate {
                Ok(config) => Ok(Box::new(config)),
                Err(error) => Err(Box::new(error))
            }
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

    }

}}

pub(crate) use implement_configuration;
