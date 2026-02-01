use anyhow::Result;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::core::RoutesStore;
use crate::model::route::{
    DestinationFormat, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal,
};
use crate::model::Route;

#[derive(Clone, Debug)]
pub struct MongodbRoutesStore {
    collection: Collection<RouteDocument>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct RouteDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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

impl MongodbRoutesStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<RouteDocument>(collection_name),
        }
    }
}

#[async_trait::async_trait()]
impl RoutesStore for MongodbRoutesStore {
    async fn store_route(&self, route: &Route) -> Result<()> {
        let doc = RouteDocument {
            switch: route.switch.clone(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            dest_format: route.dest_format.clone(),
            code: route.code,
            ttl: route.ttl,
            status: route.status.clone(),
            terminal: route.terminal.clone(),
            policy: route.policy.clone(),
            properties: route.properties.clone(),
            ..Default::default()
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn update_route(&self, route: &Route) -> Result<()> {
        let filter = doc! { "switch": &route.switch, "link": &route.link};

        let doc = RouteDocument {
            switch: route.switch.clone(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            dest_format: route.dest_format.clone(),
            code: route.code,
            ttl: route.ttl,
            status: route.status.clone(),
            terminal: route.terminal.clone(),
            policy: route.policy.clone(),
            properties: route.properties.clone(),
            ..Default::default()
        };

        self.collection.replace_one(filter, doc).await?;
        Ok(())
    }

    async fn delete_route(&self, route: &Route) -> Result<()> {
        let filter = doc! { "switch": &route.switch, "link": &route.link};
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn get_route(&self, switch: &str, link: &str) -> Result<Option<Route>> {
        let filter = doc! { "switch": switch, "link": link};

        match self.collection.find_one(filter).await? {
            Some(doc) => {
                let route = Route {
                    switch: doc.switch.clone(),
                    link: doc.link.clone(),
                    dest: doc.dest.clone(),
                    dest_format: doc.dest_format.clone(),
                    code: doc.code,
                    ttl: doc.ttl,
                    status: doc.status.clone(),
                    terminal: doc.terminal.clone(),
                    policy: doc.policy.clone(),
                    properties: doc.properties.clone(),
                    ..Default::default()
                };
                Ok(Some(route))
            }
            None => Ok(None),
        }
    }

    async fn get_route_by_route_id(&self, route_id: &str) -> Result<Option<Route>> {
        let filter = doc! { "properties.route_id": route_id };

        match self.collection.find_one(filter).await? {
            Some(doc) => {
                let route = Route {
                    switch: doc.switch.clone(),
                    link: doc.link.clone(),
                    dest: doc.dest.clone(),
                    dest_format: doc.dest_format.clone(),
                    code: doc.code,
                    ttl: doc.ttl,
                    status: doc.status.clone(),
                    terminal: doc.terminal.clone(),
                    policy: doc.policy.clone(),
                    properties: doc.properties.clone(),
                    ..Default::default()
                };
                Ok(Some(route))
            }
            None => Ok(None),
        }
    }

    async fn invalidate_route(&self, _switch: &str, _link: &str) -> Result<()> {
        // MongoDB doesn't need explicit invalidation like DynamoDB
        Ok(())
    }
}
