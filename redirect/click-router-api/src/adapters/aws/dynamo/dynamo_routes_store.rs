use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::{AttributeValue, Delete, Put, TransactWriteItem};
use aws_sdk_dynamodb::Client;
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, to_item};

use crate::core::RoutesStore;
use crate::model::Route;

type Item = HashMap<String, AttributeValue>;

/// Top-level copy of `properties.route_id`, for the index.
const ROUTE_ID: &str = "route_id";
const ROUTE_ID_INDEX: &str = "route-id-index";
/// The most DynamoDB allows in one transaction.
const MAX_TRANSACTION: usize = 100;

/// Routes as the serialized `Route`, which click-router reads the same way.
/// `link` and `switch` are the key, with the exact spelling used in lookups.
#[derive(Clone, Debug)]
pub struct DynamoRoutesStore {
    client: Client,
    routes_table: String,
}

fn route_item(route: &Route) -> Result<Item> {
    let mut item: Item = to_item(route)?;
    // Index keys can't be empty.
    if let Some(route_id) = route.properties.route_id.as_ref().filter(|id| !id.is_empty()) {
        item.insert(ROUTE_ID.into(), AttributeValue::S(route_id.clone()));
    }
    Ok(item)
}

impl DynamoRoutesStore {
    pub fn new(sdk_config: &SdkConfig, routes_table: String) -> Self {
        Self {
            routes_table,
            client: Client::new(sdk_config),
        }
    }

    fn key(switch: &str, link: &str) -> Item {
        HashMap::from([
            ("link".to_string(), AttributeValue::S(link.to_string())),
            ("switch".to_string(), AttributeValue::S(switch.to_string())),
        ])
    }

    async fn put(&self, route: &Route) -> Result<()> {
        self.client
            .put_item()
            .table_name(&self.routes_table)
            .set_item(Some(route_item(route)?))
            .send()
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait()]
impl RoutesStore for DynamoRoutesStore {
    async fn store_route(&self, route: &Route) -> Result<()> {
        self.put(route).await
    }

    async fn update_route(&self, route: &Route) -> Result<()> {
        self.put(route).await
    }

    async fn delete_route(&self, route: &Route) -> Result<()> {
        self.delete_route_by_switch_and_link(&route.switch, &route.link)
            .await
    }

    async fn get_route_by_route_id(&self, route_id: &str) -> Result<Option<Route>> {
        let output = self
            .client
            .query()
            .table_name(&self.routes_table)
            .index_name(ROUTE_ID_INDEX)
            .key_condition_expression("#id = :id")
            .expression_attribute_names("#id", ROUTE_ID)
            .expression_attribute_values(":id", AttributeValue::S(route_id.to_string()))
            .limit(1)
            .send()
            .await?;
        let item = output.items.unwrap_or_default().pop();
        Ok(item.map(from_item).transpose()?)
    }

    async fn invalidate_route(&self, _switch: &str, _link: &str) -> Result<()> {
        // Nothing is cached here
        Ok(())
    }

    async fn get_route(&self, switch: &str, link: &str) -> Result<Option<Route>> {
        let output = self
            .client
            .get_item()
            .table_name(&self.routes_table)
            .set_key(Some(Self::key(switch, link)))
            .consistent_read(true)
            .send()
            .await?;
        Ok(output.item.map(from_item).transpose()?)
    }

    async fn get_routes_by_link(&self, link: &str) -> Result<Vec<Route>> {
        let mut pages = self
            .client
            .query()
            .table_name(&self.routes_table)
            .key_condition_expression("link = :link")
            .expression_attribute_values(":link", AttributeValue::S(link.to_string()))
            .consistent_read(true)
            .into_paginator()
            .items()
            .send();

        let mut routes = Vec::new();
        while let Some(item) = pages.next().await {
            routes.push(from_item(item?)?);
        }
        Ok(routes)
    }

    async fn delete_routes_by_link(&self, link: &str) -> Result<u64> {
        let routes = self.get_routes_by_link(link).await?;
        for route in &routes {
            self.delete_route(route).await?;
        }
        Ok(routes.len() as u64)
    }

