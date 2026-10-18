//! HTTP endpoints for metrics and health monitoring
//!
//! This module provides HTTP handlers for exposing application metrics and health
//! information to monitoring systems like Prometheus and health check services.
//!
//! ## Endpoints
//! - `/metrics` - Prometheus-compatible metrics in text format
//! - `/health` - Health check with basic system information
//! - `/metrics/info` - Detailed metrics information in JSON format
//!
//! ## Integration
//! These endpoints are typically served on a separate port from the main application
//! to isolate monitoring traffic from user traffic.

use prometheus::TextEncoder;
use axum::{
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::{header, StatusCode};
use crate::core::metrics::METRICS;

/// Handler for the Prometheus metrics endpoint
///
/// Serves metrics in Prometheus text format for scraping by monitoring systems.
/// Returns all registered metrics with their current values.
///
/// # Response
/// - Content-Type: `text/plain; version=0.0.4; charset=utf-8`
/// - Body: Prometheus-formatted metrics data
pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    match encoder.encode_to_string(&metric_families) {
        Ok(metrics_text) => {
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
                metrics_text,
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to encode metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to encode metrics",
            )
                .into_response()
        }
    }
}

/// Handler for the health check endpoint
///
/// Provides a health status response with basic system metrics for monitoring
/// and load balancer health checks.
///
/// # Response
/// - Content-Type: `application/json`
/// - Body: JSON object with status, timestamp, and key metrics
pub async fn health_handler() -> impl IntoResponse {
    let health_info = serde_json::json!({
        "status": "healthy",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        "metrics": {
            "requests_total": METRICS.requests_total.get(),
            "requests_success": METRICS.requests_success.get(),
            "requests_error": METRICS.requests_error.get(),
            "active_requests": METRICS.active_requests.get(),
            "route_cache_hits": METRICS.route_cache_hits.get(),
            "route_cache_misses": METRICS.route_cache_misses.get(),
            "iterative_flow_usage": METRICS.iterative_flow_usage.get(),
            "recursive_flow_usage": METRICS.recursive_flow_usage.get(),
        }
    });

    Json(health_info)
}

/// Handler for detailed metrics information endpoint
///
/// Provides comprehensive metrics data in JSON format with calculated
/// rates and performance indicators.
///
/// # Response
/// - Content-Type: `application/json`
/// - Body: JSON object with detailed metrics, rates, and performance data
pub async fn metrics_info_handler() -> impl IntoResponse {
    let cache_hit_rate = {
        let hits = METRICS.route_cache_hits.get() as f64;
        let misses = METRICS.route_cache_misses.get() as f64;
        let total = hits + misses;
        if total > 0.0 {
            (hits / total) * 100.0
        } else {
            0.0
        }
    };
    
    let user_settings_cache_hit_rate = {
        let hits = METRICS.user_settings_cache_hits.get() as f64;
        let misses =  METRICS.user_settings_cache_misses.get() as f64;
        let total = hits + misses;
        if total > 0.0 {
            (hits / total) * 100.0
        } else {
            0.0
        }
    };
    
    let optimization_usage = {
        let iterative = METRICS.iterative_flow_usage.get() as f64;
        let recursive = METRICS.recursive_flow_usage.get() as f64;
        let total = iterative + recursive;
        if total > 0.0 {
            (iterative / total) * 100.0
        } else {
            0.0
        }
    };
    
    let metrics_info = serde_json::json!({
        "performance_metrics": {
            "total_requests": METRICS.requests_total.get(),
            "successful_requests": METRICS.requests_success.get(),
            "error_requests": METRICS.requests_error.get(),
            "active_requests": METRICS.active_requests.get(),
            "success_rate_percent": if METRICS.requests_total.get() > 0 {
                (METRICS.requests_success.get() as f64 / METRICS.requests_total.get() as f64) * 100.0
            } else {
                0.0
            }
        },
        "cache_performance": {
            "route_cache_hits": METRICS.route_cache_hits.get(),
            "route_cache_misses": METRICS.route_cache_misses.get(),
            "route_cache_hit_rate_percent": cache_hit_rate,
            "user_settings_cache_hits": METRICS.user_settings_cache_hits.get(),
            "user_settings_cache_misses": METRICS.user_settings_cache_misses.get(),
            "user_settings_cache_hit_rate_percent": user_settings_cache_hit_rate,
        },
        "optimization_impact": {
            "iterative_flow_usage": METRICS.iterative_flow_usage.get(),
            "recursive_flow_usage": METRICS.recursive_flow_usage.get(),
            "optimization_usage_percent": optimization_usage,
            "description": "Higher optimization_usage_percent indicates better performance (should be close to 100%)"
        },
        "database_operations": {
            "route_db_queries": METRICS.route_db_queries.get(),
            "hits_registered": METRICS.hits_registered.get(),
        },
        "timing_histograms": {
            "request_duration": {
                "sample_count": METRICS.request_duration.get_sample_count(),
                "sample_sum": METRICS.request_duration.get_sample_sum(),
            },
            "flow_processing_duration": {
                "sample_count": METRICS.flow_processing_duration.get_sample_count(),
                "sample_sum": METRICS.flow_processing_duration.get_sample_sum(),
            },
            "db_query_duration": {
                "sample_count": METRICS.db_query_duration.get_sample_count(),
                "sample_sum": METRICS.db_query_duration.get_sample_sum(),
            },
            "cache_lookup_duration": {
                "sample_count": METRICS.cache_lookup_duration.get_sample_count(),
                "sample_sum": METRICS.cache_lookup_duration.get_sample_sum(),
            }
        }
    });

    Json(metrics_info)
}

/// Create router for metrics endpoints
pub fn create_metrics_router() -> Router {
    Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health_handler))
        .route("/metrics/info", get(metrics_info_handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use salvo::test::{ResponseExt, TestClient};

    #[tokio::test]
    async fn test_health_endpoint() {
        let router = create_metrics_router();
        let service = Service::new(router);
        
        let content = TestClient::get("http://127.0.0.1:5800/health")
            .send(&service)
            .await
            .take_json::<serde_json::Value>()
            .await
            .unwrap();
            
        assert_eq!(content["status"], "healthy");
        assert!(content["metrics"].is_object());
    }
    
    #[tokio::test]
    async fn test_metrics_info_endpoint() {
        let router = create_metrics_router();
        let service = Service::new(router);
        
        let content = TestClient::get("http://127.0.0.1:5800/metrics/info")
            .send(&service)
            .await
            .take_json::<serde_json::Value>()
            .await
            .unwrap();
            
        assert!(content["performance_metrics"].is_object());
        assert!(content["cache_performance"].is_object());
        assert!(content["optimization_impact"].is_object());
    }
}
