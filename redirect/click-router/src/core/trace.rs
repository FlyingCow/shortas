//! Debug tracing module for click-router
//!
//! Provides distributed tracing capabilities that are enabled only for routes
//! with `allow_debug: true`. Traces capture timing information for each stage
//! of request processing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Represents a single span in the trace - a named, timed segment of processing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceSpan {
    /// Name of the processing stage (e.g., "Start", "UrlExtract", "Register")
    pub name: String,
    /// Start time in milliseconds from trace start
    pub start_ms: f64,
    /// Duration of this span in milliseconds
    pub duration_ms: f64,
}

impl TraceSpan {
    pub fn new(name: String, start_ms: f64, duration_ms: f64) -> Self {
        Self {
            name,
            start_ms,
            duration_ms,
        }
    }
}

/// Complete trace data for a single hit/request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HitTrace {
    /// Unique trace identifier (matches the hit ID)
    pub trace_id: String,
    /// All spans collected during processing
    pub spans: Vec<TraceSpan>,
    /// Total processing time in milliseconds
    pub total_ms: f64,
    /// UTC timestamp when the trace was finalized (router exit time)
    pub router_exit_utc: DateTime<Utc>,
}

impl HitTrace {
    pub fn new(trace_id: String, spans: Vec<TraceSpan>, total_ms: f64) -> Self {
        Self {
            trace_id,
            spans,
            total_ms,
            router_exit_utc: Utc::now(),
        }
    }
}

/// Collector for building traces during request processing
///
/// Usage:
/// ```ignore
/// let mut trace = TraceCollector::new("request_id");
/// trace.start_span("Start");
/// // ... do work ...
/// trace.end_span();
/// let hit_trace = trace.finalize();
/// ```
pub struct TraceCollector {
    trace_id: String,
    start: Instant,
    spans: Vec<TraceSpan>,
    current_span: Option<(String, Instant)>,
}

impl TraceCollector {
    /// Create a new trace collector with the given trace ID
    pub fn new(trace_id: &str) -> Self {
        Self {
            trace_id: trace_id.to_string(),
            start: Instant::now(),
            spans: Vec::with_capacity(8), // Pre-allocate for typical flow stages
            current_span: None,
        }
    }

    /// Start a new span with the given name
    ///
    /// If a span is already in progress, it will be automatically ended first.
    pub fn start_span(&mut self, name: &str) {
        // End any existing span first
        if self.current_span.is_some() {
            self.end_span();
        }
        self.current_span = Some((name.to_string(), Instant::now()));
    }

    /// End the current span and record its duration
    ///
    /// Does nothing if no span is in progress.
    pub fn end_span(&mut self) {
        if let Some((name, span_start)) = self.current_span.take() {
            let start_ms = span_start.duration_since(self.start).as_secs_f64() * 1000.0
                - span_start.elapsed().as_secs_f64() * 1000.0;
            let duration_ms = span_start.elapsed().as_secs_f64() * 1000.0;

            // Calculate start_ms from the trace start
            let start_ms = self.start.elapsed().as_secs_f64() * 1000.0 - duration_ms;

            self.spans.push(TraceSpan::new(name, start_ms, duration_ms));
        }
    }

    /// Get the duration of the current span in seconds (for metrics)
    ///
    /// Returns 0.0 if no span is in progress.
    pub fn current_span_duration_secs(&self) -> f64 {
        self.current_span
            .as_ref()
            .map(|(_, start)| start.elapsed().as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Finalize the trace and return the complete HitTrace
    ///
    /// This consumes the collector and returns the finished trace data.
    pub fn finalize(mut self) -> HitTrace {
        // End any in-progress span
        if self.current_span.is_some() {
            self.end_span();
        }

        let total_ms = self.start.elapsed().as_secs_f64() * 1000.0;

        HitTrace::new(self.trace_id, self.spans, total_ms)
    }

    /// Get elapsed time since trace start in seconds
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_trace_collector_basic() {
        let mut trace = TraceCollector::new("test_trace_1");

        trace.start_span("Start");
        thread::sleep(Duration::from_millis(10));
        trace.end_span();

        trace.start_span("Process");
        thread::sleep(Duration::from_millis(20));
        trace.end_span();

        let hit_trace = trace.finalize();

        assert_eq!(hit_trace.trace_id, "test_trace_1");
        assert_eq!(hit_trace.spans.len(), 2);
        assert_eq!(hit_trace.spans[0].name, "Start");
        assert_eq!(hit_trace.spans[1].name, "Process");
        assert!(hit_trace.spans[0].duration_ms >= 10.0);
        assert!(hit_trace.spans[1].duration_ms >= 20.0);
        assert!(hit_trace.total_ms >= 30.0);
    }

    #[test]
    fn test_trace_collector_auto_end_span() {
        let mut trace = TraceCollector::new("test_trace_2");

        trace.start_span("First");
        thread::sleep(Duration::from_millis(5));
        // Don't explicitly end - starting new span should auto-end
        trace.start_span("Second");
        thread::sleep(Duration::from_millis(5));

        let hit_trace = trace.finalize();

        assert_eq!(hit_trace.spans.len(), 2);
        assert_eq!(hit_trace.spans[0].name, "First");
        assert_eq!(hit_trace.spans[1].name, "Second");
    }

    #[test]
    fn test_trace_collector_finalize_ends_current_span() {
        let mut trace = TraceCollector::new("test_trace_3");

        trace.start_span("InProgress");
        thread::sleep(Duration::from_millis(5));
        // Don't end span - finalize should handle it

        let hit_trace = trace.finalize();

        assert_eq!(hit_trace.spans.len(), 1);
        assert_eq!(hit_trace.spans[0].name, "InProgress");
    }

    #[test]
    fn test_hit_trace_serialization() {
        let trace = HitTrace::new(
            "test_id".to_string(),
            vec![TraceSpan::new("Test".to_string(), 0.0, 10.0)],
            10.0,
        );

        let json = serde_json::to_string(&trace).unwrap();
        let deserialized: HitTrace = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.trace_id, "test_id");
        assert_eq!(deserialized.spans.len(), 1);
        assert_eq!(deserialized.total_ms, 10.0);
    }
}
