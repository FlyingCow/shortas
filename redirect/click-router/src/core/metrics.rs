//! Metrics collection and monitoring for the Click Router
//!
//! This module provides comprehensive metrics collection using Prometheus,
//! tracking performance, request processing, cache efficiency, and system health.
//!
//! ## Metrics Categories
//! - **Request Metrics**: Total requests, success/error rates, processing duration
//! - **Cache Metrics**: Hit/miss rates for routes and user settings
//! - **Database Metrics**: Query counts and response times
//! - **Flow Metrics**: Usage of iterative vs recursive processing
//! - **System Metrics**: Active requests, processing times
//!
//! ## Usage
//! Metrics are automatically collected throughout the request lifecycle and
//! exposed via HTTP endpoints for Prometheus scraping.

use lazy_static::lazy_static;
use prometheus::{
    Counter, Histogram, IntCounter, IntGauge, Registry, Opts, HistogramOpts,
    register_counter, register_histogram, register_int_counter, register_int_gauge,
};
use std::time::Instant;

/// Metrics for tracking flow router performance and optimization impact
#[derive(Clone)]
pub struct FlowRouterMetrics {
    /// Total number of requests processed
    pub requests_total: IntCounter,

    /// Number of requests processed successfully
    pub requests_success: IntCounter,

    /// Number of requests that resulted in errors
    pub requests_error: IntCounter,

    /// Number of cache hits for route lookups
    pub route_cache_hits: IntCounter,

    /// Number of cache misses for route lookups
    pub route_cache_misses: IntCounter,

    /// Number of database queries for routes
    pub route_db_queries: IntCounter,

    /// Number of user settings cache hits
    pub user_settings_cache_hits: IntCounter,

    /// Number of user settings cache misses
    pub user_settings_cache_misses: IntCounter,

    /// Number of QR code cache hits
    pub qr_cache_hits: IntCounter,

    /// Number of QR code cache misses
    pub qr_cache_misses: IntCounter,

    /// Number of hits registered
    pub hits_registered: IntCounter,

    /// Current number of active requests being processed
    pub active_requests: IntGauge,

    /// Histogram of request processing times
    pub request_duration: Histogram,

    /// Histogram of flow processing times (the optimized part)
    pub flow_processing_duration: Histogram,

    /// Histogram of database query times
    pub db_query_duration: Histogram,

    /// Histogram of cache lookup times
    pub cache_lookup_duration: Histogram,

    /// Histogram of QR code cache lookup times
    pub qr_cache_lookup_duration: Histogram,

    /// Histogram of QR code generation times
    pub qr_generation_duration: Histogram,

    /// Number of times the optimized iterative flow was used
    pub iterative_flow_usage: IntCounter,

    /// Number of times the legacy recursive flow was used (should be 0 after optimization)
    pub recursive_flow_usage: IntCounter,

    /// Memory allocations per request (estimated)
    pub memory_allocations_per_request: Histogram,
}

impl Default for FlowRouterMetrics {
    /// Create a default metrics instance that doesn't register with Prometheus
    /// This is used as a fallback when metrics are already registered
    fn default() -> Self {
        use prometheus::{Histogram, HistogramOpts, IntCounter, IntGauge};

        FlowRouterMetrics {
            requests_total: IntCounter::new("fallback_requests_total", "Fallback counter")
                .unwrap(),
            requests_success: IntCounter::new("fallback_requests_success", "Fallback counter")
                .unwrap(),
            requests_error: IntCounter::new("fallback_requests_error", "Fallback counter")
                .unwrap(),
            route_cache_hits: IntCounter::new("fallback_route_cache_hits", "Fallback counter")
                .unwrap(),
            route_cache_misses: IntCounter::new("fallback_route_cache_misses", "Fallback counter")
                .unwrap(),
            route_db_queries: IntCounter::new("fallback_route_db_queries", "Fallback counter")
                .unwrap(),
            user_settings_cache_hits: IntCounter::new(
                "fallback_user_settings_cache_hits",
                "Fallback counter",
            )
            .unwrap(),
            user_settings_cache_misses: IntCounter::new(
                "fallback_user_settings_cache_misses",
                "Fallback counter",
            )
            .unwrap(),
            qr_cache_hits: IntCounter::new("fallback_qr_cache_hits", "Fallback counter").unwrap(),
            qr_cache_misses: IntCounter::new("fallback_qr_cache_misses", "Fallback counter")
                .unwrap(),
            hits_registered: IntCounter::new("fallback_hits_registered", "Fallback counter")
                .unwrap(),
            active_requests: IntGauge::new("fallback_active_requests", "Fallback gauge").unwrap(),
            request_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_request_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            flow_processing_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_flow_processing_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            db_query_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_db_query_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            cache_lookup_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_cache_lookup_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            qr_cache_lookup_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_qr_cache_lookup_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            qr_generation_duration: Histogram::with_opts(HistogramOpts::new(
                "fallback_qr_generation_duration",
                "Fallback histogram",
            ))
            .unwrap(),
            iterative_flow_usage: IntCounter::new(
                "fallback_iterative_flow_usage",
                "Fallback counter",
            )
            .unwrap(),
            recursive_flow_usage: IntCounter::new(
                "fallback_recursive_flow_usage",
                "Fallback counter",
            )
            .unwrap(),
            memory_allocations_per_request: Histogram::with_opts(HistogramOpts::new(
                "fallback_memory_allocations",
                "Fallback histogram",
            ))
            .unwrap(),
        }
    }
}

