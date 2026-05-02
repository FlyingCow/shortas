//! JWT authentication middleware for Keycloak integration.

use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, DecodingKey, TokenData, Validation};
use reqwest::Client;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use crate::domain::entities::UserProfile;
use crate::settings::JwtSettings;

/// JWT claims from Keycloak.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub preferred_username: Option<String>,
    pub email_verified: Option<bool>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<serde_json::Value>,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

impl From<JwtClaims> for UserProfile {
    fn from(claims: JwtClaims) -> Self {
        let roles = claims
            .realm_access
            .map(|ra| ra.roles)
            .unwrap_or_default();

        Self {
            id: claims.sub,
            email: claims.email,
            name: claims.name,
            preferred_username: claims.preferred_username,
            email_verified: claims.email_verified.unwrap_or(false),
            roles,
        }
    }
}

/// JWKS cache with automatic refresh.
struct JwksCache {
    jwks: JwkSet,
    fetched_at: Instant,
    ttl: Duration,
}

impl JwksCache {
    fn is_expired(&self) -> bool {
        self.fetched_at.elapsed() > self.ttl
    }
}

/// JWT authentication handler.
pub struct JwtAuth {
    settings: JwtSettings,
    http_client: Client,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

impl JwtAuth {
    /// Create a new JWT authentication handler.
    pub fn new(settings: JwtSettings) -> Self {
        Self {
            settings,
            http_client: Client::new(),
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Fetch JWKS from Keycloak.
    async fn fetch_jwks(&self) -> anyhow::Result<JwkSet> {
        let response = self
            .http_client
            .get(&self.settings.jwks_url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        let jwks: JwkSet = response.json().await?;
        Ok(jwks)
    }

    /// Get JWKS with caching.
    async fn get_jwks(&self) -> anyhow::Result<JwkSet> {
        // Check cache first
        {
            let cache = self.jwks_cache.read().await;
            if let Some(ref cached) = *cache {
                if !cached.is_expired() {
                    return Ok(cached.jwks.clone());
                }
            }
        }

        // Fetch fresh JWKS
        let jwks = self.fetch_jwks().await?;

        // Update cache
        {
            let mut cache = self.jwks_cache.write().await;
            *cache = Some(JwksCache {
                jwks: jwks.clone(),
                fetched_at: Instant::now(),
                ttl: Duration::from_secs(300), // 5 minutes
            });
        }

        Ok(jwks)
    }

    /// Validate and decode a JWT token.
    async fn validate_token(&self, token: &str) -> anyhow::Result<TokenData<JwtClaims>> {
        // Decode header to get kid
        let header = decode_header(token)?;
        let kid = header
            .kid
            .ok_or_else(|| anyhow::anyhow!("Token missing kid"))?;

        // Get JWKS and find matching key
        let jwks = self.get_jwks().await?;
        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| anyhow::anyhow!("Key not found in JWKS"))?;

        // Create decoding key
        let decoding_key = DecodingKey::from_jwk(jwk)?;

        // Configure validation
        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[&self.settings.issuer]);
        validation.set_audience(&[&self.settings.audience]);

        // Decode and validate
        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)?;

        Ok(token_data)
    }
}

#[async_trait]
impl Handler for JwtAuth {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // Extract token from Authorization header
        let auth_header = req.headers().get("Authorization");

        let token = match auth_header {
            Some(value) => {
                let value_str = value.to_str().unwrap_or("");
                if value_str.starts_with("Bearer ") {
                    &value_str[7..]
                } else {
                    warn!("Invalid Authorization header format");
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(Json(serde_json::json!({
                        "code": "UNAUTHORIZED",
                        "message": "Invalid Authorization header format"
                    })));
                    ctrl.skip_rest();
                    return;
                }
            }
            None => {
                debug!("Missing Authorization header");
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(serde_json::json!({
                    "code": "UNAUTHORIZED",
                    "message": "Authorization header required"
                })));
                ctrl.skip_rest();
                return;
            }
        };

        // Validate token
        match self.validate_token(token).await {
            Ok(token_data) => {
                let user_profile: UserProfile = token_data.claims.into();
                debug!("Authenticated user: {}", user_profile.id);
                depot.inject(user_profile);
                ctrl.call_next(req, depot, res).await;
            }
            Err(e) => {
                error!("Token validation failed: {}", e);
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(serde_json::json!({
                    "code": "UNAUTHORIZED",
                    "message": "Invalid or expired token"
                })));
                ctrl.skip_rest();
            }
        }
    }
}

/// Extension trait for getting user profile from depot.
pub trait UserExt {
    fn user_profile(&self) -> anyhow::Result<&UserProfile>;
    fn user_id(&self) -> anyhow::Result<String>;
}

impl UserExt for Depot {
    fn user_profile(&self) -> anyhow::Result<&UserProfile> {
        self.obtain::<UserProfile>()
            .map_err(|_| anyhow::anyhow!("User profile not found in depot"))
    }

    fn user_id(&self) -> anyhow::Result<String> {
        Ok(self.user_profile()?.id.clone())
    }
}
