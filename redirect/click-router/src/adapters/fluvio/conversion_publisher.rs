use std::time::Duration;

use anyhow::Result;

use crate::core::{conversion_events::ConversionRecord, conversion_publisher::ConversionPublisher};

use super::{producer::FluvioProducer, settings::HitStreamConfig};

const ACKNOWLEDGE_TIMEOUT: Duration = Duration::from_secs(3);

pub struct FluvioConversionPublisher {
    producer: FluvioProducer,
}

impl FluvioConversionPublisher {
    pub fn new(settings: &HitStreamConfig) -> Self {
        Self {
            producer: FluvioProducer::new(settings),
        }
    }
}

#[async_trait::async_trait()]
impl ConversionPublisher for FluvioConversionPublisher {
    async fn publish(&self, record: &ConversionRecord) -> Result<()> {
        self.producer
            .send(serde_json::to_vec(record)?, true, ACKNOWLEDGE_TIMEOUT)
            .await
    }
}
