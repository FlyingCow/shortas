use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use chrono::{DateTime, Utc};
use crate::model::clickstream::ClickStreamItem;

/// Data Transfer Object for ClickStreamItem API responses
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct ClickStreamItemDto {
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
    pub session_clicks: Option<u64>,
    /// Whether this is a unique click
    pub is_unique: bool,
    /// Whether this click is from a bot
    pub is_bot: bool,
}

/// Response wrapper for click stream queries
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamResponseDto {
    /// List of click stream items
    pub items: Vec<ClickStreamItemDto>,
    /// Total count of items (for pagination)
    pub total: u64,
    /// Current page offset
    pub offset: u32,
    /// Current page limit
    pub limit: u32,
    /// Whether there are more pages
    pub has_more: bool,
}

impl From<ClickStreamItem> for ClickStreamItemDto {
    fn from(item: ClickStreamItem) -> Self {
        use crate::model::clickstream::{UNKNOWN, epoch_datetime};

        // Convert "_unknown" values to None for API responses
        let to_option = |s: String| {
            if s == UNKNOWN { None } else { Some(s) }
        };

        Self {
            id: item.id,
            owner_id: item.owner_id,
            creator_id: item.creator_id,
            route_id: item.route_id,
            workspace_id: item.workspace_id,
            created: item.created,
            dest: item.dest,
            ip: item.ip,
            continent: to_option(item.continent),
            country: to_option(item.country),
            location: to_option(item.location),
            os_family: to_option(item.os_family),
            os_version: to_option(item.os_version),
            user_agent_family: to_option(item.user_agent_family),
            user_agent_version: to_option(item.user_agent_version),
            device_brand: to_option(item.device_brand),
            device_family: to_option(item.device_family),
            device_model: to_option(item.device_model),
            session_first: if item.session_first == epoch_datetime() { None } else { Some(item.session_first) },
            session_clicks: if item.session_clicks == 0 { None } else { Some(item.session_clicks) },
            is_unique: item.is_unique,
            is_bot: item.is_bot,
        }
    }
}

impl From<&ClickStreamItem> for ClickStreamItemDto {
    fn from(item: &ClickStreamItem) -> Self {
        use crate::model::clickstream::{UNKNOWN, epoch_datetime};

        // Convert "_unknown" values to None for API responses
        let to_option = |s: &String| {
            if s == UNKNOWN { None } else { Some(s.clone()) }
        };

        Self {
            id: item.id.clone(),
            owner_id: item.owner_id.clone(),
            creator_id: item.creator_id.clone(),
            route_id: item.route_id.clone(),
            workspace_id: item.workspace_id.clone(),
            created: item.created,
            dest: item.dest.clone(),
            ip: item.ip.clone(),
            continent: to_option(&item.continent),
            country: to_option(&item.country),
            location: to_option(&item.location),
            os_family: to_option(&item.os_family),
            os_version: to_option(&item.os_version),
            user_agent_family: to_option(&item.user_agent_family),
            user_agent_version: to_option(&item.user_agent_version),
            device_brand: to_option(&item.device_brand),
            device_family: to_option(&item.device_family),
            device_model: to_option(&item.device_model),
            session_first: if item.session_first == epoch_datetime() { None } else { Some(item.session_first) },
            session_clicks: if item.session_clicks == 0 { None } else { Some(item.session_clicks) },
            is_unique: item.is_unique,
            is_bot: item.is_bot,
        }
    }
}

