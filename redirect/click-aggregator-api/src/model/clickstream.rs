use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;

/// Default value for unknown/missing string fields
pub const UNKNOWN: &str = "_unknown";

/// Default epoch timestamp for unknown/missing datetime fields
pub fn epoch_datetime() -> DateTime<Utc> {
    DateTime::from_timestamp(0, 0).unwrap()
}

/// Represents a click stream item from the analytics database
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
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
    /// Geographic continent (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub continent: String,
    /// Geographic country (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub country: String,
    /// Geographic location (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub location: String,
    /// Operating system family (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub os_family: String,
    /// Operating system version (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub os_version: String,
    /// User agent family (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub user_agent_family: String,
    /// User agent version (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub user_agent_version: String,
    /// Device brand (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub device_brand: String,
    /// Device family (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub device_family: String,
    /// Device model (defaults to "_unknown")
    #[serde(default = "default_unknown")]
    pub device_model: String,
    /// First session timestamp (defaults to epoch: 1970-01-01)
    #[serde(default = "epoch_datetime")]
    pub session_first: DateTime<Utc>,
    /// Number of clicks in session (defaults to 0)
    #[serde(default)]
    pub session_clicks: u64,
    /// Whether this is a unique click
    pub is_unique: bool,
    /// Whether this click is from a bot
    pub is_bot: bool,
}

impl Default for ClickStreamItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            owner_id: String::new(),
            creator_id: String::new(),
            route_id: String::new(),
            workspace_id: String::new(),
            created: Utc::now(),
            dest: String::new(),
            ip: String::new(),
            continent: UNKNOWN.to_string(),
            country: UNKNOWN.to_string(),
            location: UNKNOWN.to_string(),
            os_family: UNKNOWN.to_string(),
            os_version: UNKNOWN.to_string(),
            user_agent_family: UNKNOWN.to_string(),
            user_agent_version: UNKNOWN.to_string(),
            device_brand: UNKNOWN.to_string(),
            device_family: UNKNOWN.to_string(),
            device_model: UNKNOWN.to_string(),
            session_first: epoch_datetime(),
            session_clicks: 0,
            is_unique: false,
            is_bot: false,
        }
    }
}

/// Helper function for serde default
fn default_unknown() -> String {
    UNKNOWN.to_string()
}

impl ClickStreamItem {
    /// Check if a string field is unknown/missing
    pub fn is_unknown(value: &str) -> bool {
        value == UNKNOWN
    }

    /// Check if the session is unknown (epoch time)
    pub fn has_session(&self) -> bool {
        self.session_first != epoch_datetime()
    }

    /// Check if has valid geographic data
    pub fn has_geo_data(&self) -> bool {
        !Self::is_unknown(&self.country)
    }

    /// Check if has valid device data
    pub fn has_device_data(&self) -> bool {
        !Self::is_unknown(&self.device_family)
    }

    /// Check if has valid user agent data
    pub fn has_user_agent_data(&self) -> bool {
        !Self::is_unknown(&self.user_agent_family)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unknown() {
        assert!(ClickStreamItem::is_unknown(UNKNOWN));
        assert!(!ClickStreamItem::is_unknown("US"));
    }

    #[test]
    fn test_has_session() {
        let mut item = ClickStreamItem::default();
        assert!(!item.has_session(), "default session_first is epoch, so no session");

        item.session_first = Utc::now();
        assert!(item.has_session());
    }

    #[test]
    fn test_has_geo_data() {
        let mut item = ClickStreamItem::default();
        assert!(!item.has_geo_data());

        item.country = "US".to_string();
        assert!(item.has_geo_data());
    }

    #[test]
    fn test_has_device_data() {
        let mut item = ClickStreamItem::default();
        assert!(!item.has_device_data());

        item.device_family = "iPhone".to_string();
        assert!(item.has_device_data());
    }

    #[test]
    fn test_has_user_agent_data() {
        let mut item = ClickStreamItem::default();
        assert!(!item.has_user_agent_data());

        item.user_agent_family = "Chrome".to_string();
        assert!(item.has_user_agent_data());
    }

    #[test]
    fn test_default_item_has_unknown_geo_and_device_fields() {
        let item = ClickStreamItem::default();
        assert_eq!(item.continent, UNKNOWN);
        assert_eq!(item.country, UNKNOWN);
        assert_eq!(item.location, UNKNOWN);
        assert_eq!(item.os_family, UNKNOWN);
        assert_eq!(item.device_family, UNKNOWN);
        assert_eq!(item.session_clicks, 0);
        assert!(!item.is_unique);
        assert!(!item.is_bot);
    }

    #[test]
    fn test_deserialize_missing_optional_fields_use_defaults() {
        let json = r#"{
            "id": "1",
            "owner_id": "o1",
            "creator_id": "c1",
            "route_id": "r1",
            "workspace_id": "w1",
            "created": "2024-01-01T00:00:00Z",
            "dest": "https://example.com",
            "ip": "1.2.3.4",
            "is_unique": true,
            "is_bot": false
        }"#;
        let item: ClickStreamItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.continent, UNKNOWN);
        assert_eq!(item.country, UNKNOWN);
        assert_eq!(item.session_clicks, 0);
        assert_eq!(item.session_first, epoch_datetime());
        assert!(item.is_unique);
        assert!(!item.is_bot);
    }

    #[test]
    fn test_query_default_has_pagination_but_no_filters() {
        let query = ClickStreamQuery::default();
        assert_eq!(query.limit, Some(100));
        assert_eq!(query.offset, Some(0));
        assert!(!query.has_filters());
    }

    #[test]
    fn test_query_builder_sets_filters() {
        let query = ClickStreamQuery::new()
            .with_owner_id("owner-1".to_string())
            .with_route_id("route-1".to_string())
            .with_pagination(50, 10);

        assert!(query.has_filters());
        assert_eq!(query.owner_id, Some("owner-1".to_string()));
        assert_eq!(query.route_id, Some("route-1".to_string()));
        assert_eq!(query.limit, Some(50));
        assert_eq!(query.offset, Some(10));
    }

    #[test]
    fn test_query_with_date_range_counts_as_a_filter() {
        let from = epoch_datetime();
        let to = Utc::now();
        let query = ClickStreamQuery::new().with_date_range(from, to);
        assert!(query.has_filters());
        assert_eq!(query.created_from, Some(from));
        assert_eq!(query.created_to, Some(to));
    }
}

