use std::collections::HashMap;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;

use crate::core::RoutesStore;
use crate::model::Route;

#[derive(Clone, Debug)]
pub struct DynamoRoutesStore {
    client: Client,
    routes_table: String,
}

impl DynamoRoutesStore {
    pub fn new(sdk_config: &SdkConfig, routes_table: String) -> Self {
        Self {
            routes_table,
            client: Client::new(sdk_config),
        }
    }
}

#[async_trait::async_trait()]
impl RoutesStore for DynamoRoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.routes_table)
            .set_key(Some(HashMap::from([
                (
                    "link".to_string(),
                    AttributeValue::S(path.to_string()),
                ),
                (
                    "switch".to_string(),
                    AttributeValue::S(switch.to_string()),
                ),
            ])))
            .send()
            .await?;

        Ok(item.item.map(from_item).transpose()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::route::{RouteStatus, RoutingPolicy};

    /// Reads routes written by a click-router-api in an AWS run mode at
    /// `TEST_ROUTER_API_URL`, from `TEST_ROUTES_TABLE` at `TEST_DYNAMODB_URL`.
    #[tokio::test]
    #[ignore = "needs DynamoDB and click-router-api"]
    async fn reads_what_click_router_api_writes() {
        let env = |name: &str| std::env::var(name).unwrap();
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .endpoint_url(env("TEST_DYNAMODB_URL"))
            .load()
            .await;
        let store = DynamoRoutesStore::new(&config, env("TEST_ROUTES_TABLE"));
        let http = reqwest::Client::new();
        let put = |id: &'static str, link: &'static str, policy: serde_json::Value, status: &'static str| {
            let request = http.put(format!("{}/v1/routes/{id}", env("TEST_ROUTER_API_URL"))).json(&serde_json::json!({
                "switch": "main", "link": link, "dest": "https://e.com/a%20b", "destFormat": "Http",
                "code": 307, "status": status, "terminal": "External", "policy": policy,
                "properties": {
                    "ownerId": "o", "workspaceId": "w", "tags": ["a"], "custom": { "n": 1.5 },
                    "opengraph": false, "allowDebug": true, "trackConversions": true
                }
            }));
            async move {
                let response = request.send().await.unwrap();
                let status = response.status();
                assert!(status.is_success(), "{status} {}", response.text().await.unwrap());
            }
        };

        // Twice: created, then updated.
        for _ in 0..2 {
            put("plain", "e.com%2Fplain", "Basic".into(), "Active").await;
        }
        let route = store.get_route("main", "e.com%2Fplain").await.unwrap().unwrap();
        assert_eq!(route.dest.as_deref(), Some("https://e.com/a%20b"));
        assert_eq!(route.code, Some(307));
        assert!(matches!(route.status, RouteStatus::Active));
        assert!(matches!(route.policy, RoutingPolicy::Basic));
        let p = route.properties;
        assert_eq!((p.owner_id.as_deref(), p.workspace_id.as_deref()), (Some("o"), Some("w")));
        assert!(p.allow_debug && p.track_conversions && !p.conversion_opt_out);
        assert_eq!(p.custom, Some(serde_json::json!({ "n": 1.5 })));

        put("blocked", "e.com%2Fblocked", "Basic".into(), "Blocked").await;
        let route = store.get_route("main", "e.com%2Fblocked").await.unwrap().unwrap();
        assert!(matches!(route.status, RouteStatus::Blocked(_)));

        let policy = serde_json::json!({ "Conditional": [{
            "key": "Firefox", "dest": "https://e.com/ff", "condition": { "ua": { "EQ": "Firefox" } }
        }]});
        put("cond", "e.com%2Fcond", policy, "Active").await;
        let route = store.get_route("main", "e.com%2Fcond").await.unwrap().unwrap();
        let RoutingPolicy::Conditional(conditions) = route.policy else { panic!("{:?}", route.policy) };
        assert!(conditions[0].condition.ua.is_some());
        let branch = store.get_route(&conditions[0].key, "e.com%2Fcond").await.unwrap().unwrap();
        assert_eq!(branch.dest.as_deref(), Some("https://e.com/ff"));
        assert!(store.get_route("main", "e.com%2Fnone").await.unwrap().is_none());
    }
}