impl FlowRouterMetrics {
    /// Create a new metrics instance with default Prometheus registry
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(FlowRouterMetrics {
            requests_total: register_int_counter!(
                "flow_router_requests_total",
                "Total number of requests processed by the flow router"
            )?,

            requests_success: register_int_counter!(
                "flow_router_requests_success_total",
                "Number of requests processed successfully"
            )?,

            requests_error: register_int_counter!(
                "flow_router_requests_error_total",
                "Number of requests that resulted in errors"
            )?,

            route_cache_hits: register_int_counter!(
                "flow_router_route_cache_hits_total",
                "Number of cache hits for route lookups"
            )?,

            route_cache_misses: register_int_counter!(
                "flow_router_route_cache_misses_total",
                "Number of cache misses for route lookups"
            )?,

            route_db_queries: register_int_counter!(
                "flow_router_route_db_queries_total",
                "Number of database queries for routes"
            )?,

            user_settings_cache_hits: register_int_counter!(
                "flow_router_user_settings_cache_hits_total",
                "Number of user settings cache hits"
            )?,

            user_settings_cache_misses: register_int_counter!(
                "flow_router_user_settings_cache_misses_total",
                "Number of user settings cache misses"
            )?,

            qr_cache_hits: register_int_counter!(
                "flow_router_qr_cache_hits_total",
                "Number of QR code cache hits"
            )?,

            qr_cache_misses: register_int_counter!(
                "flow_router_qr_cache_misses_total",
                "Number of QR code cache misses"
            )?,

            hits_registered: register_int_counter!(
                "flow_router_hits_registered_total",
                "Number of hits registered"
            )?,

            active_requests: register_int_gauge!(
                "flow_router_active_requests",
                "Current number of active requests being processed"
            )?,

            request_duration: register_histogram!(
                HistogramOpts::new(
                    "flow_router_request_duration_seconds",
                    "Histogram of request processing times in seconds"
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0])
            )?,

            flow_processing_duration: register_histogram!(
                HistogramOpts::new(
                    "flow_router_flow_processing_duration_seconds",
                    "Histogram of flow processing times in seconds (optimized iterative flow)"
                )
                .buckets(vec![0.0001, 0.0005, 0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1])
            )?,

            db_query_duration: register_histogram!(
                HistogramOpts::new(
                    "flow_router_db_query_duration_seconds",
                    "Histogram of database query times in seconds"
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
            )?,

            cache_lookup_duration: register_histogram!(
                HistogramOpts::new(
                    "flow_router_cache_lookup_duration_seconds",
                    "Histogram of cache lookup times in seconds"
                )
                .buckets(vec![0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01])
            )?,

            qr_cache_lookup_duration: register_histogram!(
                HistogramOpts::new(
                    "flow_router_qr_cache_lookup_duration_seconds",
                    "Histogram of QR code cache lookup times in seconds"
                )
                .buckets(vec![0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01])
            )?,

            qr_generation_duration: register_histogram!(
                HistogramOpts::new(
                    "flow_router_qr_generation_duration_seconds",
                    "Histogram of QR code generation times in seconds"
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5])
            )?,

            iterative_flow_usage: register_int_counter!(
                "flow_router_iterative_flow_usage_total",
                "Number of times the optimized iterative flow was used"
            )?,

            recursive_flow_usage: register_int_counter!(
                "flow_router_recursive_flow_usage_total",
                "Number of times the legacy recursive flow was used (should be 0 after optimization)"
            )?,

            memory_allocations_per_request: register_histogram!(
                HistogramOpts::new(
                    "flow_router_memory_allocations_per_request",
                    "Estimated memory allocations per request"
                )
                .buckets(vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0])
            )?,
        })
    }
    
    /// Create a new metrics instance with custom registry
    pub fn with_registry(registry: &Registry) -> Result<Self, prometheus::Error> {
        Ok(FlowRouterMetrics {
            requests_total: IntCounter::with_opts(Opts::new(
                "flow_router_requests_total",
                "Total number of requests processed",
            ))?,

            requests_success: IntCounter::with_opts(Opts::new(
                "flow_router_requests_success_total",
                "Number of successful requests",
            ))?,

            requests_error: IntCounter::with_opts(Opts::new(
                "flow_router_requests_error_total",
                "Number of error requests",
            ))?,

            route_cache_hits: IntCounter::with_opts(Opts::new(
                "flow_router_route_cache_hits_total",
                "Route cache hits",
            ))?,

            route_cache_misses: IntCounter::with_opts(Opts::new(
                "flow_router_route_cache_misses_total",
                "Route cache misses",
            ))?,

            route_db_queries: IntCounter::with_opts(Opts::new(
                "flow_router_route_db_queries_total",
                "Route database queries",
            ))?,

            user_settings_cache_hits: IntCounter::with_opts(Opts::new(
                "flow_router_user_settings_cache_hits_total",
                "User settings cache hits",
            ))?,

            user_settings_cache_misses: IntCounter::with_opts(Opts::new(
                "flow_router_user_settings_cache_misses_total",
                "User settings cache misses",
            ))?,

            qr_cache_hits: IntCounter::with_opts(Opts::new(
                "flow_router_qr_cache_hits_total",
                "QR code cache hits",
            ))?,

            qr_cache_misses: IntCounter::with_opts(Opts::new(
                "flow_router_qr_cache_misses_total",
                "QR code cache misses",
            ))?,

            hits_registered: IntCounter::with_opts(Opts::new(
                "flow_router_hits_registered_total",
                "Hits registered",
            ))?,

            active_requests: IntGauge::with_opts(Opts::new(
                "flow_router_active_requests",
                "Active requests",
            ))?,

            request_duration: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_request_duration_seconds",
                    "Request duration",
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]),
            )?,

            flow_processing_duration: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_flow_processing_duration_seconds",
                    "Flow processing duration",
                )
                .buckets(vec![
                    0.0001, 0.0005, 0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1,
                ]),
            )?,

            db_query_duration: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_db_query_duration_seconds",
                    "Database query duration",
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]),
            )?,

            cache_lookup_duration: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_cache_lookup_duration_seconds",
                    "Cache lookup duration",
                )
                .buckets(vec![0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01]),
            )?,

            qr_cache_lookup_duration: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_qr_cache_lookup_duration_seconds",
                    "QR code cache lookup duration",
                )
                .buckets(vec![0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01]),
            )?,

            qr_generation_duration: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_qr_generation_duration_seconds",
                    "QR code generation duration",
                )
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5]),
            )?,

            iterative_flow_usage: IntCounter::with_opts(Opts::new(
                "flow_router_iterative_flow_usage_total",
                "Iterative flow usage",
            ))?,

            recursive_flow_usage: IntCounter::with_opts(Opts::new(
                "flow_router_recursive_flow_usage_total",
                "Recursive flow usage",
            ))?,

            memory_allocations_per_request: Histogram::with_opts(
                HistogramOpts::new(
                    "flow_router_memory_allocations_per_request",
                    "Memory allocations per request",
                )
                .buckets(vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0]),
            )?,
        })
    }
}

