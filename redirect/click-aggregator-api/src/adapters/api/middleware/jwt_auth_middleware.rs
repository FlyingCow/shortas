//! JWT Authentication middleware for the Click Aggregator API
//! 
//! This module provides JWT token validation and Keycloak integration
//! for securing API endpoints.

use salvo::{prelude::*, Request, Response, FlowCtrl};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{warn, error};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use base64::Engine;

use crate::adapters::api::error_presenter::ErrorResponse;
use crate::model::error::{ApiError, AuthenticationError};

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,                    // Subject (user ID)
    pub iss: String,                    // Issuer
    pub aud: String,                    // Audience
    pub exp: i64,                       // Expiration time
    pub iat: i64,                       // Issued at
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<HashMap<String, ResourceAccess>>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAccess {
    pub roles: Vec<String>,
}

/// Authentication context for JWT tokens
#[derive(Clone, Debug)]
pub struct JwtAuthContext {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub realm_roles: Vec<String>,
    pub resource_roles: HashMap<String, Vec<String>>,
    pub scope: Option<String>,
    pub is_authenticated: bool,
    pub token_type: TokenType,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    AccessToken,
    RptToken,
}

impl JwtAuthContext {
    pub fn new(claims: JwtClaims, token_type: TokenType) -> Self {
        let realm_roles = claims.realm_access
            .map(|ra| ra.roles)
            .unwrap_or_default();
        
        let resource_roles = claims.resource_access
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v.roles))
            .collect();
        
        Self {
            user_id: claims.sub,
            username: claims.preferred_username,
            email: claims.email,
            name: claims.name,
            realm_roles,
            resource_roles,
            scope: claims.scope,
            is_authenticated: true,
            token_type,
        }
    }
    
    pub fn anonymous() -> Self {
        Self {
            user_id: "anonymous".to_string(),
            username: None,
            email: None,
            name: None,
            realm_roles: vec![],
            resource_roles: HashMap::new(),
            scope: None,
            is_authenticated: false,
            token_type: TokenType::AccessToken,
        }
    }
    
    pub fn has_realm_role(&self, role: &str) -> bool {
        self.realm_roles.contains(&role.to_string())
    }
    
    pub fn has_resource_role(&self, resource: &str, role: &str) -> bool {
        self.resource_roles
            .get(resource)
            .map(|roles| roles.contains(&role.to_string()))
            .unwrap_or(false)
    }
    
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scope
            .as_ref()
            .map(|s| s.split(' ').any(|s| s == scope))
            .unwrap_or(false)
    }
}

/// Keycloak configuration
#[derive(Debug, Clone)]
pub struct KeycloakConfig {
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub introspection_endpoint: String,
    pub jwks_endpoint: String,
}

impl KeycloakConfig {
    pub fn new(base_url: String, realm: String, client_id: String) -> Self {
        let introspection_endpoint = format!("{}/realms/{}/protocol/openid-connect/token/introspect", base_url, realm);
        let jwks_endpoint = format!("{}/realms/{}/protocol/openid-connect/certs", base_url, realm);
        
        Self {
            base_url,
            realm,
            client_id,
            client_secret: None,
            introspection_endpoint,
            jwks_endpoint,
        }
    }
}

/// JWKS (JSON Web Key Set) cache
#[derive(Clone)]
pub struct JwksCache {
    cache: Arc<RwLock<HashMap<String, DecodingKey>>>,
    client: Client,
    jwks_endpoint: String,
}

