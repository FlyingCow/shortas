use serde::{Deserialize, Serialize};

use crate::dto::route_dto::RouteDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeAction {
    Created,
    Updated,
    Deleted,
}

/// Route change event payload: route id, public DTO, and optional private data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteChangedMessage {
    pub route_id: String,
    pub action: ChangeAction,
    /// All properties the management API exposes as route DTO.
    pub public: RouteDto,
    /// Reserved for future use (e.g. internal-only fields).
    #[serde(default)]
    pub private: RouteChangedPrivate,
}

/// Private payload for route events.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteChangedPrivate {
    /// Previous destination URL (only set on updates when dest changed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_dest: Option<String>,
}

impl RouteChangedMessage {
    pub fn from_route(route: &crate::model::route::Route, action: ChangeAction) -> Self {
        Self {
            route_id: route
                .properties
                .route_id
                .as_deref()
                .unwrap_or("")
                .to_string(),
            action,
            public: RouteDto::from(route),
            private: RouteChangedPrivate::default(),
        }
    }

    pub fn from_route_with_previous(
        route: &crate::model::route::Route,
        action: ChangeAction,
        previous_dest: Option<String>,
    ) -> Self {
        Self {
            route_id: route
                .properties
                .route_id
                .as_deref()
                .unwrap_or("")
                .to_string(),
            action,
            public: RouteDto::from(route),
            private: RouteChangedPrivate { previous_dest },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsChangedMessage {
    pub user_id: String,
    pub action: ChangeAction,
}
