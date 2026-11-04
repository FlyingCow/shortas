use salvo::oapi::endpoint;
use salvo::prelude::*;
use tracing::info;

use crate::adapters::api::routes::{
    challenge_controller, crypto_controller, custom_pages_controller, routes_controller,
    user_settings_controller,
};

// Simple request logging middleware
#[handler]
async fn request_logger(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let query = req.uri().query().unwrap_or("");

    if !query.is_empty() {
        info!("📨 Incoming request: {} {}?{}", method, path, query);
    } else {
        info!("📨 Incoming request: {} {}", method, path);
    }

    ctrl.call_next(req, depot, res).await;

    if let Some(status) = res.status_code {
        info!("📤 Response status: {}", status);
    }
}

pub fn routes() -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::with_path("/public")
        .push(Router::with_path("/health").get(health_check))
        .push(Router::with_path("/metrics").get(metrics_endpoint));

    // API routes (all authentication and validation disabled for testing)
    let protected_routes = Router::with_path("/v1")
        //.hoop(security_headers_middleware)
        // .hoop(rate_limit_middleware)         // Disabled for testing
        // .hoop(validation_middleware)         // Disabled for testing
        // .hoop(jwt_auth_middleware)           // Disabled for testing
        // .hoop(jwt_authorization_middleware)  // Disabled for testing
        .push(routes_controller::api_routes())
        .push(crypto_controller::api_routes())
        .push(user_settings_controller::api_routes())
        .push(challenge_controller::api_routes())
        .push(custom_pages_controller::api_routes());

    // Combine all routes with logging
    Router::new()
        .hoop(request_logger)  // Log all requests
        .push(public_routes)
        .push(protected_routes)
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
