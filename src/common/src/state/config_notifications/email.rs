use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::state::enums::notifications_events::NotificationsEvents;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailNotificationsConfiguration {
    pub enabled: bool,
    pub contacts: Vec<String>,
    pub events: HashMap<NotificationsEvents, String>
}