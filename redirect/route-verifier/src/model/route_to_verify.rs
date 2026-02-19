use serde::{Deserialize, Serialize};

/// Simplified route model for verification purposes.
/// Contains only the fields needed to check destinations against Safe Browsing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteToVerify {
    /// The route ID from the management API (used to update status)
    #[serde(rename = "_id")]
    pub id: String,

    /// The route link (e.g., "example.com/path")
    pub link: String,

    /// All destinations to verify (main dest + conditional dests)
    #[serde(default)]
    pub destinations: Vec<String>,

    /// Owner ID for RabbitMQ notifications
    #[serde(default)]
    pub owner_id: Option<String>,

    /// Workspace ID for RabbitMQ notifications
    #[serde(default)]
    pub workspace_id: Option<String>,

    /// Current status: "Active" or "Blocked"
    #[serde(default = "default_status")]
    pub status: String,

    /// Reason for blocking (if blocked)
    #[serde(default)]
    pub blocked_reason: Option<String>,

    /// Timestamp of last safety check
    #[serde(default)]
    pub last_safety_check: Option<i64>,

    /// Timestamp for next scheduled check
    #[serde(default)]
    pub next_safety_check: Option<i64>,
}

fn default_status() -> String {
    "Active".to_string()
}

impl RouteToVerify {
    pub fn new(
        id: String,
        link: String,
        destinations: Vec<String>,
        owner_id: Option<String>,
        workspace_id: Option<String>,
    ) -> Self {
        Self {
            id,
            link,
            destinations,
            owner_id,
            workspace_id,
            status: "Active".to_string(),
            blocked_reason: None,
            last_safety_check: None,
            next_safety_check: None,
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.status == "Blocked"
    }
}
