use http::StatusCode;
use serde_json::Value;

use super::condition::Condition;

#[derive(Debug, Clone)]
pub enum RoutingTerminal {
    External,
    Internal,
    Middleware,
}

#[derive(Debug, Clone)]
pub enum DestinationFormat{
    Http,
    Native
}

#[derive(Debug, Clone)]
pub struct FileRouting {
    pub content_type: String
}

#[derive(Debug, Clone)]
pub struct ConditionalRouting {
    pub key: String,
    pub condition: Condition,
}

#[derive(Debug, Clone)]
pub struct ChallengeRouting {
    pub key: String,
    pub source: String,
    pub challenge_type: String

}

#[derive(Debug, Clone)]
pub enum RoutingPolicy {
    Basic,
    Conditional(Vec<ConditionalRouting>),
    Challenge(ChallengeRouting),
    File(FileRouting),
    Mirroring,
    Unknown
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum BlockedReason{
    Resoned(String),
    Unknown
}

#[derive(Debug, Clone)]
pub enum RouteStatus{
    Active,
    Blocked(BlockedReason)
}

#[derive(Debug, Clone)]
pub struct Route {
    pub switch: String,
    pub link: String,
    pub dest: Option<String>,
    pub dest_format: DestinationFormat,
    pub status_code: StatusCode,
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
        dest_format: DestinationFormat,
        status_code: StatusCode,
        status: RouteStatus,
        ttl: Option<u128>,
        terminal: RoutingTerminal,
        policy: RoutingPolicy,
        properties: RouteProperties,
    ) -> Self {
        Route {
            switch,
            link,
            dest,
            dest_format,
            status_code,
            status,
            ttl,
            terminal,
            policy,
            properties,
        }
    }
}
