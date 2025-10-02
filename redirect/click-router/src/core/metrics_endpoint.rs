use prometheus::{Encoder, TextEncoder};
use salvo::{prelude::*, writing::Json, writing::Text, Response};
use crate::core::metrics::METRICS;

/// Handler for Prometheus metrics endpoint
#[handler]
pub async fn metrics_handler(res: &mut Response) {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    
    match encoder.encode_to_string(&metric_families) {
        Ok(metrics_text) => {
            res.add_header("Content-Type", "text/plain; version=0.0.4; charset=utf-8", true)
                .unwrap();
            res.render(Text::Plain(metrics_text));
        }
        Err(e) => {
            tracing::error!("Failed to encode metrics: {}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Text::Plain("Failed to encode metrics"));
        }
    }
}

/// Handler for health check endpoint
#[handler]
pub async fn health_handler(res: &mut Response) {
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
    
    res.render(Json(health_info));
}

/// Handler for detailed metrics information
#[handler]
pub async fn metrics_info_handler(res: &mut Response) {
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
        let misses = METRICS.user_settings_cache_misses.get() as f64;
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
    
    res.render(Json(metrics_info));
}

/// Create router for metrics endpoints
pub fn create_metrics_router() -> Router {
    Router::new()
        .push(Router::with_path("/metrics").get(metrics_handler))
        .push(Router::with_path("/health").get(health_handler))
        .push(Router::with_path("/metrics/info").get(metrics_info_handler))
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
