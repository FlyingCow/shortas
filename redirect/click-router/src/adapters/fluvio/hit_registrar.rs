use std::time::Duration;

use anyhow::Result;
use fluvio::{
    spu::SpuSocketPool, Fluvio, FluvioClusterConfig, TopicProducer, TopicProducerConfigBuilder,
};
use tracing::info;

use crate::{core::hits_register::BaseHitRegistrar, model::Hit, utils::async_queue::AsyncQueue};

use super::settings::HitStreamConfig;

#[derive(Clone)]
pub struct FluvioHitRegistrar {
    producer: TopicProducer<SpuSocketPool>,
}

impl FluvioHitRegistrar {
    pub async fn new(settings: HitStreamConfig) -> Self {
        let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(settings.host.clone()))
            .await
            .expect("Can not connect to fluvio cluster.");

        let producer = fluvio
            .topic_producer_with_config(
                settings.topic.clone(),
                TopicProducerConfigBuilder::default()
                    .linger(Duration::from_millis(settings.linger_millis))
                    .batch_size(settings.batch_size_bytes)
                    .build()
                    .expect("Can not build click aggs registrar topic config."),
            )
            .await
            .expect("Can not build click aggs registrar topic producer.");

        Self { producer }
    }
}

#[async_trait::async_trait()]
impl BaseHitRegistrar for FluvioHitRegistrar {
    async fn register(&self, hit: Hit) -> Result<()> {
        self.producer
            .send(
                hit.id.as_str(),
                serde_json::to_string(&hit).expect("Can not serialize an instance of hit."),
            )
            .await
            .map_err(anyhow::Error::msg)?;

        Ok(())
    }
}
