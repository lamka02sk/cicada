use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatesConfiguration {
    pub enabled: bool,
    pub frequency: Option<u32>
}