    async fn store_route_family(&self, routes: &[Route]) -> Result<()> {
        let Some(first) = routes.first() else {
            return Ok(());
        };

        // A transaction can't write a key twice; the last one wins, as in
        // MongoDB.
        let current: HashMap<&str, &Route> =
            routes.iter().map(|r| (r.switch.as_str(), r)).collect();
        let stale: HashSet<String> = self
            .get_routes_by_link(&first.link)
            .await?
            .into_iter()
            .map(|r| r.switch)
            .filter(|switch| !current.contains_key(switch.as_str()))
            .collect();

        let mut writes = Vec::new();
        for route in current.values() {
            let put = Put::builder()
                .table_name(&self.routes_table)
                .set_item(Some(route_item(route)?))
                .build()?;
            writes.push(TransactWriteItem::builder().put(put).build());
        }
        for switch in &stale {
            let delete = Delete::builder()
                .table_name(&self.routes_table)
                .set_key(Some(Self::key(switch, &first.link)))
                .build()?;
            writes.push(TransactWriteItem::builder().delete(delete).build());
        }
        anyhow::ensure!(
            writes.len() <= MAX_TRANSACTION,
            "a route can't have more than {MAX_TRANSACTION} branches, including removed ones"
        );

        self.client
            .transact_write_items()
            .set_transact_items(Some(writes))
            .send()
            .await
            .context("storing the route family failed")?;
        Ok(())
    }

    async fn delete_route_by_switch_and_link(&self, switch: &str, link: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.routes_table)
            .set_key(Some(Self::key(switch, link)))
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::condition::Condition;
    use crate::model::route::*;
    use aws_sdk_dynamodb::types::{
        AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType,
        Projection, ProjectionType, ScalarAttributeType,
    };
    use serde_json::json;

    const LINK: &str = "example.com%2Fabc";

