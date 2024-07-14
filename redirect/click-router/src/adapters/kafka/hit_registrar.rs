use std::time::Duration;

use anyhow::Result;
use kafka::{
    client::RequiredAcks,
    producer::{AsBytes, Producer, Record},
};
use tracing::warn;

use crate::{
    core::hits_register::BaseHitRegistrar,
    model::Hit,
    utils::async_queue::{AsyncQueue, BatchProcess},
};

use super::settings::HitStreamConfig;

// impl Into<Vec<u8>> for &Hit {
//     fn into(self) -> Vec<u8> {
//         serde_json::to_vec(self).unwrap()
//     }
// }

struct KafkaHitsProducer {
    producer: Producer,
    settings: HitStreamConfig,
}

#[async_trait::async_trait()]
impl BatchProcess<Hit> for KafkaHitsProducer {
    async fn process(&mut self, batch: Vec<Hit>) -> Result<()> {
        warn!("batch");

        let mut records: Vec<Record<(), Vec<u8>>> = vec![];

        for hit in batch{
            let record = Record::from_value(
                    self.settings.topic.as_str(),
                    serde_json::to_vec(&hit).unwrap(),
                );

            records.push(record)
        }
        
        self.producer.send_all(records.as_slice())?;

        Ok(())
    }
}

impl KafkaHitsProducer {
    pub fn new(settings: HitStreamConfig) -> Self {
        let producer = Producer::from_hosts(settings.hosts.clone())
            .with_ack_timeout(Duration::from_secs(settings.ack_timeout_secs))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Self { producer, settings }
    }
}

#[derive(Clone, Debug)]
pub struct KafkaHitRegistrar {
    queue: AsyncQueue<Hit>,
}

impl KafkaHitRegistrar {
    pub fn new(settings: HitStreamConfig) -> Self {
        let iteration_duration = settings.iteration_seconds;
        let consumers_count = settings.consumers_count;
        let batch_size = settings.batch_size;

        let hit_processor = KafkaHitsProducer::new(settings);
        let hits_queue = AsyncQueue::new(
            Box::new(hit_processor),
            batch_size,
            consumers_count,
            Duration::from_secs(iteration_duration),
        );

        Self { queue: hits_queue }
    }
}

#[async_trait::async_trait()]
impl BaseHitRegistrar for KafkaHitRegistrar {
    async fn register(&self, hit: Hit) -> Result<()> {
        self.queue.enqueue(hit).await
    }
}
