mod cache;
mod updates;
mod users;
mod logs;

pub use cache::CacheConfiguration;
pub use updates::UpdatesConfiguration;
pub use users::UsersConfiguration;

use std::any::Any;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::{Configuration, FileManager, JsonFile, implement_configuration};
use serde_json::error::Result as SerdeResult;
use crate::state::config_system::logs::LogsConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfiguration {
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

implement_configuration!(SystemConfiguration);
