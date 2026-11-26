use anyhow::Result;

use super::conversion_events::ConversionRecord;

#[async_trait::async_trait()]
pub trait ConversionPublisher: Send + Sync {
    /// Returns once the stream has acknowledged the record.
    async fn publish(&self, record: &ConversionRecord) -> Result<()>;
}