/// Timer helper for measuring durations
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
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

/// Global metrics instance
lazy_static! {
    pub static ref METRICS: FlowRouterMetrics = {
        FlowRouterMetrics::new().unwrap_or_else(|e| {
            // If metrics are already registered, create a default instance
            // This can happen during testing or multiple initializations
            tracing::warn!("Metrics already registered, using default implementation: {}", e);
            FlowRouterMetrics::default()
        })
    };
}

/// Macro for timing operations
#[macro_export]
macro_rules! time_operation {
    ($histogram:expr, $operation:expr) => {{
        let timer = $crate::core::metrics::Timer::new();
        let result = $operation;
        timer.observe_duration_seconds($histogram);
        result
    }};
}

/// Macro for timing async operations
#[macro_export]
macro_rules! time_async_operation {
    ($histogram:expr, $operation:expr) => {{
        let timer = $crate::core::metrics::Timer::new();
        let result = $operation.await;
        timer.observe_duration_seconds($histogram);
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_creation() {
        // Try to create metrics, but if they're already registered (from parallel tests),
        // use the default implementation instead
        let metrics = FlowRouterMetrics::new().unwrap_or_else(|_| FlowRouterMetrics::default());

        // Test counter increment - note: if using default, this won't reflect in global registry
        // but we're testing the API contract works
        let initial_value = metrics.requests_total.get();
        metrics.requests_total.inc();
        assert_eq!(metrics.requests_total.get(), initial_value + 1);

        // Test gauge
        let initial_active = metrics.active_requests.get();
        metrics.active_requests.inc();
        assert_eq!(metrics.active_requests.get(), initial_active + 1);
        metrics.active_requests.dec();
        assert_eq!(metrics.active_requests.get(), initial_active);
    }
    
    #[test]
    fn test_timer() {
        let timer = Timer::new();
        thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed_seconds();
        assert!(elapsed >= 0.01);
        assert!(elapsed < 0.1); // Should be much less than 100ms
    }
    
    #[test]
    fn test_histogram_observation() {
        // Try to create metrics, but if they're already registered (from parallel tests),
        // use the default implementation instead
        let metrics = FlowRouterMetrics::new().unwrap_or_else(|_| FlowRouterMetrics::default());
        let timer = Timer::new();
        thread::sleep(Duration::from_millis(1));

        let initial_count = metrics.request_duration.get_sample_count();
        timer.observe_duration_seconds(&metrics.request_duration);

        // Check that the histogram recorded the observation
        assert!(metrics.request_duration.get_sample_count() > initial_count);
    }
}
