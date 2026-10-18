//! Route types for URL shortening and conditional routing.
//!
//! These types define the core route entity and its associated types,
//! including routing policies, route properties, and status.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::condition::Condition;

/// Terminal type indicating where the route ends.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub enum RoutingTerminal {
    #[default]
    External,
    Internal,
    Middleware,
}

/// Destination format for the route.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub enum DestinationFormat {
    #[default]
    Http,
    Native,
}

/// File routing configuration for serving static files.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct FileRouting {
    pub content_type: String,
}

/// Conditional routing with key, condition expression, and destination.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConditionalRouting {
    pub key: String,
    pub condition: Condition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
}

/// Challenge routing for captcha or other verification.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct ChallengeRouting {
    pub key: String,
    pub source: String,
    pub challenge_type: String,
}

/// Routing policy determining how a route behaves.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum RoutingPolicy {
    #[default]
    Basic,
    Conditional {
        conditions: Vec<ConditionalRouting>,
    },
    Challenge {
        challenge: Option<ChallengeRouting>,
    },
    File {
        file: Option<FileRouting>,
    },
    Mirroring,
    Unknown,
}

impl RoutingPolicy {
    /// Get the policy type as a string.
    pub fn policy_type(&self) -> &'static str {
        match self {
            RoutingPolicy::Basic => "Basic",
            RoutingPolicy::Conditional { .. } => "Conditional",
            RoutingPolicy::Challenge { .. } => "Challenge",
            RoutingPolicy::File { .. } => "File",
            RoutingPolicy::Mirroring => "Mirroring",
            RoutingPolicy::Unknown => "Unknown",
        }
    }
}

/// Reason why a route is blocked.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub enum BlockedReason {
    Reasoned(String),
    #[default]
    Unknown,
}

/// Route status indicating if it's active or blocked.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub enum RouteStatus {
    #[default]
    Active,
    Blocked(BlockedReason),
}

impl RouteStatus {
    /// Check if the route is active.
    pub fn is_active(&self) -> bool {
        matches!(self, RouteStatus::Active)
    }

    /// Get the status as a string for database storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            RouteStatus::Active => "Active",
            RouteStatus::Blocked(_) => "Blocked",
        }
    }
}

/// QR code settings for a route.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct QrSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_correction: Option<String>,
}

/// Route properties containing metadata and configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct RouteProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundling: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_settings: Option<QrSettings>,
    #[serde(default)]
    pub opengraph: bool,
    #[serde(default)]
    pub allow_debug: bool,
}

impl Default for RouteProperties {
    fn default() -> Self {
        Self {
            route_id: None,
            domain_id: None,
            owner_id: None,
            creator_id: None,
            workspace_id: None,
            scripts: None,
            tags: None,
            custom: None,
            native: None,
            bundling: None,
            qr_settings: None,
            opengraph: false,
            allow_debug: false,
        }
    }
}

/// Core route entity.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Route {
    pub id: Uuid,
    pub switch: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    #[serde(default)]
    pub dest_format: DestinationFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    #[serde(default)]
    pub status: RouteStatus,
    #[serde(default)]
    pub terminal: RoutingTerminal,
    #[serde(default)]
    pub policy: RoutingPolicy,
    #[serde(default)]
    pub properties: RouteProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<Uuid>,
}

impl Route {
    /// Create a new route with the given parameters.
    pub fn new(switch: String, link: String, dest: Option<String>, properties: RouteProperties) -> Self {
        Route {
            id: Uuid::new_v4(),
            switch,
            link,
            dest,
            properties,
            ..Default::default()
        }
    }

