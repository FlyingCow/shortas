use anyhow::Result;
use async_trait::async_trait;
use mongodb::{
    bson::doc,
    options::ClientOptions,
    Client, Collection,
};
use std::time::Duration;

use crate::core::OrderStore;
use crate::model::{CertificateOrder, OrderStatus};
use crate::settings::MongodbSettings;

#[derive(Clone)]
pub struct MongodbOrderStore {
    collection: Collection<CertificateOrder>,
}

impl MongodbOrderStore {
    pub async fn new(settings: &MongodbSettings) -> Result<Self> {
        let mut client_options = ClientOptions::parse(&settings.uri).await?;

        client_options.min_pool_size = Some(5);
        client_options.max_pool_size = Some(20);
        client_options.max_idle_time = Some(Duration::from_secs(300));
        client_options.connect_timeout = Some(Duration::from_secs(5));

        let client = Client::with_options(client_options)?;
        let db = client.database(&settings.database);
        let collection = db.collection::<CertificateOrder>(&settings.orders_collection);

        Ok(Self { collection })
    }
}

#[async_trait]
impl OrderStore for MongodbOrderStore {
    async fn store_order(&self, order: &CertificateOrder) -> Result<()> {
        self.collection.insert_one(order).await?;
        Ok(())
    }

    async fn update_order(&self, order: &CertificateOrder) -> Result<()> {
        let filter = doc! { "order_id": &order.order_id };
        self.collection.replace_one(filter, order).await?;
        Ok(())
    }

    async fn get_order(&self, order_id: &str) -> Result<Option<CertificateOrder>> {
        let filter = doc! { "order_id": order_id };
        Ok(self.collection.find_one(filter).await?)
    }

    async fn get_orders_by_status(&self, status: OrderStatus, limit: usize) -> Result<Vec<CertificateOrder>> {
        use futures::TryStreamExt;

        let status_str = serde_json::to_string(&status)?;
        let filter = doc! { "status": status_str.trim_matches('"') };

        let cursor = self.collection.find(filter).limit(limit as i64).await?;
        let orders: Vec<CertificateOrder> = cursor.try_collect().await?;

        Ok(orders)
    }

    async fn get_active_order_for_domain(&self, domain: &str) -> Result<Option<CertificateOrder>> {
        let filter = doc! {
            "domain": domain.to_lowercase(),
            "status": { "$nin": ["valid", "failed", "expired"] }
        };
        Ok(self.collection.find_one(filter).await?)
    }

    async fn delete_order(&self, order_id: &str) -> Result<()> {
        let filter = doc! { "order_id": order_id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
