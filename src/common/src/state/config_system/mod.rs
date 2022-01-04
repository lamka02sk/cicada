mod cache;
mod updates;
mod users;
mod logs;

pub use cache::CacheConfiguration;
pub use updates::UpdatesConfiguration;
pub use users::UsersConfiguration;

use std::any::Any;
use std::error::Error;
use log::{error, info};
use serde::{Deserialize, Serialize};
use crate::{Configuration, FileManager, JsonFile, implement_configuration, CicadaResult, AppError};
use serde_json::error::Result as SerdeResult;
use crate::crypto::random::token;
use crate::state::config_system::logs::LogsConfiguration;

const TOKEN_STRENGTH: usize = 128;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemConfiguration {
    _filename: Option<String>,
    pub name: String,
    pub hostname: String,
    pub bind: Vec<String>,
    pub port: u16,
    pub workers: usize,
    pub token: Option<String>,
    pub frontend: Option<String>,
    pub cors: Option<Vec<String>>,
    pub cache: CacheConfiguration,
    pub logs: LogsConfiguration,
    pub updates: UpdatesConfiguration,
    pub users: UsersConfiguration
}

impl SystemConfiguration {

    pub fn generate_token(&mut self) {

        if self.token.is_some() {
            return;
        }

        info!("Generating application token");

        self.token = match token(TOKEN_STRENGTH) {
            Ok(token) => Some(token),
            Err(error) => {
                error!("Application token could not be generated: {}", error);
                panic!("Application token could not be generated: {}", error);
            }
        };

        if let Err(error) = self.save() {
            error!("Application token could not be written into configuration: {}", error);
            panic!("Application token could not be written into configuration: {}", error);
        }

    }

}

implement_configuration!(SystemConfiguration);

#[cfg(test)]
mod test {

    use std::fs::copy;
    use base64::decode;
    use crate::state::config_system::TOKEN_STRENGTH;
    use crate::SystemConfiguration;
    use crate::state::Configuration;

    fn get_config(cp: bool) -> Box<dyn Configuration> {

        if cp {
            copy("../../config/system.default.json", "../../testfiles/system_config.json").unwrap();
        }

        SystemConfiguration::new("../../testfiles/system_config.json").unwrap()

    }

    #[test]
    fn test_generate_token() {

        let mut config = get_config(true);
        assert!(config.as_any().downcast_ref::<SystemConfiguration>().unwrap().token.is_none());

        config.as_any_mut().downcast_mut::<SystemConfiguration>().unwrap().generate_token();

        let config = get_config(false);
        assert!(config.as_any().downcast_ref::<SystemConfiguration>().unwrap().token.is_some());

        let token = decode(config.as_any().downcast_ref::<SystemConfiguration>().unwrap().token.as_ref().unwrap());

        assert!(token.is_ok());
        assert_eq!(token.unwrap().len(), TOKEN_STRENGTH);

    }

}
