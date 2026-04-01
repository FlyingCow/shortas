use anyhow::Result;
use chrono::Utc;
use tracing::info;

use crate::{
    adapters::{ClickStreamStore, ClickStreamStoreType},
    core::{aggs_pipe::AggsModule, metrics::{Timer, METRICS}, AggsPipeContext},
};

#[derive(Clone)]
pub struct StoreModule {
    click_stream_store: ClickStreamStoreType,
}

#[async_trait::async_trait()]
impl AggsModule for StoreModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        info!("{}", serde_json::json!(context.click));

        // Track metrics
        METRICS.clicks_processed_total.inc();

        let has_trace = context.click.trace.is_some();
        if has_trace {
            METRICS.debug_clicks_total.inc();

            // Calculate queue latency if we have router exit time
            if let Some(ref trace) = context.click.trace {
                if let Some(router_exit_utc) = trace.router_exit_utc {
                    let queue_latency = Utc::now()
                        .signed_duration_since(router_exit_utc)
                        .num_milliseconds() as f64
                        / 1000.0;
                    METRICS.debug_queue_latency.observe(queue_latency.max(0.0));
                }
            }
        }

        let store_timer = Timer::new();

        let result = self.click_stream_store
            .register(context.click.clone())
            .await;

        // Record store duration for debug clicks
        if has_trace {
            store_timer.observe_duration_seconds(&METRICS.debug_store_duration);
        }

        result
    }
}

impl StoreModule {
    pub fn new(click_stream_store: ClickStreamStoreType) -> Self {
        Self { click_stream_store }
    }
}
