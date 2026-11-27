//! Conversion events: stored from their own stream, then matched to clicks.

pub mod attribution;
pub mod consumer;
pub mod reconciler;
pub mod record;
pub mod settings;
pub mod store;
#[cfg(test)]
mod store_tests;

use std::time::Duration;

use lazy_static::lazy_static;
use prometheus::{register_int_counter_vec, register_int_gauge, IntCounterVec, IntGauge};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::settings::Settings;
use store::ConversionStore;

lazy_static! {
    pub(crate) static ref RECORDS: IntCounterVec = register_int_counter_vec!(
        "conversion_records_total",
        "Conversion records by what became of them",
        &["outcome"]
    )
    .unwrap();
    pub(crate) static ref WAITING: IntGauge = register_int_gauge!(
        "conversion_events_waiting",
        "Conversion events waiting for their click"
    )
    .unwrap();
    pub(crate) static ref LATE: IntGauge = register_int_gauge!(
        "conversion_events_late",
        "Conversion events counted as unattributed whose click is still looked for"
    )
    .unwrap();
    pub(crate) static ref ERRORS: IntCounterVec = register_int_counter_vec!(
        "conversion_errors_total",
        "Failed conversion operations, which are retried",
        &["operation"]
    )
    .unwrap();
}

const RETRY_DELAY: Duration = Duration::from_secs(5);

/// Starts the consumer, the reconciler and the late re-checks.
pub fn start(settings: &Settings, token: CancellationToken) -> JoinSet<()> {
    let mut tasks = JoinSet::new();
    let conversions = &settings.conversions;
    if !conversions.enabled {
        return tasks;
    }
    let store = ConversionStore::new(&settings.clickhouse.click_stream_store);

    let (stream, consumer_store, consumer_token) =
        (conversions.stream.clone(), store.clone(), token.clone());
    tasks.spawn(async move {
        repeat("consume", RETRY_DELAY, &consumer_token, || {
            consumer::consume(&stream, &consumer_store)
        })
        .await;
    });

    let (interval, reconcile_store, reconcile_token) = (
        Duration::from_millis(conversions.reconcile_millis),
        store.clone(),
        token.clone(),
    );
    tasks.spawn(async move {
        repeat("reconcile", interval, &reconcile_token, || async {
            reconciler::reconcile(&reconcile_store, chrono::Utc::now()).await.map(drop)
        })
        .await;
    });

    let interval = Duration::from_millis(conversions.late_recheck_millis);
    tasks.spawn(async move {
        repeat("recheck_late", interval, &token, || async {
            reconciler::recheck_late(&store, chrono::Utc::now()).await.map(drop)
        })
        .await;
    });
    tasks
}

/// Runs `operation` until cancelled, with a pause after each run.
async fn repeat<F, T>(name: &str, pause: Duration, token: &CancellationToken, operation: F)
where
    F: Fn() -> T,
    T: std::future::Future<Output = anyhow::Result<()>>,
{
    loop {
        tokio::select! {
            _ = token.cancelled() => return,
            result = operation() => {
                if let Err(e) = result {
                    ERRORS.with_label_values(&[name]).inc();
                    tracing::error!(error = %format!("{e:#}"), "Conversions: {name} failed");
                }
            }
        }
        tokio::select! {
            _ = token.cancelled() => return,
            _ = tokio::time::sleep(pause) => {}
        }
    }
}
