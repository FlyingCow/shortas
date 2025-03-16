use std::time::Duration;

use anyhow::{Ok, Result};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

use crate::{
    core::hits_register::BaseHitRegistrar,
    model::Hit,
};

use crate::adapters::kafka::settings::HitStreamConfig;


#[derive(Clone)]
pub struct KafkaHitRegistrar {
    producer: FutureProducer,
    settings: HitStreamConfig
}

impl KafkaHitRegistrar {
    pub fn new(settings: HitStreamConfig) -> Self {

        let producer = ClientConfig::new()
            .set("bootstrap.servers", settings.hosts.join(","))
            .set("message.timeout.ms", settings.ack_timeout_secs.to_string())
            .create()
            .expect("Hit registrar producer creation error");

        Self { producer, settings }
    }
}

#[async_trait::async_trait()]
impl BaseHitRegistrar for KafkaHitRegistrar {
    async fn register(&self, hit: Hit) -> Result<()> {
        let result = self.producer
            .send(
                FutureRecord::to(&self.settings.topic)
                    .payload(&serde_json::to_vec(&hit).unwrap())
                    .key(&hit.id),
                    // .headers(OwnedHeaders::new().insert(Header {
                    //     key: "header_key",
                    //     value: Some("header_value"),
                    // })),
                Duration::from_secs(0),
            )
            .await;

        if let Err(err) = result {
            return Err(err.0.into());
        }

        Ok(())
    }
}
