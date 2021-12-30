use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SystemLog {
    Off,
    Info,
    Warning,
    Error
}

impl SystemLog {

    pub fn as_level(&self) -> LevelFilter {
        match self {
            SystemLog::Off => LevelFilter::Off,
            SystemLog::Info => LevelFilter::Info,
            SystemLog::Warning => LevelFilter::Warn,
            SystemLog::Error => LevelFilter::Error
        }
    }

}
