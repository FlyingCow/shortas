use serde::{Deserialize, Serialize};
use serde_json::Value;
use salvo::oapi::ToSchema;

use super::condition::Condition;

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub enum RoutingTerminal {
    #[default]
    External,
    Internal,
    Middleware,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub enum DestinationFormat {
    #[default]
    Http,
    Native,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct FileRouting {
    pub content_type: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalRouting {
    pub key: String,
    pub condition: Condition,
    /// Destination URL for this condition branch
    pub dest: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct ChallengeRouting {
    pub key: String,
    pub source: String,
    pub challenge_type: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum RoutingPolicy {
    #[default]
    Basic,
    Conditional(Vec<ConditionalRouting>),
    Challenge(ChallengeRouting),
    File(FileRouting),
    Mirroring,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RouteProperties {
    pub route_id: Option<String>,
    pub domain_id: Option<String>,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub workspace_id: Option<String>,
    pub scripts: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub custom: Option<Value>,
    pub native: Option<Value>,
    pub bundling: Option<Value>,
    #[serde(default)]
    pub opengraph: bool,
    #[serde(default)]
    pub allow_debug: bool,
}

impl Default for RouteProperties {
    fn default() -> Self {
        Self {
            route_id: Default::default(),
            domain_id: Default::default(),
            owner_id: Default::default(),
            creator_id: Default::default(),
            workspace_id: Default::default(),
            scripts: Default::default(),
            tags: Default::default(),
            custom: Default::default(),
            native: Default::default(),
            bundling: Default::default(),
            opengraph: false,
            allow_debug: false,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub enum BlockedReason {
    Resoned(String),
    #[default]
    Unknown,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub enum RouteStatus {
    #[default]
    Active,
    Blocked(BlockedReason),
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Route {
    pub switch: String,
    pub link: String,
    pub dest: Option<String>,
    #[serde(default)]
    pub dest_format: DestinationFormat,
    pub code: Option<u16>,
    pub ttl: Option<u64>,
    #[serde(default)]
    pub status: RouteStatus,
    #[serde(default)]
    pub terminal: RoutingTerminal,
    #[serde(default)]
    pub policy: RoutingPolicy,
    #[serde(default)]
    pub properties: RouteProperties,
}

impl Route {
    pub fn new(
        switch: String,
        link: String,
        dest: Option<String>,
        properties: RouteProperties,
    ) -> Self {
        Route {
            switch,
            link,
            dest,
            properties,
            ..Default::default()
        }
    }

    /// Build a route family from a conditional route.
    /// Returns the master route plus child routes for each condition branch.
    /// The master route's conditional policy will have dest stripped (destinations only live in child routes).
    /// If the route is not conditional, returns just the route itself.
    pub fn build_family(self) -> Vec<Route> {
        match &self.policy {
            RoutingPolicy::Conditional(conditions) => {
                let mut family = Vec::with_capacity(conditions.len() + 1);

                // Create child routes for each condition (with their destinations)
                // Child routes do NOT get route_id - only master has it for lookup
                for cond in conditions {
                    let mut child_properties = self.properties.clone();
                    child_properties.route_id = None; // Clear route_id for child routes

                    let child = Route {
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
                    policy: RoutingPolicy::Conditional(stripped_conditions),
                    ..self
                };
                family.push(master);

                family
            }
            _ => vec![self],
        }
    }

    /// Check if this route has a conditional policy
    pub fn is_conditional(&self) -> bool {
        matches!(self.policy, RoutingPolicy::Conditional(_))
    }
}
