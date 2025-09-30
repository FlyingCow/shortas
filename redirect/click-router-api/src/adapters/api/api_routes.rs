use salvo::prelude::*;
use salvo::oapi::endpoint;

use crate::adapters::api::{
    middleware::{jwt_auth_middleware_fn as jwt_auth_middleware, jwt_authorization_middleware, rate_limit_middleware, validation_middleware, security_headers_middleware},
    routes::{routes_controller, crypto_controller, user_settings_controller},
};

pub fn routes() -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::with_path("/public")
        .push(Router::with_path("/health").get(health_check))
        .push(Router::with_path("/metrics").get(metrics_endpoint));
    
    // Protected API routes (require JWT authentication and authorization)
    let protected_routes = Router::with_path("/v1")
        .hoop(security_headers_middleware)
        .hoop(rate_limit_middleware)
        .hoop(validation_middleware)
        .hoop(jwt_auth_middleware)
        .hoop(jwt_authorization_middleware)
        .push(routes_controller::api_routes())
        .push(crypto_controller::api_routes())
        .push(user_settings_controller::api_routes());
    
    // Combine all routes
    Router::new()
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