    /// Build a route family from a conditional route.
    ///
    /// Returns the master route plus child routes for each condition branch.
    /// The master route's conditional policy will have dest stripped (destinations only live in child routes).
    /// If the route is not conditional, returns just the route itself.
    pub fn build_family(self) -> Vec<Route> {
        match &self.policy {
            RoutingPolicy::Conditional { conditions } => {
                let mut family = Vec::with_capacity(conditions.len() + 1);

                // Create child routes for each condition (with their destinations)
                // Child routes do NOT get route_id - only master has it for lookup
                for cond in conditions {
                    let mut child_properties = self.properties.clone();
                    child_properties.route_id = None; // Clear route_id for child routes

                    let child = Route {
                        id: Uuid::new_v4(),
                        switch: cond.key.clone(),
                        link: self.link.clone(),
                        dest: cond.dest.clone(),
                        dest_format: self.dest_format.clone(),
                        code: self.code,
                        ttl: self.ttl,
                        status: self.status.clone(),
                        terminal: self.terminal.clone(),
                        policy: RoutingPolicy::Basic,
                        properties: child_properties,
                        domain_id: self.domain_id,
                    };
                    family.push(child);
                }

                // Build master route with dest stripped from conditions
                // (destinations are derived from child routes by key)
                let stripped_conditions: Vec<ConditionalRouting> = conditions
                    .iter()
                    .map(|c| ConditionalRouting {
                        key: c.key.clone(),
                        condition: c.condition.clone(),
                        dest: None,
                    })
                    .collect();

                let master = Route {
                    policy: RoutingPolicy::Conditional {
                        conditions: stripped_conditions,
                    },
                    ..self
                };
                family.push(master);

                family
            }
            _ => vec![self],
        }
    }

    /// Check if this route has a conditional policy.
    pub fn is_conditional(&self) -> bool {
        matches!(self.policy, RoutingPolicy::Conditional { .. })
    }

    /// Get the route's full URL path (domain + link).
    pub fn full_path(&self, domain_name: &str) -> String {
        format!("{}/{}", domain_name, self.link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition::StringCondition;

    #[test]
    fn test_route_creation() {
        let route = Route::new(
            "main".to_string(),
            "test-link".to_string(),
            Some("https://example.com".to_string()),
            RouteProperties::default(),
        );
        assert_eq!(route.switch, "main");
        assert_eq!(route.link, "test-link");
        assert!(route.dest.is_some());
    }

    #[test]
    fn test_route_family_basic() {
        let route = Route::new(
            "main".to_string(),
            "test".to_string(),
            Some("https://example.com".to_string()),
            RouteProperties::default(),
        );
        let family = route.build_family();
        assert_eq!(family.len(), 1);
    }

    #[test]
    fn test_route_family_conditional() {
        let mut route = Route::new(
            "main".to_string(),
            "test".to_string(),
            Some("https://default.com".to_string()),
            RouteProperties {
                route_id: Some("route-123".to_string()),
                ..Default::default()
            },
        );
        route.policy = RoutingPolicy::Conditional {
            conditions: vec![
                ConditionalRouting {
                    key: "cond-1".to_string(),
                    condition: Condition {
                        ua: Some(StringCondition::Eq("Chrome".to_string())),
                        ..Default::default()
                    },
                    dest: Some("https://chrome.com".to_string()),
                },
                ConditionalRouting {
                    key: "cond-2".to_string(),
                    condition: Condition {
                        ua: Some(StringCondition::Eq("Firefox".to_string())),
                        ..Default::default()
                    },
                    dest: Some("https://firefox.com".to_string()),
                },
            ],
        };

        let family = route.build_family();
        assert_eq!(family.len(), 3); // 2 children + 1 master

        // Check child routes don't have route_id
        let child1 = &family[0];
        assert!(child1.properties.route_id.is_none());
        assert_eq!(child1.switch, "cond-1");
        assert!(matches!(child1.policy, RoutingPolicy::Basic));

        // Check master route still has route_id
        let master = &family[2];
        assert_eq!(master.properties.route_id, Some("route-123".to_string()));
        assert!(matches!(master.policy, RoutingPolicy::Conditional { .. }));
    }

    #[test]
    fn test_routing_policy_serialization() {
        let policy = RoutingPolicy::Conditional {
            conditions: vec![ConditionalRouting {
                key: "test".to_string(),
                condition: Condition::default(),
                dest: Some("https://example.com".to_string()),
            }],
        };
        let json = serde_json::to_string(&policy).unwrap();
        assert!(json.contains("\"type\":\"Conditional\""));
    }
}
