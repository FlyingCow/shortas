use std::time::Duration;

use anyhow::{bail, Context, Result};
use chrono::Utc;
use fluvio::{
    consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
    Fluvio, FluvioClusterConfig, Offset,
};
use futures::StreamExt;
use tokio::time::{timeout, timeout_at, Instant};

use crate::adapters::fluvio::settings::ClickStreamConfig;

use super::{
    record::ConversionRecord,
    store::{Batch, ConversionStore},
    RECORDS,
};

const MAX_BATCH: usize = 1_000;
const BATCH_TIME: Duration = Duration::from_millis(500);
const COMMIT_TIME: Duration = Duration::from_secs(1);
const COMMIT_TIMEOUT: Duration = Duration::from_secs(10);

/// Stores records until something fails. Offsets are committed only after
/// their records are stored, and the last of a burst with the next record.
/// Records may be stored again after a restart, which changes nothing.
pub async fn consume(settings: &ClickStreamConfig, store: &ConversionStore) -> Result<()> {
    let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(settings.host.clone()))
        .await
        .context("connecting failed")?;
    let config = ConsumerConfigExtBuilder::default()
        .topic(settings.topic.clone())
        .offset_consumer(settings.consumer.clone())
        .offset_start(Offset::beginning())
        .offset_strategy(OffsetManagementStrategy::Manual)
        .build()?;
    let mut stream = fluvio.consumer_with_config(config).await?;

    let mut last_commit: Option<Instant> = None;
    loop {
        let Some(first) = stream.next().await else {
            bail!("the stream ended");
        };
        let mut records = vec![first.map_err(|e| anyhow::anyhow!("{e}"))?];

        // A cancelled `next` keeps the stream locked, and committing would wait
        // for it forever. So commits follow a record directly, without
        // waiting for more.
        let commit = last_commit.is_none_or(|at| at.elapsed() >= COMMIT_TIME);
        if !commit {
            let deadline = Instant::now() + BATCH_TIME;
            while records.len() < MAX_BATCH {
                match timeout_at(deadline, stream.next()).await {
                    Ok(Some(record)) => records.push(record.map_err(|e| anyhow::anyhow!("{e}"))?),
                    // An ended stream is noticed on the next read.
                    Ok(None) | Err(_) => break,
                }
            }
        }

        store_batch(store, records.iter().map(|r| r.as_ref())).await?;
        if commit {
            timeout(COMMIT_TIMEOUT, async {
                stream.offset_commit().await?;
                stream.offset_flush().await
            })
            .await
            .context("committing offsets timed out")?
            .context("committing offsets failed")?;
            last_commit = Some(Instant::now());
        }
    }
}

pub async fn store_batch<'a>(
    store: &ConversionStore,
    payloads: impl Iterator<Item = &'a [u8]>,
) -> Result<()> {
    let now = Utc::now();
    let mut batch = Batch::default();
    let mut outcomes = Vec::new();
    for payload in payloads {
        let outcome = serde_json::from_slice::<ConversionRecord>(payload)
            .map_err(anyhow::Error::from)
            .and_then(|record| batch.add(&record, now));
        // Skipped, or it would block every later record.
        outcomes.push(outcome.unwrap_or_else(|e| {
            tracing::error!(error = %e, "Skipping an unreadable conversion record");
            "unreadable"
        }));
    }
    store.insert(batch).await?;
    for outcome in outcomes {
        RECORDS.with_label_values(&[outcome]).inc();
    }
    Ok(())
}
