//! JWT Configuration and Keycloak integration
//! 
//! This module provides configuration management for JWT authentication
//! and Keycloak integration.

use serde::{Deserialize, Serialize};
use std::env;

/// JWT Configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub keycloak_base_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub audience: Option<String>,
    pub issuer: String,
    pub jwks_endpoint: String,
    pub introspection_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let keycloak_base_url = env::var("KEYCLOAK_BASE_URL")
            .unwrap_or_else(|_| "http://keycloak:8080".to_string());
        
        let realm = env::var("KEYCLOAK_REALM")
            .unwrap_or_else(|_| "master".to_string());
        
        let client_id = env::var("KEYCLOAK_CLIENT_ID")
            .unwrap_or_else(|_| "click-aggregator-api".to_string());
        
        let client_secret = env::var("KEYCLOAK_CLIENT_SECRET").ok();
        
        let audience = env::var("KEYCLOAK_AUDIENCE").ok();
        
        let issuer = format!("{}/realms/{}", keycloak_base_url, realm);
        let jwks_endpoint = format!("{}/realms/{}/protocol/openid-connect/certs", keycloak_base_url, realm);
        let introspection_endpoint = format!("{}/realms/{}/protocol/openid-connect/token/introspect", keycloak_base_url, realm);
        let token_endpoint = format!("{}/realms/{}/protocol/openid-connect/token", keycloak_base_url, realm);
        let userinfo_endpoint = format!("{}/realms/{}/protocol/openid-connect/userinfo", keycloak_base_url, realm);
        
        Self {
            keycloak_base_url,
            realm,
            client_id,
            client_secret,
            audience,
            issuer,
            jwks_endpoint,
            introspection_endpoint,
            token_endpoint,
            userinfo_endpoint,
        }
    }
    
    pub fn with_custom_config(
        keycloak_base_url: String,
        realm: String,
        client_id: String,
        client_secret: Option<String>,
        audience: Option<String>,
    ) -> Self {
        let issuer = format!("{}/realms/{}", keycloak_base_url, realm);
        let jwks_endpoint = format!("{}/realms/{}/protocol/openid-connect/certs", keycloak_base_url, realm);
        let introspection_endpoint = format!("{}/realms/{}/protocol/openid-connect/token/introspect", keycloak_base_url, realm);
        let token_endpoint = format!("{}/realms/{}/protocol/openid-connect/token", keycloak_base_url, realm);
        let userinfo_endpoint = format!("{}/realms/{}/protocol/openid-connect/userinfo", keycloak_base_url, realm);
        
        Self {
            keycloak_base_url,
            realm,
            client_id,
            client_secret,
            audience,
            issuer,
            jwks_endpoint,
            introspection_endpoint,
            token_endpoint,
            userinfo_endpoint,
        }
    }
}

/// Permission mapping for JWT claims
#[derive(Debug, Clone)]
pub struct PermissionMapper {
    pub role_mappings: std::collections::HashMap<String, Vec<String>>,
    pub scope_mappings: std::collections::HashMap<String, Vec<String>>,
}

impl PermissionMapper {
    pub fn new() -> Self {
        let mut role_mappings = std::collections::HashMap::new();
        let mut scope_mappings = std::collections::HashMap::new();
        
        // Map Keycloak roles to API permissions
        role_mappings.insert("admin".to_string(), vec![
            "routes:read".to_string(),
            "routes:write".to_string(),
            "routes:delete".to_string(),
            "certificates:read".to_string(),
            "certificates:write".to_string(),
            "certificates:delete".to_string(),
            "user-settings:read".to_string(),
            "user-settings:write".to_string(),
        ]);
        
        role_mappings.insert("user".to_string(), vec![
            "routes:read".to_string(),
            "certificates:read".to_string(),
            "user-settings:read".to_string(),
        ]);
        
        role_mappings.insert("api-user".to_string(), vec![
            "routes:read".to_string(),
            "routes:write".to_string(),
            "certificates:read".to_string(),
        ]);
        
        // Map OAuth scopes to API permissions
        scope_mappings.insert("routes:read".to_string(), vec!["routes:read".to_string()]);
        scope_mappings.insert("routes:write".to_string(), vec!["routes:write".to_string()]);
        scope_mappings.insert("routes:delete".to_string(), vec!["routes:delete".to_string()]);
        scope_mappings.insert("certificates:read".to_string(), vec!["certificates:read".to_string()]);
        scope_mappings.insert("certificates:write".to_string(), vec!["certificates:write".to_string()]);
        scope_mappings.insert("certificates:delete".to_string(), vec!["certificates:delete".to_string()]);
        scope_mappings.insert("user-settings:read".to_string(), vec!["user-settings:read".to_string()]);
        scope_mappings.insert("user-settings:write".to_string(), vec!["user-settings:write".to_string()]);
        
        Self {
            role_mappings,
            scope_mappings,
        }
    }
    
    pub fn get_permissions_for_roles(&self, roles: &[String]) -> Vec<String> {
        let mut permissions = Vec::new();
        
        for role in roles {
            if let Some(role_permissions) = self.role_mappings.get(role) {
                permissions.extend(role_permissions.clone());
            }
        }
        
        permissions.sort();
        permissions.dedup();
        permissions
    }
    
    pub fn get_permissions_for_scopes(&self, scopes: &[String]) -> Vec<String> {
        let mut permissions = Vec::new();
        
        for scope in scopes {
            if let Some(scope_permissions) = self.scope_mappings.get(scope) {
                permissions.extend(scope_permissions.clone());
            }
        }
        
        permissions.sort();
        permissions.dedup();
        permissions
    }
}

/// JWT Token validation settings
#[derive(Debug, Clone)]
pub struct TokenValidationConfig {
    pub validate_issuer: bool,
    pub validate_audience: bool,
    pub validate_expiration: bool,
    pub clock_skew_seconds: u64,
    pub require_scope: bool,
    pub allowed_algorithms: Vec<jsonwebtoken::Algorithm>,
}

impl Default for TokenValidationConfig {
    fn default() -> Self {
        Self {
            validate_issuer: true,
            validate_audience: true,
            validate_expiration: true,
            clock_skew_seconds: 60,
            require_scope: false,
            allowed_algorithms: vec![
                jsonwebtoken::Algorithm::RS256,
                jsonwebtoken::Algorithm::RS384,
                jsonwebtoken::Algorithm::RS512,
            ],
        }
    }
}

/// RPT Token configuration
#[derive(Debug, Clone)]
pub struct RptConfig {
    pub enabled: bool,
    pub introspection_timeout_seconds: u64,
    pub cache_ttl_seconds: u64,
    pub require_uma_scope: bool,
}

impl Default for RptConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            introspection_timeout_seconds: 30,
            cache_ttl_seconds: 300, // 5 minutes
            require_uma_scope: true,
        }
    }
}

