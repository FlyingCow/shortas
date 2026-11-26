use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{bail, Result};
use fluvio::{
    Compression, Fluvio, FluvioClusterConfig, RecordKey, TopicProducerConfigBuilder,
    TopicProducerPool,
};
use tokio::{sync::Mutex, time::timeout};

use super::settings::HitStreamConfig;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
/// How long to fail right away after a failed connection attempt.
const RETRY_AFTER: Duration = Duration::from_secs(1);

#[derive(Default)]
struct State {
    /// Numbered, so a late error can't discard its replacement.
    producer: Option<(u64, TopicProducerPool)>,
    connections: u64,
    failed: Option<Instant>,
}

/// A topic producer that connects when needed and again after errors.
#[derive(Clone)]
pub struct FluvioProducer {
    settings: HitStreamConfig,
    state: Arc<Mutex<State>>,
}

impl FluvioProducer {
    pub fn new(settings: &HitStreamConfig) -> Self {
        Self {
            settings: settings.clone(),
            state: Arc::default(),
        }
    }

    /// Queues the record, and waits for the acknowledgement if asked to.
    pub async fn send(&self, record: Vec<u8>, acknowledged: bool, limit: Duration) -> Result<()> {
        let (connection, producer) = self.producer().await?;
        let result = timeout(limit, async {
            let output = producer.send(RecordKey::NULL, record).await?;
            if acknowledged {
                output.wait().await?;
            }
            anyhow::Ok(())
        })
        .await;

        let error = match result {
            Ok(Ok(())) => return Ok(()),
            Ok(Err(e)) => e,
            Err(_) => anyhow::anyhow!("timed out"),
        };
        let mut state = self.state.lock().await;
        if state.producer.as_ref().is_some_and(|(c, _)| *c == connection) {
            state.producer = None;
        }
        bail!("publishing to {} failed: {error:#}", self.settings.topic);
    }

    async fn producer(&self) -> Result<(u64, TopicProducerPool)> {
        let mut state = self.state.lock().await;
        if let Some(producer) = &state.producer {
            return Ok(producer.clone());
        }
        if state.failed.is_some_and(|at| at.elapsed() < RETRY_AFTER) {
            bail!("the event stream is unavailable");
        }
        match timeout(CONNECT_TIMEOUT, connect(&self.settings)).await {
            Ok(Ok(producer)) => {
                state.connections += 1;
                state.failed = None;
                let producer = (state.connections, producer);
                state.producer = Some(producer.clone());
                Ok(producer)
            }
            result => {
                state.failed = Some(Instant::now());
                match result {
                    Ok(Err(e)) => Err(e.context("connecting to the event stream failed")),
                    _ => bail!("connecting to the event stream timed out"),
                }
            }
        }
    }
}

async fn connect(settings: &HitStreamConfig) -> Result<TopicProducerPool> {
    let config = TopicProducerConfigBuilder::default()
        .batch_size(settings.batch_size)
        .linger(Duration::from_millis(settings.linger))
        .compression(Compression::Gzip)
        .build()?;
    let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(&settings.host)).await?;
    fluvio.topic_producer_with_config(&settings.topic, config).await
}
