//! Rate limiting middleware.

use async_trait::async_trait;
use salvo::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::warn;

use super::auth::UserExt;

/// Rate limit configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window.
    pub max_requests: u32,
    /// Window duration in seconds.
    pub window_seconds: u64,
    /// Burst capacity (initial tokens).
    pub burst: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 60,
            burst: 20,
        }
    }
}

/// Token bucket for rate limiting.
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_update: Instant,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
}

impl TokenBucket {
    fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            tokens: max_tokens,
            last_update: Instant::now(),
            max_tokens,
            refill_rate,
        }
    }

    fn try_consume(&mut self) -> bool {
        self.refill();

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_update = now;
    }

    fn remaining(&self) -> u32 {
        self.tokens as u32
    }
}

/// Rate limiter middleware using token bucket algorithm.
pub struct RateLimiter {
    config: RateLimitConfig,
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
}

impl RateLimiter {
    /// Create a new rate limiter with default configuration.
    pub fn new() -> Self {
        Self::with_config(RateLimitConfig::default())
    }

    /// Create a new rate limiter with custom configuration.
    pub fn with_config(config: RateLimitConfig) -> Self {
        // Start cleanup task
        let buckets: Arc<RwLock<HashMap<String, TokenBucket>>> =
            Arc::new(RwLock::new(HashMap::new()));

        let cleanup_buckets = buckets.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                let mut map = cleanup_buckets.write().await;
                let now = Instant::now();
                map.retain(|_, bucket| {
                    now.duration_since(bucket.last_update) < Duration::from_secs(600)
                });
            }
        });

        Self { config, buckets }
    }

    /// Get or create a bucket for the given key.
    async fn get_bucket(&self, key: &str) -> bool {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            let max_tokens = self.config.burst as f64;
            let refill_rate = self.config.max_requests as f64 / self.config.window_seconds as f64;
            TokenBucket::new(max_tokens, refill_rate)
        });

        bucket.try_consume()
    }

    /// Get remaining tokens for a key.
    async fn get_remaining(&self, key: &str) -> u32 {
        let buckets = self.buckets.read().await;
        buckets
            .get(key)
            .map(|b| b.remaining())
            .unwrap_or(self.config.burst)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Handler for RateLimiter {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // Get rate limit key (user ID or IP)
        let key = match depot.user_id() {
            Ok(id) => id,
            Err(_) => format!("{:?}", req.remote_addr()),
        };

        // Check rate limit
        if !self.get_bucket(&key).await {
            warn!("Rate limit exceeded for: {}", key);

            let remaining = self.get_remaining(&key).await;

            res.status_code(StatusCode::TOO_MANY_REQUESTS);
            res.headers_mut().insert(
                "X-RateLimit-Limit",
                self.config.max_requests.to_string().parse().unwrap(),
            );
            res.headers_mut().insert(
                "X-RateLimit-Remaining",
                remaining.to_string().parse().unwrap(),
            );
            res.headers_mut().insert(
                "Retry-After",
                "60".parse().unwrap(),
            );
            res.render(Json(serde_json::json!({
                "code": "RATE_LIMIT_EXCEEDED",
                "message": "Too many requests. Please try again later."
            })));
            ctrl.skip_rest();
            return;
        }

        // Add rate limit headers
        let remaining = self.get_remaining(&key).await;
        res.headers_mut().insert(
            "X-RateLimit-Limit",
            self.config.max_requests.to_string().parse().unwrap(),
        );
        res.headers_mut().insert(
            "X-RateLimit-Remaining",
            remaining.to_string().parse().unwrap(),
        );

        ctrl.call_next(req, depot, res).await;
    }
}
