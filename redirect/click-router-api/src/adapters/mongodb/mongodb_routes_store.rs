use std::collections::HashMap;

use anyhow::Result;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::BaseRoutesStore;
use crate::model::Route;

#[derive(Clone, Debug)]
pub struct MongodbRoutesStore {
    collection: Collection<Route>,
}

impl MongodbRoutesStore {
    pub async fn new(settings: &Mongodb) -> Self {
        let client = Client::with_uri_str(&settings.uri)
            .await
            .expect("can not connect to mongodb");

        let db = client.database(&settings.database);
        let collection = db.collection::<Route>(&settings.routes_collection);
        Self { collection }
    }
}

#[async_trait::async_trait()]
impl BaseRoutesStore for MongodbRoutesStore {
    async fn store_route(&self, route: &Route) -> Result<()> {
        self.collection.insert_one(route).await?;
        Ok(())
    }

    async fn update_route(&self, _: &Route) -> Result<()> {
        todo!()
    }
    async fn delete_route(&self, _: &Route) -> Result<()> {
        todo!()
    }

    async fn invalidate_route(&self, _switch: &str, _domain: &str, _path: &str) -> Result<()> {
        todo!()
    }

    async fn get_route(&self, switch: &str, domain: &str, path: &str) -> Result<Option<Route>> {
        let path = format!("{}%2f{}", domain, path);
        let target_id_str = format!("{}|{}", switch, path);
        let target_object_id = ObjectId::parse_str(target_id_str)?;

        let filter = doc! { "_id": target_object_id };
        Ok(self.collection.find_one(filter).await?)
    }
}
