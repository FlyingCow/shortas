use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;

/// Represents a click stream item from the analytics database
#[derive(Clone, Default, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamItem {
    /// Unique identifier for the click stream item
    pub id: String,
    /// Owner of the route
    pub owner_id: String,
    /// Creator of the route
    pub creator_id: String,
    /// Route identifier
    pub route_id: String,
    /// Workspace identifier
    pub workspace_id: String,
    /// Timestamp when the click occurred
    pub created: DateTime<Utc>,
    /// Destination URL
    pub dest: String,
    /// IP address of the clicker
    pub ip: String,
    /// Geographic continent (optional)
    pub continent: Option<String>,
    /// Geographic country (optional)
    pub country: Option<String>,
    /// Geographic location (optional)
    pub location: Option<String>,
    /// Operating system family (optional)
    pub os_family: Option<String>,
    /// Operating system version (optional)
    pub os_version: Option<String>,
    /// User agent family (optional)
    pub user_agent_family: Option<String>,
    /// User agent version (optional)
    pub user_agent_version: Option<String>,
    /// Device brand (optional)
    pub device_brand: Option<String>,
    /// Device family (optional)
    pub device_family: Option<String>,
    /// Device model (optional)
    pub device_model: Option<String>,
    /// First session timestamp (optional)
    pub session_first: Option<DateTime<Utc>>,
    /// Number of clicks in session (optional)
    pub session_clicks: Option<u128>,
    /// Whether this is a unique click
    pub is_unique: bool,
    /// Whether this click is from a bot
    pub is_bot: bool,
}

/// Query parameters for filtering click stream data
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamQuery {
    /// Filter by owner ID
    pub owner_id: Option<String>,
    /// Filter by creator ID
    pub creator_id: Option<String>,
    /// Filter by route ID
    pub route_id: Option<String>,
    /// Filter by workspace ID
    pub workspace_id: Option<String>,
    /// Filter by creation date (start)
    pub created_from: Option<DateTime<Utc>>,
    /// Filter by creation date (end)
    pub created_to: Option<DateTime<Utc>>,
    /// Limit number of results
    pub limit: Option<u32>,
    /// Offset for pagination
    pub offset: Option<u32>,
}

impl Default for ClickStreamQuery {
    fn default() -> Self {
        Self {
            owner_id: None,
            creator_id: None,
            route_id: None,
            workspace_id: None,
            created_from: None,
            created_to: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

impl ClickStreamQuery {
    /// Create a new query with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set owner ID filter
    pub fn with_owner_id(mut self, owner_id: String) -> Self {
        self.owner_id = Some(owner_id);
        self
    }

    /// Set creator ID filter
    pub fn with_creator_id(mut self, creator_id: String) -> Self {
        self.creator_id = Some(creator_id);
        self
    }

    /// Set route ID filter
    pub fn with_route_id(mut self, route_id: String) -> Self {
        self.route_id = Some(route_id);
        self
    }

    /// Set workspace ID filter
    pub fn with_workspace_id(mut self, workspace_id: String) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }

    /// Set date range filter
    pub fn with_date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.created_from = Some(from);
        self.created_to = Some(to);
        self
    }

    /// Set pagination
    pub fn with_pagination(mut self, limit: u32, offset: u32) -> Self {
        self.limit = Some(limit);
        self.offset = Some(offset);
        self
    }

    /// Check if query has any filters
    pub fn has_filters(&self) -> bool {
        self.owner_id.is_some()
            || self.creator_id.is_some()
            || self.route_id.is_some()
            || self.workspace_id.is_some()
            || self.created_from.is_some()
            || self.created_to.is_some()
    }
}

/// Response wrapper for click stream queries
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamResponse {
    /// List of click stream items
    pub items: Vec<ClickStreamItem>,
    /// Total count of items (for pagination)
    pub total: u64,
    /// Current page offset
    pub offset: u32,
    /// Current page limit
    pub limit: u32,
    /// Whether there are more pages
    pub has_more: bool,
}

