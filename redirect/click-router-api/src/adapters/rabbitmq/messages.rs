use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeAction {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteChangedMessage {
    pub switch: String,
    pub link: String,
    pub action: ChangeAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsChangedMessage {
    pub user_id: String,
    pub action: ChangeAction,
}
