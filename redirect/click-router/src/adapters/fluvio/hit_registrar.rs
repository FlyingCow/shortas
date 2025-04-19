use std::{sync::Arc, time::Duration};

use anyhow::{Ok, Result};
use fluvio::{spu::SpuSocketPool, RecordKey, TopicProducer};
use fluvio::{Compression, Fluvio, FluvioClusterConfig, TopicProducerConfigBuilder};

use crate::{core::hits_register::BaseHitRegistrar, model::Hit};

use crate::adapters::fluvio::settings::HitStreamConfig;

#[derive(Clone)]
pub struct FluvioHitRegistrar {
    producer: Arc<TopicProducer<SpuSocketPool>>,
}

impl FluvioHitRegistrar {
    pub async fn new(settings: HitStreamConfig) -> Self {
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
impl BaseHitRegistrar for FluvioHitRegistrar {
    async fn register(&self, hit: Hit) -> Result<()> {
        let record = serde_json::to_vec(&hit).unwrap();
        self.producer
            .send(RecordKey::NULL, record)
            .await
            .expect("Failed to send record");

        Ok(())
    }
}
