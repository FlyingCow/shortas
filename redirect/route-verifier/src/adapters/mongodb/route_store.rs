use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, to_document, Document},
    options::{FindOptions, CountOptions, UpdateOptions},
    Collection, Database,
};

use crate::core::RouteStore;
use crate::model::RouteToVerify;

#[derive(Clone, Debug)]
pub struct MongodbRouteStore {
    collection: Collection<Document>,
}

impl MongodbRouteStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<Document>(collection_name),
        }
    }
}

#[async_trait::async_trait]
impl RouteStore for MongodbRouteStore {
    async fn store_route(&self, route: &RouteToVerify) -> Result<()> {
        let filter = doc! { "_id": &route.id };
        let doc = to_document(route)?;
        let update = doc! { "$set": doc };
        let options = UpdateOptions::builder().upsert(true).build();

        self.collection
            .update_one(filter, update)
            .with_options(options)
            .await?;
        Ok(())
    }

    async fn update_route(&self, route: &RouteToVerify) -> Result<()> {
        let filter = doc! { "_id": &route.id };
        let update = doc! {
            "$set": {
                "link": &route.link,
                "destinations": &route.destinations,
                "owner_id": &route.owner_id,
                "workspace_id": &route.workspace_id,
            }
        };

        self.collection.update_one(filter, update).await?;
        Ok(())
    }

    async fn delete_route(&self, id: &str) -> Result<()> {
        let filter = doc! { "_id": id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn get_route(&self, id: &str) -> Result<Option<RouteToVerify>> {
        let filter = doc! { "_id": id };

        let doc = self.collection.find_one(filter).await?;
        match doc {
            Some(d) => {
                let route = mongodb::bson::from_document::<RouteToVerify>(d)?;
                Ok(Some(route))
            }
            None => Ok(None),
        }
    }

    async fn list_routes(
        &self,
        owner_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<RouteToVerify>, u64)> {
        let mut filter = doc! {};

        if let Some(oid) = owner_id {
            filter.insert("owner_id", oid);
        }

        let skip = ((page - 1) * page_size) as u64;
        let options = FindOptions::builder()
            .skip(skip)
            .limit(page_size as i64)
            .sort(doc! { "link": 1 })
            .build();

        let count_options = CountOptions::builder().build();
        let total_count = self
            .collection
            .count_documents(filter.clone())
            .with_options(count_options)
            .await?;

        let mut cursor = self.collection.find(filter).with_options(options).await?;
        let mut routes = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            if let Ok(route) = mongodb::bson::from_document::<RouteToVerify>(doc) {
                routes.push(route);
            }
        }

        Ok((routes, total_count))
    }

    async fn get_routes_for_verification(
        &self,
        before: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<RouteToVerify>> {
        let before_millis = before.timestamp_millis();

        // Query routes where:
        // - has at least one destination to check
        // - next_safety_check <= now OR next_safety_check doesn't exist
        let filter = doc! {
            "destinations.0": { "$exists": true },
            "$or": [
                { "next_safety_check": { "$lte": before_millis } },
                { "next_safety_check": { "$exists": false } },
                { "next_safety_check": null }
            ]
        };

        let options = FindOptions::builder()
            .limit(limit as i64)
            .sort(doc! { "next_safety_check": 1 })
            .build();

        let mut cursor = self.collection.find(filter).with_options(options).await?;
        let mut routes = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            if let Ok(route) = mongodb::bson::from_document::<RouteToVerify>(doc) {
                routes.push(route);
            }
        }

        Ok(routes)
    }

    async fn update_safety_check_timestamps(
        &self,
        route_id: &str,
        last_check: DateTime<Utc>,
        next_check: DateTime<Utc>,
    ) -> Result<()> {
        let filter = doc! { "_id": route_id };

        let update = doc! {
            "$set": {
                "last_safety_check": last_check.timestamp_millis(),
                "next_safety_check": next_check.timestamp_millis()
            }
        };

        self.collection.update_one(filter, update).await?;
        Ok(())
    }

    async fn update_route_status(
        &self,
        route_id: &str,
        status: &str,
        blocked_reason: Option<&str>,
    ) -> Result<()> {
        let filter = doc! { "_id": route_id };

        let update = doc! {
            "$set": {
                "status": status,
                "blocked_reason": blocked_reason
            }
        };

        self.collection.update_one(filter, update).await?;
        Ok(())
    }
}
