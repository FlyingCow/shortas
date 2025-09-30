use async_trait::async_trait;
use anyhow::Result;
use crate::model::clickstream::{ClickStreamQuery, ClickStreamResponse};

/// Trait for click stream data operations
#[async_trait]
pub trait ClickStreamStore: Send + Sync {
    /// Query click stream data with filters
    async fn query_clickstream(&self, query: &ClickStreamQuery) -> Result<ClickStreamResponse>;
    
    /// Get total count of click stream items matching the query
    async fn count_clickstream(&self, query: &ClickStreamQuery) -> Result<u64>;
}
