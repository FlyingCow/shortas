use anyhow::Result;
use async_trait::async_trait;

use crate::model::{CertificateOrder, OrderStatus};

#[async_trait]
pub trait OrderStore: Send + Sync {
    async fn store_order(&self, order: &CertificateOrder) -> Result<()>;
    async fn update_order(&self, order: &CertificateOrder) -> Result<()>;
    async fn get_order(&self, order_id: &str) -> Result<Option<CertificateOrder>>;
    async fn get_orders_by_status(&self, status: OrderStatus, limit: usize) -> Result<Vec<CertificateOrder>>;
    async fn get_active_order_for_domain(&self, domain: &str) -> Result<Option<CertificateOrder>>;
    async fn delete_order(&self, order_id: &str) -> Result<()>;
}
