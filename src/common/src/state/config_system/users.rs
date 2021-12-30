use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsersConfiguration {
    pub signup: bool,
    pub restrict: Option<Vec<String>>
}