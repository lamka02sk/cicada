mod email;
mod slack;

pub use email::EmailNotificationsConfiguration;
pub use slack::SlackNotificationsConfiguration;

use std::any::Any;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::{Configuration, FileManager, JsonFile, implement_configuration};
use serde_json::error::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationsConfiguration {
    pub email: EmailNotificationsConfiguration,
    pub slack: SlackNotificationsConfiguration
}

implement_configuration!(NotificationsConfiguration);
