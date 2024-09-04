use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClickStreamItem {
    pub id: String,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub route_id: Option<String>,
    pub workspace_id: Option<String>,
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
    pub session_first: Option<DateTime<Utc>>,
    pub session_clicks: Option<u128>,
    pub is_unique: bool,
    pub is_bot: bool,
}
