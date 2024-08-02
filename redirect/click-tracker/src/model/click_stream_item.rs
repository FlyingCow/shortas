use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClickStreamItem {
    pub id: String,
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub created: DateTime<Utc>,
    pub dest: Option<String>,
    pub ip: Option<String>,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_family: Option<String>,
    pub device_model: Option<String>,
    pub first_click: Option<DateTime<Utc>>,
    pub is_uniqueu: bool,
    pub is_bot: bool,
}
