use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::net::IpAddr;

use crate::model::{Hit, hit::HitRoute};
use crate::core::{ConversionEvent, ConversionFunnelStep, hits_register::HitRegistrar};
use crate::get_flow_router;

/// Request DTO for conversion tracking
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversionRequest {
    pub route_id: String,
    pub conversion_type: String,
    pub conversion_name: String,
    pub conversion_value: Option<f64>,
    pub attributed_click_id: Option<String>,
    pub attribution_type: Option<String>,
    pub attribution_window_hours: Option<u32>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Request DTO for funnel step tracking
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunnelStepRequest {
    pub route_id: String,
    pub funnel_name: String,
    pub funnel_steps: Option<Vec<String>>,
    pub step_name: String,
    pub step_position: u8,
    pub step_value: Option<f64>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Handler for conversion tracking
pub struct ConversionHandler;

#[async_trait]
impl Handler for ConversionHandler {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let conversion_data: ConversionRequest = match req.parse_json().await {
            Ok(data) => data,
            Err(_) => {
                res.status_code(salvo::http::StatusCode::BAD_REQUEST);
                res.render(Json(serde_json::json!({"error": "Invalid JSON data"})));
                return;
            }
        };

        // Extract IP address from request
        let client_ip = req.remote_addr()
            .clone()
            .into_std()
            .map(|std_addr| std_addr.ip())
            .or_else(|| {
                req.headers()
                    .get("x-forwarded-for")
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header| header.split(',').next())
                    .and_then(|ip_str| ip_str.trim().parse::<IpAddr>().ok())
            });

        // Create conversion event
        let conversion_event = ConversionEvent {
            id: ulid::Ulid::new().to_string(),
            owner_id: None, // Will be enriched by click-tracker
            creator_id: None,
            route_id: Some(conversion_data.route_id.clone()),
            workspace_id: None,
            conversion_type: conversion_data.conversion_type,
            conversion_name: conversion_data.conversion_name,
            conversion_value: conversion_data.conversion_value,
            attributed_click_id: conversion_data.attributed_click_id,
            attribution_type: conversion_data.attribution_type.unwrap_or_else(|| "direct".to_string()),
            attribution_window_hours: conversion_data.attribution_window_hours.unwrap_or(24),
            user_id: conversion_data.user_id,
            session_id: conversion_data.session_id,
            ip: client_ip,
            continent: None, // Will be enriched by click-tracker
            country: None,
            location: None,
            device_family: None,
            device_brand: None,
            device_model: None,
            os_family: None,
            os_version: None,
            user_agent_family: None,
            user_agent_version: None,
            created: Utc::now(),
            click_created: None,
            metadata: conversion_data.metadata.map(|m| m.to_string()),
            referrer: req.headers()
                .get("referer")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string()),
            is_unique: Some(1),
        };

        // Create hit with conversion data
        let hit = Hit::conversion(
            &conversion_event.id,
            Utc::now(),
            req.headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok()),
            client_ip,
            conversion_event.clone(),
            Some(HitRoute {
                id: Some(conversion_data.route_id),
                owner_id: None,
                creator_id: None,
                workspace_id: None,
            }),
        );

        // Get the flow router and register the hit
        let flow_router = get_flow_router();
        match flow_router.hit_registrar().register(&hit).await {
            Ok(_) => {
                res.status_code(salvo::http::StatusCode::CREATED);
                res.render(Json(serde_json::json!({
                    "success": true,
                    "conversion_id": hit.id,
                    "message": "Conversion tracked successfully"
                })));
            }
            Err(e) => {
                tracing::error!("Failed to register conversion: {:?}", e);
                res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(serde_json::json!({
                    "error": "Failed to track conversion"
                })));
            }
        }
    }
}

/// Handler for funnel step tracking
pub struct FunnelStepHandler;

#[async_trait]
impl Handler for FunnelStepHandler {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let funnel_data: FunnelStepRequest = match req.parse_json().await {
            Ok(data) => data,
            Err(_) => {
                res.status_code(salvo::http::StatusCode::BAD_REQUEST);
                res.render(Json(serde_json::json!({"error": "Invalid JSON data"})));
                return;
            }
        };

        // Extract IP address from request
        let client_ip = req.remote_addr()
            .clone()
            .into_std()
            .map(|std_addr| std_addr.ip())
            .or_else(|| {
                req.headers()
                    .get("x-forwarded-for")
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header| header.split(',').next())
                    .and_then(|ip_str| ip_str.trim().parse::<IpAddr>().ok())
            });

        // Create funnel step event
        let funnel_step = ConversionFunnelStep {
            id: ulid::Ulid::new().to_string(),
            owner_id: None, // Will be enriched by click-tracker
            workspace_id: None,
            funnel_name: funnel_data.funnel_name,
            funnel_steps: funnel_data.funnel_steps.unwrap_or_default(),
            user_id: funnel_data.user_id,
            session_id: funnel_data.session_id,
            route_id: Some(funnel_data.route_id.clone()),
            step_name: funnel_data.step_name,
            step_position: funnel_data.step_position,
            step_completed: 1,
            step_value: funnel_data.step_value,
            step_created: Utc::now(),
            funnel_started: Some(Utc::now()),
            metadata: funnel_data.metadata.map(|m| m.to_string()),
        };

        // Create hit with funnel step data
        let hit = Hit::funnel_step(
            &funnel_step.id,
            Utc::now(),
            req.headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok()),
            client_ip,
            funnel_step.clone(),
            Some(HitRoute {
                id: Some(funnel_data.route_id),
                owner_id: None,
                creator_id: None,
                workspace_id: None,
            }),
        );

        // Get the flow router and register the hit
        let flow_router = get_flow_router();
        match flow_router.hit_registrar().register(&hit).await {
            Ok(_) => {
                res.status_code(salvo::http::StatusCode::CREATED);
                res.render(Json(serde_json::json!({
                    "success": true,
                    "funnel_step_id": hit.id,
                    "message": "Funnel step tracked successfully"
                })));
            }
            Err(e) => {
                tracing::error!("Failed to register funnel step: {:?}", e);
                res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(serde_json::json!({
                    "error": "Failed to track funnel step"
                })));
            }
        }
    }
}

/// Create conversion tracking routes
pub fn conversion_routes() -> Router {
    Router::with_path("/conversions")
        .push(Router::with_path("/track").post(ConversionHandler))
        .push(Router::with_path("/funnel").post(FunnelStepHandler))
}
