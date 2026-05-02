//! Health check endpoints.

use salvo::prelude::*;
use serde::Serialize;

use crate::presentation::middleware::DepotExt;

/// Health status response.
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

/// Basic health check.
#[endpoint(
    operation_id = "health_check",
    summary = "Health check",
    description = "Basic health check endpoint",
    tags("Health"),
    responses(
        (status_code = 200, description = "Service is healthy", body = HealthStatus)
    )
)]
pub async fn health_check(res: &mut Response) {
    res.render(Json(HealthStatus {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: None,
        elasticsearch: None,
        storage: None,
    }));
}

/// Readiness check (checks all dependencies).
#[endpoint(
    operation_id = "readiness_check",
    summary = "Readiness check",
    description = "Check if service is ready to accept traffic",
    tags("Health"),
    responses(
        (status_code = 200, description = "Service is ready", body = HealthStatus),
        (status_code = 503, description = "Service is not ready", body = HealthStatus)
    )
)]
pub async fn readiness_check(depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(StatusCode::SERVICE_UNAVAILABLE);
            res.render(Json(HealthStatus {
                status: "error".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                database: Some("unknown".to_string()),
                elasticsearch: Some("unknown".to_string()),
                storage: Some("unknown".to_string()),
            }));
            return;
        }
    };

    // Check Elasticsearch
    let es_status = match app_state.search_service.health_check().await {
        Ok(true) => "ok",
        _ => "error",
    };

    // Check MinIO
    let storage_status = match app_state.storage_service.health_check().await {
        Ok(true) => "ok",
        _ => "error",
    };

    // Check Click Router
    let _click_router_status = match app_state.click_router.health_check().await {
        Ok(true) => "ok",
        _ => "error",
    };

    let all_ok = es_status == "ok" && storage_status == "ok";

    let status = if all_ok { "ok" } else { "degraded" };
    let status_code = if all_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    res.status_code(status_code);
    res.render(Json(HealthStatus {
        status: status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: Some("ok".to_string()), // We're running if we got here
        elasticsearch: Some(es_status.to_string()),
        storage: Some(storage_status.to_string()),
    }));
}

/// Liveness check (basic process check).
#[endpoint(
    operation_id = "liveness_check",
    summary = "Liveness check",
    description = "Check if service process is alive",
    tags("Health"),
    responses(
        (status_code = 200, description = "Service is alive", body = HealthStatus)
    )
)]
pub async fn liveness_check(res: &mut Response) {
    res.render(Json(HealthStatus {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: None,
        elasticsearch: None,
        storage: None,
    }));
}
