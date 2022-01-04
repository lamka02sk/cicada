use std::any::Any;
use std::error::Error;
use crate::{Configuration, FileManager, JsonFile, implement_configuration, CicadaResult, CicadaError, CicadaErrorLog};
use serde::{Deserialize, Serialize};
use serde_json::error::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfiguration {
    _filename: Option<String>,
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

#[cfg(test)]
mod test {

    use crate::{DatabaseConfiguration};

    const HOST: &str = "localhost";
    const PORT: i16 = 0;
    const USER: &str = "user";
    const PASS: &str = "secret";
    const DB: &str = "schema";

    fn get_sample_config() -> DatabaseConfiguration {
        DatabaseConfiguration {
            _filename: None,
            host: HOST.to_string(),
            port: PORT,
            username: USER.to_string(),
            password: PASS.to_string(),
            database: DB.to_string()
        }
    }

    #[test]
    fn generate_database_url() {
        assert_eq!(get_sample_config().get_database_url(), String::from("postgres://") + USER + ":" + PASS + "@" + HOST + ":" + &PORT.to_string() + "/" + DB);
    }

}

