use salvo::oapi::endpoint;
use salvo::prelude::*;

use crate::adapters::api::routes::clickstream_controller;

pub fn routes() -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::with_path("/public")
        .push(Router::with_path("/health").get(health_check))
        .push(Router::with_path("/metrics").get(metrics_endpoint));

    // Protected API routes (authorization disabled)
    let protected_routes = Router::with_path("/v1").push(clickstream_controller::api_routes());

    // Combine all routes
    Router::new().push(public_routes).push(protected_routes)
}

/// Health check endpoint
///
/// Returns the current health status of the API service.
/// This endpoint is public and does not require authentication.
#[endpoint]
pub async fn health_check(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": "0.1.0"
    })));
}

/// Metrics endpoint
///
/// Returns basic metrics about the API service.
/// This endpoint is public and does not require authentication.
#[endpoint]
pub async fn metrics_endpoint(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    // Basic metrics endpoint - in production, you'd want to integrate with Prometheus
    res.render(Json(serde_json::json!({
        "requests_total": 0,
        "errors_total": 0,
        "uptime_seconds": 0
    })));
}
