use serde::{Deserialize, Serialize};
use crate::enums::system_log::SystemLog;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogsConfiguration {
    pub level: SystemLog,
    pub file: String
}