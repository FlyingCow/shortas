//! Token buckets in memory, so limits are per router instance.

use std::{
    collections::HashMap,
    hash::Hash,
    sync::Mutex,
    time::{Duration, Instant},
};

use serde_derive::Deserialize;

/// Buckets kept at once. Reaching it resets the limits.
const MAX_BUCKETS: usize = 100_000;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rate {
    pub per_second: f64,
    pub burst: f64,
}

struct Bucket {
    tokens: f64,
    updated: Instant,
}

pub struct RateLimiter<K> {
    rate: Rate,
    buckets: Mutex<HashMap<K, Bucket>>,
}

impl<K: Eq + Hash> RateLimiter<K> {
    pub fn new(rate: Rate) -> Self {
        Self {
            rate,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Takes a token, or says how long until there is one.
    pub fn take(&self, key: K) -> Result<(), Duration> {
        self.take_at(key, Instant::now())
    }

    /// Like `take`, but leaves the token.
    pub fn check(&self, key: &K) -> Result<(), Duration> {
        let Rate { per_second, burst } = self.rate;
        let buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
        let Some(bucket) = buckets.get(key) else {
            return Ok(());
        };
        let tokens = (bucket.tokens + bucket.updated.elapsed().as_secs_f64() * per_second).min(burst);
        if tokens >= 1.0 {
            return Ok(());
        }
        Err(Duration::from_secs_f64((1.0 - tokens) / per_second.max(0.001)))
    }

    fn take_at(&self, key: K, now: Instant) -> Result<(), Duration> {
        let Rate { per_second, burst } = self.rate;
        let refilled = |b: &Bucket| {
            let elapsed = now.saturating_duration_since(b.updated).as_secs_f64();
            (b.tokens + elapsed * per_second).min(burst)
        };

        let mut buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
        if buckets.len() >= MAX_BUCKETS && !buckets.contains_key(&key) {
            buckets.retain(|_, b| refilled(b) < burst);
            if buckets.len() >= MAX_BUCKETS {
                buckets.clear();
            }
        }

        let bucket = buckets.entry(key).or_insert(Bucket {
            tokens: burst,
            updated: now,
        });
        bucket.tokens = refilled(bucket);
        bucket.updated = now;
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            return Ok(());
        }
        Err(Duration::from_secs_f64((1.0 - bucket.tokens) / per_second.max(0.001)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_the_burst_then_the_rate() {
        let limiter = RateLimiter::new(Rate { per_second: 2.0, burst: 3.0 });
        let start = Instant::now();
        for _ in 0..3 {
            assert!(limiter.take_at("a", start).is_ok());
        }
        assert_eq!(limiter.take_at("a", start), Err(Duration::from_millis(500)));
        assert!(limiter.take_at("b", start).is_ok());
        let later = start + Duration::from_millis(500);
        assert!(limiter.take_at("a", later).is_ok());
        assert!(limiter.take_at("a", later).is_err());
    }

    #[test]
    fn memory_is_bounded() {
        let limiter = RateLimiter::new(Rate { per_second: 1.0, burst: 1.0 });
        let now = Instant::now();
        for i in 0..MAX_BUCKETS * 2 + 5 {
            let _ = limiter.take_at(i, now);
        }
        assert!(limiter.buckets.lock().unwrap().len() <= MAX_BUCKETS);
    }
}
