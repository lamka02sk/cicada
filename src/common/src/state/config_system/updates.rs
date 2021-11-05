use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatesConfiguration {
    pub enabled: bool,
    pub frequency: Option<u32>
}