    async fn store(table: &str) -> DynamoRoutesStore {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .endpoint_url(std::env::var("TEST_DYNAMODB_URL").unwrap())
            .load()
            .await;
        let store = DynamoRoutesStore::new(&config, table.into());
        let _ = store.client.delete_table().table_name(table).send().await;
        let key = |name: &str, key: KeyType| {
            KeySchemaElement::builder().attribute_name(name).key_type(key).build().unwrap()
        };
        let mut create = store.client.create_table().table_name(table).billing_mode(BillingMode::PayPerRequest);
        for name in ["link", "switch", ROUTE_ID] {
            create = create.attribute_definitions(
                AttributeDefinition::builder().attribute_name(name).attribute_type(ScalarAttributeType::S).build().unwrap(),
            );
        }
        create
            .key_schema(key("link", KeyType::Hash))
            .key_schema(key("switch", KeyType::Range))
            .global_secondary_indexes(
                GlobalSecondaryIndex::builder()
                    .index_name(ROUTE_ID_INDEX)
                    .key_schema(key(ROUTE_ID, KeyType::Hash))
                    .projection(Projection::builder().projection_type(ProjectionType::All).build())
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
        store
    }

    fn full() -> Route {
        Route {
            switch: "main".into(),
            link: LINK.into(),
            dest: Some("https://e.com/a%20b?x=1&y=%2F".into()),
            dest_format: DestinationFormat::Native,
            code: Some(307),
            ttl: Some(60),
            status: RouteStatus::Blocked(BlockedReason::Resoned("unsafe".into())),
            terminal: RoutingTerminal::Internal,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties {
                route_id: Some("r".into()),
                domain_id: Some("d".into()),
                owner_id: Some("o".into()),
                creator_id: Some("c".into()),
                workspace_id: Some("w".into()),
                scripts: Some(vec![]),
                tags: Some(vec!["a".into(), "a".into()]),
                custom: Some(json!({ "n": 1.5, "nested": [null, { "k": true }] })),
                native: Some(json!({})),
                bundling: Some(json!("x")),
                opengraph: true,
                allow_debug: true,
                conversion_opt_out: true,
                track_conversions: true,
            },
        }
    }

    #[tokio::test]
    #[ignore = "needs DynamoDB"]
    async fn every_field_round_trips() {
        let store = store("routes-round-trip").await;
        let policies = [
            RoutingPolicy::Basic,
            RoutingPolicy::Challenge(ChallengeRouting {
                key: "Token.Case".into(),
                source: "acme".into(),
                challenge_type: "http-01".into(),
            }),
            RoutingPolicy::File(FileRouting { content_type: "text/plain".into() }),
            RoutingPolicy::Mirroring,
            RoutingPolicy::Unknown,
        ];
        for policy in policies {
            let route = Route { policy, ..full() };
            store.store_route(&route).await.unwrap();
            assert_eq!(store.get_route("main", LINK).await.unwrap(), Some(route.clone()));
            assert_eq!(store.get_route_by_route_id("r").await.unwrap(), Some(route));
        }

        // Nothing optional is needed, and an update replaces everything.
        let minimal = Route { switch: "main".into(), link: LINK.into(), ..Default::default() };
        store.update_route(&minimal).await.unwrap();
        assert_eq!(store.get_route("main", LINK).await.unwrap(), Some(minimal));
        assert_eq!(store.get_route_by_route_id("r").await.unwrap(), None);
        store.delete_route_by_switch_and_link("main", LINK).await.unwrap();
        assert_eq!(store.get_route("main", LINK).await.unwrap(), None);
    }

    #[tokio::test]
    #[ignore = "needs DynamoDB"]
    async fn items_without_newer_fields_are_read() {
        let store = store("routes-older").await;
        let mut item = route_item(&full()).unwrap();
        item.remove("status");
        item.remove("ttl");
        let Some(AttributeValue::M(properties)) = item.get_mut("properties") else { panic!() };
        properties.remove("conversion_opt_out");
        properties.remove("track_conversions");
        store.client.put_item().table_name("routes-older").set_item(Some(item)).send().await.unwrap();
        let route = store.get_route("main", LINK).await.unwrap().unwrap();
        assert!(!route.properties.conversion_opt_out && !route.properties.track_conversions);
        assert_eq!((route.status, route.ttl), (RouteStatus::Active, None));
    }

    #[tokio::test]
    #[ignore = "needs DynamoDB"]
    async fn families_replace_their_branches() {
        let store = store("routes-family").await;
        let family = |keys: &[&str]| {
            let conditions = keys
                .iter()
                .map(|key| ConditionalRouting {
                    key: key.to_string(),
                    condition: Condition::default(),
                    dest: Some(format!("https://e.com/{key}")),
                })
                .collect();
            Route { policy: RoutingPolicy::Conditional(conditions), ..full() }.build_family()
        };
        let switches = || async {
            let mut all: Vec<_> = store.get_routes_by_link(LINK).await.unwrap().into_iter().map(|r| r.switch).collect();
            all.sort();
            all
        };

        store.store_route_family(&family(&["US", "de"])).await.unwrap();
        assert_eq!(switches().await, ["US", "de", "main"]);
        let branch = store.get_route("US", LINK).await.unwrap().unwrap();
        assert_eq!(branch.dest.as_deref(), Some("https://e.com/US"));
        assert_eq!(store.get_route_by_route_id("r").await.unwrap().unwrap().switch, "main");

        store.store_route_family(&family(&["US", "fr"])).await.unwrap();
        assert_eq!(switches().await, ["US", "fr", "main"]);
        // More than one old-style batch.
        let many: Vec<String> = (0..60).map(|i| format!("k{i}")).collect();
        store.store_route_family(&family(&many.iter().map(String::as_str).collect::<Vec<_>>())).await.unwrap();
        assert_eq!(store.delete_routes_by_link(LINK).await.unwrap(), 61);
        assert!(switches().await.is_empty());
    }

    #[tokio::test]
    #[ignore = "needs DynamoDB"]
    async fn a_failed_family_changes_nothing() {
        let store = store("routes-atomic").await;
        store.store_route(&full()).await.unwrap();
        let mut bad = full();
        bad.switch = String::new(); // keys can't be empty
        let changed = Route { dest: None, ..full() };
        assert!(store.store_route_family(&[changed, bad]).await.is_err());
        assert_eq!(store.get_route("main", LINK).await.unwrap(), Some(full()));
    }

    #[tokio::test]
    #[ignore = "needs DynamoDB"]
    async fn a_missing_index_is_an_error() {
        let store = DynamoRoutesStore { routes_table: "no-such-table".into(), ..store("routes-any").await };
        assert!(store.get_route_by_route_id("r").await.is_err());
    }
}
