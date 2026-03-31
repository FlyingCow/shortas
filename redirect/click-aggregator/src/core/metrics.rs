//! Metrics collection for Click Aggregator
//!
//! Provides Prometheus metrics for monitoring aggregator performance,
//! with debug-specific metrics for routes with allow_debug=true.

use lazy_static::lazy_static;
use prometheus::{
    Histogram, HistogramOpts, IntCounter, Opts,
    register_histogram, register_int_counter,
};

/// Metrics for tracking aggregator performance
#[derive(Clone)]
pub struct AggregatorMetrics {
    /// Total clicks processed
    pub clicks_processed_total: IntCounter,

    /// Total clicks with debug trace data
    pub debug_clicks_total: IntCounter,

    /// Full pipeline processing time for debug clicks
    pub debug_pipeline_duration: Histogram,

    /// ClickHouse write time for debug clicks
    pub debug_store_duration: Histogram,

    /// Queue latency (time from router exit to aggregator processing) for debug clicks
    pub debug_queue_latency: Histogram,
}

impl Default for AggregatorMetrics {
    fn default() -> Self {
        Self {
            clicks_processed_total: IntCounter::new(
                "fallback_clicks_processed_total",
                "Fallback counter",
            )
            .unwrap(),
            debug_clicks_total: IntCounter::new(
                "fallback_debug_clicks_total",
                "Fallback counter",
            )
            .unwrap(),
            debug_pipeline_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_debug_pipeline_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            debug_store_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_debug_store_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            debug_queue_latency: Histogram::with_opts(HistogramOpts::new(
                "fallback_debug_queue_latency",
                "Fallback histogram",
            ))
            .unwrap(),
        }
    }
}

impl AggregatorMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            clicks_processed_total: register_int_counter!(
                "aggregator_clicks_processed_total",
                "Total number of clicks processed by the aggregator"
            )?,

            debug_clicks_total: register_int_counter!(
                "aggregator_debug_clicks_total",
                "Total number of debug clicks processed (with trace data)"
            )?,

            debug_pipeline_duration: register_histogram!(
                HistogramOpts::new(
                    "aggregator_debug_pipeline_duration_seconds",
                    "Full pipeline processing time for debug clicks"
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
            )?,

            debug_store_duration: register_histogram!(
                HistogramOpts::new(
                    "aggregator_debug_store_duration_seconds",
                    "ClickHouse write time for debug clicks"
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
            )?,

            debug_queue_latency: register_histogram!(
                HistogramOpts::new(
                    "aggregator_debug_queue_latency_seconds",
                    "Queue latency from router exit to aggregator processing"
                )
                .buckets(vec![0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0])
            )?,
        })
    }
}

/// Global metrics instance
lazy_static! {
    pub static ref METRICS: AggregatorMetrics = {
        AggregatorMetrics::new().unwrap_or_else(|e| {
            tracing::warn!("Metrics already registered, using default implementation: {}", e);
            AggregatorMetrics::default()
        })
    };
}

/// Timer helper for measuring durations
pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    pub fn observe_duration_seconds(&self, histogram: &Histogram) {
        let duration = self.start.elapsed();
        histogram.observe(duration.as_secs_f64());
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}