impl JwksCache {
    pub fn new(jwks_endpoint: String) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            client: Client::new(),
            jwks_endpoint,
        }
    }
    
    pub async fn get_key(&self, kid: &str) -> Result<Option<DecodingKey>, ApiError> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(key) = cache.get(kid) {
                return Ok(Some(key.clone()));
            }
        }
        
        // Fetch from Keycloak
        self.fetch_jwks().await?;
        
        // Check cache again
        let cache = self.cache.read().await;
        Ok(cache.get(kid).cloned())
    }
    
    async fn fetch_jwks(&self) -> Result<(), ApiError> {
        let response = self.client
            .get(&self.jwks_endpoint)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(
                crate::model::error::ExternalServiceError::Unavailable(format!("Failed to fetch JWKS: {}", e))
            ))?;
        
        if !response.status().is_success() {
            return Err(ApiError::ExternalService(
                crate::model::error::ExternalServiceError::Unavailable("Failed to fetch JWKS from Keycloak".to_string())
            ));
        }
        
        let jwks: JwksResponse = response.json().await
            .map_err(|e| ApiError::ExternalService(
                crate::model::error::ExternalServiceError::Unavailable(format!("Failed to parse JWKS: {}", e))
            ))?;
        
        // Cache the keys
        let mut cache = self.cache.write().await;
        for key in jwks.keys {
            if let Ok(decoding_key) = self.create_decoding_key(&key) {
                cache.insert(key.kid, decoding_key);
            }
        }
        
        Ok(())
    }
    
    fn create_decoding_key(&self, jwk: &Jwk) -> Result<DecodingKey, ApiError> {
        match jwk.kty.as_str() {
            "RSA" => {
                let n = base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(&jwk.n)
                    .map_err(|e| ApiError::Internal(crate::model::error::InternalError::Serialization(format!("Invalid n: {}", e))))?;
                let e = base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(&jwk.e)
                    .map_err(|e| ApiError::Internal(crate::model::error::InternalError::Serialization(format!("Invalid e: {}", e))))?;
                
                // Create RSA public key from components
                let n_str = base64::engine::general_purpose::STANDARD.encode(&n);
                let e_str = base64::engine::general_purpose::STANDARD.encode(&e);
                Ok(DecodingKey::from_rsa_components(&n_str, &e_str)
                    .map_err(|e| ApiError::Internal(crate::model::error::InternalError::Serialization(format!("Invalid RSA key: {}", e))))?)
            }
            _ => Err(ApiError::Authentication(AuthenticationError::InvalidApiKey))
        }
    }
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kty: String,
    kid: String,
    n: String,
    e: String,
    alg: String,
}

