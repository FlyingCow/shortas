pub mod aws;
pub mod click_router_api;
pub mod mongodb;
pub mod rabbitmq;

pub use aws::DynamoOrderStore;
pub use mongodb::MongodbOrderStore;

use crate::core::OrderStore;
use crate::model::{CertificateOrder, OrderStatus};
use anyhow::Result;

/// Enum-dispatch wrapper over the two `OrderStore` backends (DynamoDB for AWS
/// run modes, MongoDB otherwise). Preferred over `Box<dyn OrderStore>` here to
/// match the rest of the Rust workspace's convention for storage adapters
/// (e.g. click-router's `RoutesStoreType`): no vtable, Clone without Arc.
#[derive(Clone)]
pub enum OrderStoreType {
    Mongodb(MongodbOrderStore),
    Dynamo(DynamoOrderStore),
}

#[async_trait::async_trait]
impl OrderStore for OrderStoreType {
    async fn store_order(&self, order: &CertificateOrder) -> Result<()> {
        match self {
            OrderStoreType::Mongodb(store) => store.store_order(order).await,
            OrderStoreType::Dynamo(store) => store.store_order(order).await,
        }
    }

    async fn update_order(&self, order: &CertificateOrder) -> Result<()> {
        match self {
            OrderStoreType::Mongodb(store) => store.update_order(order).await,
            OrderStoreType::Dynamo(store) => store.update_order(order).await,
        }
    }

    async fn get_order(&self, order_id: &str) -> Result<Option<CertificateOrder>> {
        match self {
            OrderStoreType::Mongodb(store) => store.get_order(order_id).await,
            OrderStoreType::Dynamo(store) => store.get_order(order_id).await,
        }
    }

    async fn get_orders_by_status(&self, status: OrderStatus, limit: usize) -> Result<Vec<CertificateOrder>> {
        match self {
            OrderStoreType::Mongodb(store) => store.get_orders_by_status(status, limit).await,
            OrderStoreType::Dynamo(store) => store.get_orders_by_status(status, limit).await,
        }
    }

    async fn get_active_order_for_domain(&self, domain: &str) -> Result<Option<CertificateOrder>> {
        match self {
            OrderStoreType::Mongodb(store) => store.get_active_order_for_domain(domain).await,
            OrderStoreType::Dynamo(store) => store.get_active_order_for_domain(domain).await,
        }
    }

    async fn delete_order(&self, order_id: &str) -> Result<()> {
        match self {
            OrderStoreType::Mongodb(store) => store.delete_order(order_id).await,
            OrderStoreType::Dynamo(store) => store.delete_order(order_id).await,
        }
    }
}
