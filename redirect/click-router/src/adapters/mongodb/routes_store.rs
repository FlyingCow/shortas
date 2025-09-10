use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::RoutesStore;

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
impl RoutesStore for MongodbRoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        let filter = doc! { "switch": switch, "link": path};

        let result = Ok(match self.collection.find_one(filter).await? {
            Some(mongo_route) => Some(mongo_route),
            None => None,
        });

        result
    }
}
