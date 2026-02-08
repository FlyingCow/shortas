use salvo::oapi::endpoint;
use salvo::prelude::*;
use tracing::info;

use crate::adapters::api::routes::domain_controller;

#[handler]
async fn request_logger(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let query = req.uri().query().unwrap_or("");

    if !query.is_empty() {
        info!("Incoming request: {} {}?{}", method, path, query);
    } else {
        info!("Incoming request: {} {}", method, path);
    }

    ctrl.call_next(req, depot, res).await;

    if let Some(status) = res.status_code {
        info!("Response status: {}", status);
    }
}

pub fn routes() -> Router {
    let public_routes = Router::with_path("/public")
        .push(Router::with_path("/health").get(health_check))
        .push(Router::with_path("/metrics").get(metrics_endpoint));

    let protected_routes = Router::with_path("/v1")
        .push(domain_controller::api_routes())
        .push(Router::with_path("/dns-config").get(domain_controller::get_dns_config));

    Router::new()
        .hoop(request_logger)
        .push(public_routes)
        .push(protected_routes)
}

#[endpoint]
pub async fn health_check(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": "0.1.0",
        "service": "domain-verifier"
    })));
}

#[endpoint]
pub async fn metrics_endpoint(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Json(serde_json::json!({
        "requests_total": 0,
        "verifications_total": 0,
        "verifications_success": 0,
        "verifications_failed": 0,
        "uptime_seconds": 0
    })));
}
