use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemLog {
    None,
    Info,
    Warning,
    Error
}