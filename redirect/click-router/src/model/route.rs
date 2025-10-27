use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::expression::Expression;

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
    pub condition: Expression,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_create_route_with_new() {
        let properties = RouteProperties::default();
        let route = Route::new(
            "main".to_string(),
            "test-link".to_string(),
            Some("https://example.com".to_string()),
            properties.clone(),
        );

        assert_eq!(route.switch, "main");
        assert_eq!(route.link, "test-link");
        assert_eq!(route.dest, Some("https://example.com".to_string()));
        assert!(matches!(route.status, RouteStatus::Active));
        assert!(matches!(route.terminal, RoutingTerminal::External));
        assert!(matches!(route.policy, RoutingPolicy::Basic));
    }

    #[test]
    fn should_create_default_route() {
        let route = Route::default();

        assert_eq!(route.switch, "");
        assert_eq!(route.link, "");
        assert_eq!(route.dest, None);
        assert!(matches!(route.status, RouteStatus::Active));
        assert!(matches!(route.terminal, RoutingTerminal::External));
        assert!(matches!(route.policy, RoutingPolicy::Basic));
        assert!(matches!(route.dest_format, DestinationFormat::Http));
    }

    #[test]
    fn should_create_default_route_properties() {
        let props = RouteProperties::default();

        assert_eq!(props.route_id, None);
        assert_eq!(props.domain_id, None);
        assert_eq!(props.owner_id, None);
        assert_eq!(props.creator_id, None);
        assert_eq!(props.workspace_id, None);
        assert_eq!(props.scripts, None);
        assert_eq!(props.tags, None);
        assert_eq!(props.custom, None);
        assert_eq!(props.native, None);
        assert_eq!(props.bundling, None);
        assert_eq!(props.opengraph, false);
        assert_eq!(props.allow_debug, false);
    }

    #[test]
    fn should_serialize_and_deserialize_route() {
        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties::default(),
        };

        let serialized = serde_json::to_string(&route).unwrap();
        let deserialized: Route = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.switch, "main");
        assert_eq!(deserialized.link, "test");
        assert_eq!(deserialized.dest, Some("https://example.com".to_string()));
        assert_eq!(deserialized.code, Some(302));
        assert_eq!(deserialized.ttl, Some(3600));
    }

    #[test]
    fn should_handle_routing_policy_variants() {
        let basic = RoutingPolicy::Basic;
        assert!(matches!(basic, RoutingPolicy::Basic));

        let conditional = RoutingPolicy::Conditional(vec![]);
        assert!(matches!(conditional, RoutingPolicy::Conditional(_)));

        let challenge = RoutingPolicy::Challenge(ChallengeRouting {
            key: "test".to_string(),
            source: "user".to_string(),
            challenge_type: "captcha".to_string(),
        });
        assert!(matches!(challenge, RoutingPolicy::Challenge(_)));

        let file = RoutingPolicy::File(FileRouting {
            content_type: "text/html".to_string(),
        });
        assert!(matches!(file, RoutingPolicy::File(_)));

        let mirroring = RoutingPolicy::Mirroring;
        assert!(matches!(mirroring, RoutingPolicy::Mirroring));

        let unknown = RoutingPolicy::Unknown;
        assert!(matches!(unknown, RoutingPolicy::Unknown));
    }

    #[test]
    fn should_handle_route_status_variants() {
        let active = RouteStatus::Active;
        assert!(matches!(active, RouteStatus::Active));

        let blocked_unknown = RouteStatus::Blocked(BlockedReason::Unknown);
        assert!(matches!(blocked_unknown, RouteStatus::Blocked(BlockedReason::Unknown)));

        let blocked_reason = RouteStatus::Blocked(BlockedReason::Resoned("spam".to_string()));
        if let RouteStatus::Blocked(BlockedReason::Resoned(reason)) = blocked_reason {
            assert_eq!(reason, "spam");
        } else {
            panic!("Expected Blocked with Resoned reason");
        }
    }

    #[test]
    fn should_handle_routing_terminal_variants() {
        let external = RoutingTerminal::External;
        assert!(matches!(external, RoutingTerminal::External));

        let internal = RoutingTerminal::Internal;
        assert!(matches!(internal, RoutingTerminal::Internal));

        let middleware = RoutingTerminal::Middleware;
        assert!(matches!(middleware, RoutingTerminal::Middleware));

        let default = RoutingTerminal::default();
        assert!(matches!(default, RoutingTerminal::External));
    }

    #[test]
    fn should_handle_destination_format_variants() {
        let http = DestinationFormat::Http;
        assert!(matches!(http, DestinationFormat::Http));

        let native = DestinationFormat::Native;
        assert!(matches!(native, DestinationFormat::Native));

        let default = DestinationFormat::default();
        assert!(matches!(default, DestinationFormat::Http));
    }

    #[test]
    fn should_serialize_route_properties_with_custom_data() {
        let props = RouteProperties {
            route_id: Some("route_123".to_string()),
            domain_id: Some("domain_456".to_string()),
            owner_id: Some("user_789".to_string()),
            creator_id: Some("user_789".to_string()),
            workspace_id: Some("ws_123".to_string()),
            scripts: Some(vec!["script1.js".to_string(), "script2.js".to_string()]),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            custom: Some(json!({"key": "value"})),
            native: Some(json!({"native_key": "native_value"})),
            bundling: Some(json!({"bundle": true})),
            opengraph: true,
            allow_debug: true,
        };

        let serialized = serde_json::to_string(&props).unwrap();
        let deserialized: RouteProperties = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.route_id, Some("route_123".to_string()));
        assert_eq!(deserialized.opengraph, true);
        assert_eq!(deserialized.allow_debug, true);
        assert!(deserialized.custom.is_some());
    }

    #[test]
    fn should_clone_route() {
        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            ..Default::default()
        };

        let cloned = route.clone();

        assert_eq!(cloned.switch, route.switch);
        assert_eq!(cloned.link, route.link);
        assert_eq!(cloned.dest, route.dest);
    }
}
