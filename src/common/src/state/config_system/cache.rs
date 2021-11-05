use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheConfiguration {
    pub enabled: bool,
    pub path: Option<String>
}