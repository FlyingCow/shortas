use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::model::RouteToVerify;

/// Request to create or update a route for verification.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateRouteRequest {
    /// The route ID from the management API
    pub id: String,
    /// The route link (e.g., "example.com/path")
    pub link: String,
    /// All destinations to verify (main + conditional)
    pub destinations: Vec<String>,
    /// Owner ID for notifications
    pub owner_id: Option<String>,
    /// Workspace ID for notifications
    pub workspace_id: Option<String>,
}

impl From<CreateRouteRequest> for RouteToVerify {
    fn from(req: CreateRouteRequest) -> Self {
        RouteToVerify::new(
            req.id,
            req.link,
            req.destinations,
            req.owner_id,
            req.workspace_id,
        )
    }
}

/// Route response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteDto {
    pub id: String,
    pub link: String,
    pub destinations: Vec<String>,
    pub status: String,
    pub blocked_reason: Option<String>,
    pub owner_id: Option<String>,
    pub workspace_id: Option<String>,
    pub last_safety_check: Option<i64>,
    pub next_safety_check: Option<i64>,
}

impl From<RouteToVerify> for RouteDto {
    fn from(route: RouteToVerify) -> Self {
        Self {
            id: route.id,
            link: route.link,
            destinations: route.destinations,
            status: route.status,
            blocked_reason: route.blocked_reason,
            owner_id: route.owner_id,
            workspace_id: route.workspace_id,
            last_safety_check: route.last_safety_check,
            next_safety_check: route.next_safety_check,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteListResponse {
    pub data: Vec<RouteDto>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(error: &str, code: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::new(
            "NOT_FOUND",
            "RESOURCE_NOT_FOUND",
            &format!("{} with id {} not found", resource, id),
        )
    }

    pub fn validation(field: &str, message: &str) -> Self {
        Self::new(
            "VALIDATION_ERROR",
            "INVALID_INPUT",
            &format!("{}: {}", field, message),
        )
    }

    pub fn conflict(message: &str) -> Self {
        Self::new("CONFLICT", "RESOURCE_EXISTS", message)
    }

    pub fn internal(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", "INTERNAL_SERVER_ERROR", message)
    }
}
