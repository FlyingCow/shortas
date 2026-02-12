use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeAction {
    Created,
    Updated,
    Deleted,
}

/// Route change event: route id, public DTO (as JSON), and private payload.
/// Matches the shape published by click-router-api; we only need switch/link from public for cache invalidation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteChangedMessage {
    pub route_id: String,
    pub action: ChangeAction,
    pub public: Value,
    #[serde(default)]
    pub private: Value,
}

impl RouteChangedMessage {
    /// Switch and link from the public DTO for cache invalidation.
    pub fn switch_link(&self) -> Option<(String, String)> {
        let switch = self.public.get("switch")?.as_str()?.to_string();
        let link = self.public.get("link")?.as_str()?.to_string();
        Some((switch, link))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsChangedMessage {
    pub user_id: String,
    pub action: ChangeAction,
}
