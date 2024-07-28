use std::sync::Arc;

use super::settings::HitStreamConfig;
use crate::{core::hit_stream::BaseHitStream, model::Hit};
use anyhow::{Ok, Result};
use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};
use tokio::sync::Mutex;

const TRACKERS_GROUP: &str = "trackers";

#[derive(Clone)]
pub struct KafkaHitStream {
    consumer: Arc<Mutex<Consumer>>
}

impl KafkaHitStream {
    pub fn new(settings: HitStreamConfig) -> Self {
        let consumer = Consumer::from_hosts(settings.hosts.clone())
            .with_topic(settings.topic.clone())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(TRACKERS_GROUP.to_string())
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()
            .unwrap();

        Self {
            consumer: Arc::new(Mutex::new(consumer))
        }
    }
}

#[async_trait::async_trait()]
impl BaseHitStream for KafkaHitStream {
    async fn pull(&mut self) -> Result<Vec<Hit>> {
        let mut consumer = self.consumer.lock().await;
        for ms in consumer.poll()?.iter() {
            for m in ms.messages() {
                let str = String::from_utf8_lossy(m.value);
                println!("{:?}", str);
            }
            let _ = consumer.consume_messageset(ms);
            consumer.commit_consumed().unwrap();
        }

        Ok(vec![])
    }
}
