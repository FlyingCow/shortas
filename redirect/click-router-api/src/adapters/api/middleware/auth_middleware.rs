//! Security middleware for the Click Router API
//! 
//! This module provides rate limiting, input validation, and security headers
//! middleware for securing API endpoints.

use salvo::{prelude::*, Request, Response, FlowCtrl};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

use crate::adapters::api::error_presenter::ErrorResponse;
use crate::model::error::ApiError;

/// Rate limiting storage
#[derive(Clone)]
pub struct RateLimitStore {
    store: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

#[derive(Clone, Debug)]
struct RateLimitEntry {
    count: u32,
    reset_time: std::time::SystemTime,
}

impl RateLimitStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn check_rate_limit(&self, key: &str, limit: u32, window_seconds: u64) -> bool {
        let now = std::time::SystemTime::now();
        
        let should_allow = {
            let mut store = self.store.write().await;
            let entry = store.get(key).cloned();
            match entry {
                Some(entry) => {
                    if now < entry.reset_time {
                        if entry.count >= limit {
                            false
                        } else {
                            store.insert(key.to_string(), RateLimitEntry {
                                count: entry.count + 1,
                                reset_time: entry.reset_time,
                            });
                            true
                        }
                    } else {
                        store.insert(key.to_string(), RateLimitEntry {
                            count: 1,
                            reset_time: now + std::time::Duration::from_secs(window_seconds),
                        });
                        true
                    }
                }
                None => {
                    store.insert(key.to_string(), RateLimitEntry {
                        count: 1,
                        reset_time: now + std::time::Duration::from_secs(window_seconds),
                    });
                    true
                }
            }
        };
        
        should_allow
    }
}


/// Rate limiting middleware
#[handler]
pub async fn rate_limit_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // Get rate limit store from depot
    let rate_limit_store = match depot.get::<RateLimitStore>("rate_limit_store") {
        Ok(store) => store,
        Err(_) => {
            warn!("RateLimitStore not found in depot");
            ctrl.call_next(req, depot, res).await;
            return;
        }
    };
    
    // Get client identifier (IP + User-Agent)
    let client_id = get_client_identifier(req);
    
    // Apply rate limiting (100 requests per minute by default)
    if !rate_limit_store.check_rate_limit(&client_id, 100, 60).await {
        let error_response = ErrorResponse::from_api_error(&ApiError::ExternalService(
            crate::model::error::ExternalServiceError::RateLimited(
                "Rate limit exceeded. Try again later.".to_string()
            )
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }
    
    ctrl.call_next(req, depot, res).await;
}


/// Input validation middleware
#[handler]
pub async fn validation_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // Validate request parameters
    if let Err(error) = validate_request_parameters(req).await {
        let error_response = ErrorResponse::from_api_error(&error);
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }
    
    // Validate request body if present
    if let Err(error) = validate_request_body(req).await {
        let error_response = ErrorResponse::from_api_error(&error);
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }
    
    ctrl.call_next(req, depot, res).await;
}

/// Security headers middleware
#[handler]
pub async fn security_headers_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // Add security headers
    res.headers_mut().insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap(),
    );
    res.headers_mut().insert(
        "X-Frame-Options",
        "DENY".parse().unwrap(),
    );
    res.headers_mut().insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );
    res.headers_mut().insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    res.headers_mut().insert(
        "Content-Security-Policy",
        "default-src 'self'".parse().unwrap(),
    );
    
    ctrl.call_next(req, depot, res).await;
}

// Helper functions


fn get_client_identifier(req: &Request) -> String {
    let ip = req.remote_addr().to_string();
    let user_agent = req.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    
    format!("{}:{}", ip, user_agent)
}

async fn validate_request_parameters(req: &Request) -> Result<(), ApiError> {
    // Validate path parameters
    for (key, value) in req.params().iter() {
        if value.len() > 1000 {
            return Err(ApiError::validation_error(
                key.to_string(),
                "Parameter value too long".to_string()
            ));
        }
        
        // Check for potential injection attacks
        if value.contains("'") || value.contains("\"") || value.contains(";") {
            return Err(ApiError::validation_error(
                key.to_string(),
                "Invalid characters in parameter".to_string()
            ));
        }
    }
    
    Ok(())
}

async fn validate_request_body(req: &Request) -> Result<(), ApiError> {
    // Check content length
    if let Some(content_length) = req.headers().get("Content-Length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                if length > 1024 * 1024 { // 1MB limit
                    return Err(ApiError::validation_error(
                        "body".to_string(),
                        "Request body too large".to_string()
                    ));
                }
            }
        }
    }
    
    Ok(())
}
