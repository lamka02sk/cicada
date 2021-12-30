use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum NotificationsEvents {
    #[serde(rename = "auth:login")]
    AuthLogin,
    #[serde(rename = "auth:signup")]
    AuthSignup,
    #[serde(rename = "deploy:fail")]
    DeployFail,
    #[serde(rename = "deploy:success")]
    DeploySuccess,
    #[serde(rename = "system:update")]
    SystemUpdate
}