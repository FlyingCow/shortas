use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::condition::Condition;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum RoutingTerminal {
    #[default]
    External,
    Internal,
    Middleware,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum DestinationFormat {
    #[default]
    Http,
    Native,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct FileRouting {
    pub content_type: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalRouting {
    pub key: String,
    pub condition: Condition,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub opengraph: bool,
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
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum BlockedReason {
    Resoned(String),
    #[default]
    Unknown,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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
    pub dest_format: DestinationFormat,
    pub code: Option<u16>,
    pub ttl: Option<u128>,

    pub status: RouteStatus,
    pub terminal: RoutingTerminal,
    pub policy: RoutingPolicy,
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
}
