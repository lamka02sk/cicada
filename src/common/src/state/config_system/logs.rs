use serde::{Deserialize, Serialize};
use crate::enums::system_log::SystemLog;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogsConfiguration {
    pub level: SystemLog,
    pub file: String
}