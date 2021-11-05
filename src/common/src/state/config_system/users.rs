use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersConfiguration {
    pub signup: bool,
    pub restrict: Option<Vec<String>>
}