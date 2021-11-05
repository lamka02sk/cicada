mod cache;
mod updates;
mod users;

pub use cache::CacheConfiguration;
pub use updates::UpdatesConfiguration;
pub use users::UsersConfiguration;

use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::{Configuration, FileManager, JsonFile, implement_configuration};
use serde_json::error::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfiguration {
    pub name: String,
    pub hostname: String,
    pub bind: String,
    pub port: i16,
    pub log: String,
    pub workers: u8,
    pub token: Option<String>,
    pub frontend: Option<String>,
    pub cache: CacheConfiguration,
    pub updates: UpdatesConfiguration,
    pub users: UsersConfiguration
}

implement_configuration!(SystemConfiguration);
