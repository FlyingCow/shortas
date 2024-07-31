use std::{sync::Arc, time::Duration};

use super::settings::ClickAggsConfig;
use crate::{core::click_aggs_register::BaseClickAggsRegistrar, model::ClickStreamItem};
use anyhow::Result;
use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct KafkaClickAggsRegistrar {
    settings: ClickAggsConfig,
    producer: Arc<Mutex<Producer>>,
}

impl KafkaClickAggsRegistrar {
    pub fn new(settings: ClickAggsConfig) -> Self {
        let producer = Producer::from_hosts(settings.hosts.clone())
            .with_ack_timeout(Duration::from_secs(settings.ack_timeout_secs))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Self {
            producer: Arc::new(Mutex::new(producer)),
            settings,
        }
    }
}

#[async_trait::async_trait()]
impl BaseClickAggsRegistrar for KafkaClickAggsRegistrar {
    async fn register(&self, click: ClickStreamItem) -> Result<()> {
        info!("Sending a hits batch");

        let record = Record::from_value(
            self.settings.topic.as_str(),
            serde_json::to_vec(&click).unwrap(),
        );

        self.producer.lock().await.send(&record)?;

        Ok(())
    }
}