/// JWT Authentication middleware
#[handler]
pub async fn jwt_auth_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // Skip authentication for public endpoints
    if is_public_endpoint(req.uri().path()) {
        depot.insert("jwt_auth_context", JwtAuthContext::anonymous());
        ctrl.call_next(req, depot, res).await;
        return;
    }
    
    // Extract token from Authorization header
    let token = extract_bearer_token(req);
    
    if token.is_empty() {
        let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
            AuthenticationError::MissingToken
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }
    
    // Get Keycloak config from depot
    let keycloak_config = match depot.get::<KeycloakConfig>("keycloak_config") {
        Ok(config) => config,
        Err(_) => {
            warn!("KeycloakConfig not found in depot");
            let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
                AuthenticationError::InvalidApiKey
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };
    
    // Validate JWT token
    match validate_jwt_token(&keycloak_config, &token).await {
        Ok(auth_context) => {
            depot.insert("jwt_auth_context", auth_context);
            ctrl.call_next(req, depot, res).await;
        }
        Err(error) => {
            let error_response = ErrorResponse::from_api_error(&error);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// RPT (Requesting Party Token) validation middleware
#[handler]
pub async fn rpt_auth_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // Extract RPT token from Authorization header
    let token = extract_bearer_token(req);
    
    if token.is_empty() {
        let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
            AuthenticationError::MissingToken
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }
    
    // Get Keycloak config from depot
    let keycloak_config = match depot.get::<KeycloakConfig>("keycloak_config") {
        Ok(config) => config,
        Err(_) => {
            warn!("KeycloakConfig not found in depot");
            let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
                AuthenticationError::InvalidApiKey
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };
    
    // Validate RPT token
    match validate_rpt_token(&keycloak_config, &token).await {
        Ok(auth_context) => {
            depot.insert("jwt_auth_context", auth_context);
            ctrl.call_next(req, depot, res).await;
        }
        Err(error) => {
            let error_response = ErrorResponse::from_api_error(&error);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// JWT Authorization middleware
#[handler]
pub async fn jwt_authorization_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // Get auth context
    let auth_context = match depot.get::<JwtAuthContext>("jwt_auth_context") {
        Ok(context) => context,
        Err(_) => {
            warn!("JwtAuthContext not found in depot");
            let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
                AuthenticationError::MissingToken
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };
    
    // Check if user is authenticated
    if !auth_context.is_authenticated {
        let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
            AuthenticationError::InvalidApiKey
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }
    
    // Check permissions based on endpoint
    let required_permission = get_required_permission(req.uri().path());
    if let Some(permission) = required_permission {
        if !check_permission(&auth_context, &permission) {
            let error_response = ErrorResponse::from_api_error(&ApiError::Authentication(
                AuthenticationError::InsufficientPermissions(
                    format!("Required permission: {}", permission)
                )
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    }
    
    ctrl.call_next(req, depot, res).await;
}

// Helper functions

fn extract_bearer_token(req: &Request) -> String {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return auth_str[7..].to_string();
            }
        }
    }
    String::new()
}

async fn validate_jwt_token(config: &KeycloakConfig, token: &str) -> Result<JwtAuthContext, ApiError> {
    // Decode JWT header to get key ID
    let header = jsonwebtoken::decode_header(token)
        .map_err(|_| ApiError::Authentication(AuthenticationError::InvalidApiKey))?;
    
    let kid = header.kid.ok_or_else(|| ApiError::Authentication(AuthenticationError::InvalidApiKey))?;
    
    // Get JWKS cache from depot (this would need to be passed differently in real implementation)
    // For now, we'll create a temporary cache
    let jwks_cache = JwksCache::new(config.jwks_endpoint.clone());
    let decoding_key = jwks_cache.get_key(&kid).await?
        .ok_or_else(|| ApiError::Authentication(AuthenticationError::InvalidApiKey))?;
    
    // Validate token
    let validation = Validation::new(Algorithm::RS256);
    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| {
            error!("JWT validation failed: {}", e);
            ApiError::Authentication(AuthenticationError::InvalidApiKey)
        })?;
    
    // Check expiration
    let now = chrono::Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err(ApiError::Authentication(AuthenticationError::ExpiredToken));
    }
    
    // Check issuer
    if token_data.claims.iss != format!("{}/realms/{}", config.base_url, config.realm) {
        return Err(ApiError::Authentication(AuthenticationError::InvalidApiKey));
    }
    
    Ok(JwtAuthContext::new(token_data.claims, TokenType::AccessToken))
}

async fn validate_rpt_token(config: &KeycloakConfig, token: &str) -> Result<JwtAuthContext, ApiError> {
    // RPT tokens are typically opaque tokens that need introspection
    let client = Client::new();
    
    let mut form = HashMap::new();
    form.insert("token", token);
    form.insert("token_type_hint", "requesting_party_token");
    
    let response = client
        .post(&config.introspection_endpoint)
        .form(&form)
        .basic_auth(&config.client_id, config.client_secret.as_deref())
        .send()
        .await
        .map_err(|e| ApiError::ExternalService(
            crate::model::error::ExternalServiceError::Unavailable(format!("Introspection failed: {}", e))
        ))?;
    
    if !response.status().is_success() {
        return Err(ApiError::Authentication(AuthenticationError::InvalidApiKey));
    }
    
    let introspection: TokenIntrospection = response.json().await
        .map_err(|e| ApiError::ExternalService(
            crate::model::error::ExternalServiceError::Unavailable(format!("Failed to parse introspection: {}", e))
        ))?;
    
    if !introspection.active {
        return Err(ApiError::Authentication(AuthenticationError::InvalidApiKey));
    }
    
    // Convert introspection response to auth context
    let auth_context = JwtAuthContext {
        user_id: introspection.sub.unwrap_or_default(),
        username: introspection.preferred_username,
        email: introspection.email,
        name: introspection.name,
        realm_roles: introspection.realm_access
            .map(|ra| ra.roles)
            .unwrap_or_default(),
        resource_roles: introspection.resource_access
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v.roles))
            .collect(),
        scope: introspection.scope,
        is_authenticated: true,
        token_type: TokenType::RptToken,
    };
    
    Ok(auth_context)
}

#[derive(Debug, Deserialize)]
struct TokenIntrospection {
    active: bool,
    sub: Option<String>,
    preferred_username: Option<String>,
    email: Option<String>,
    name: Option<String>,
    realm_access: Option<RealmAccess>,
    resource_access: Option<HashMap<String, ResourceAccess>>,
    scope: Option<String>,
}

fn is_public_endpoint(path: &str) -> bool {
    let public_paths = [
        "/health",
        "/metrics",
        "/api-doc",
        "/swagger-ui",
        "/public",
    ];
    
    public_paths.iter().any(|&public_path| path.starts_with(public_path))
}

fn get_required_permission(path: &str) -> Option<String> {
    if path.contains("/routes") {
        Some("routes:read".to_string())
    } else if path.contains("/certificates") {
        Some("certificates:read".to_string())
    } else if path.contains("/user-settings") {
        Some("user-settings:read".to_string())
    } else {
        None
    }
}

fn check_permission(auth_context: &JwtAuthContext, permission: &str) -> bool {
    // Check realm roles
    if auth_context.has_realm_role("admin") {
        return true;
    }
    
    // Check specific permissions
    match permission {
        "routes:read" => auth_context.has_realm_role("user") || auth_context.has_scope("routes:read"),
        "routes:write" => auth_context.has_realm_role("user") || auth_context.has_scope("routes:write"),
        "certificates:read" => auth_context.has_realm_role("user") || auth_context.has_scope("certificates:read"),
        "certificates:write" => auth_context.has_realm_role("user") || auth_context.has_scope("certificates:write"),
        "user-settings:read" => auth_context.has_realm_role("user") || auth_context.has_scope("user-settings:read"),
        "user-settings:write" => auth_context.has_realm_role("user") || auth_context.has_scope("user-settings:write"),
        _ => false,
    }
}
