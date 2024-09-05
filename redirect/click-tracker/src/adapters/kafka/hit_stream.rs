use std::{sync::Arc, time::Duration};

use super::settings::HitStreamConfig;
use crate::{core::hit_stream::BaseHitStream, model::Hit};
use anyhow::{Ok, Result};
use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};
use tokio::{
    sync::Mutex,
    time::sleep,
};

const TRACKERS_GROUP: &str = "trackers";
const IDLE_TIMEOUT: u64 = 500;
const MAX_BYTES_FETCH: i32 = 55728640;

#[derive(Clone)]
pub struct KafkaHitStream {
    consumer: Arc<Mutex<Consumer>>,
}

impl KafkaHitStream {
    pub fn new(settings: HitStreamConfig) -> Self {
        let consumer = Consumer::from_hosts(settings.hosts.clone())
            .with_topic(settings.topic.clone())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(TRACKERS_GROUP.to_string())
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .with_fetch_max_bytes_per_partition(MAX_BYTES_FETCH)
            .create()
            .unwrap();

        Self {
            consumer: Arc::new(Mutex::new(consumer)),
        }
    }
}

#[async_trait::async_trait()]
impl BaseHitStream for KafkaHitStream {
    async fn pull(&mut self) -> Result<Vec<Hit>> {
        let mut consumer = self.consumer.lock().await;

        let mut hits: Vec<Hit> = vec![];
        for ms in consumer.poll()?.iter() {
            for m in ms.messages() {
                let hit = serde_json::from_slice(m.value)?;
                hits.push(hit);
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();

        if hits.len() == 0 {
            sleep(Duration::from_millis(IDLE_TIMEOUT)).await;
        }

        Ok(hits)
    }
}
