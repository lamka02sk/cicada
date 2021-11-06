use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::state::enums::notifications_events::NotificationsEvents;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackNotificationsConfiguration {
    pub enabled: bool,
    pub webhook: String,
    pub default_channels: Vec<String>,
    pub events: HashMap<NotificationsEvents, Option<Vec<String>>>
}