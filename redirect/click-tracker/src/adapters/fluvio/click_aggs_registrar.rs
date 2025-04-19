use std::{sync::Arc, time::Duration};

use super::settings::ClickAggsConfig;
use crate::{core::click_aggs_register::BaseClickAggsRegistrar, model::ClickStreamItem};
use anyhow::Result;
use fluvio::{
    spu::SpuSocketPool, Compression, Fluvio, FluvioClusterConfig, RecordKey, TopicProducer,
    TopicProducerConfigBuilder,
};
use tracing::info;

#[derive(Clone)]
pub struct FluvioClickAggsRegistrar {
    producer: Arc<TopicProducer<SpuSocketPool>>,
}

impl FluvioClickAggsRegistrar {
    pub async fn new(settings: ClickAggsConfig) -> Self {
        // Use config builder to create a topic producer config
        let producer_config = TopicProducerConfigBuilder::default()
            .batch_size(settings.batch_size)
            .linger(Duration::from_secs(settings.linger))
            .compression(Compression::Gzip)
            .build()
            .expect("Failed to create topic producer config");

        let config = FluvioClusterConfig::new(settings.host);

        // Connet to fluvio cluster & create a producer
        let fluvio = Fluvio::connect_with_config(&config)
            .await
            .expect("Failed to connect to Fluvio");

        let producer = fluvio
            .topic_producer_with_config(settings.topic, producer_config)
            .await
            .expect("Failed to create a producer");

        let producer = Arc::new(producer);

        Self { producer }
    }
}

#[async_trait::async_trait()]
impl BaseClickAggsRegistrar for FluvioClickAggsRegistrar {
    async fn register(&self, hit: ClickStreamItem) -> Result<()> {
        let record = serde_json::to_vec(&hit).unwrap();
        self.producer
            .send(RecordKey::NULL, record)
            .await
            .expect("Failed to send record");

        Ok(())
    }
}
