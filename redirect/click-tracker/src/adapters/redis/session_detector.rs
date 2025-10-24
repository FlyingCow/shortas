use std::net::IpAddr;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use redis::aio::ConnectionManager;
use redis::Client;
use redis::Script;
use tokio::time::timeout;
use tracing::info;

use crate::core::session::{Session, SessionDetector};

use super::settings::Redis;

const EXPIRATION_OFFSET: i64 = 30 * 60;
const REDIS_TIMEOUT_SECS: u64 = 5;  // 5 second timeout for Redis operations

#[derive(Clone)]
pub struct RedisSessionDetector {
    connection_manager: ConnectionManager,
}

impl RedisSessionDetector {
    pub async fn new(settings: &Redis) -> Self {
        info!("  redis -> {}", &settings.host);

        let client = Client::open(settings.host.as_str())
            .expect("Failed to parse Redis connection string");

        info!("  redis -> Establishing connection manager...");

        // Use async ConnectionManager for connection pooling and reuse
        // ConnectionManager will automatically reconnect on connection loss
        let connection_manager = ConnectionManager::new(client)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to create Redis connection manager. Is Redis running at {}? Error: {}",
                    settings.host, e
                )
            });

        info!("  redis -> Connection manager established successfully");

        Self {
            connection_manager,
        }
    }
}

#[async_trait::async_trait]
impl SessionDetector for RedisSessionDetector {
    async fn detect(
        &self,
        route_id: &str,
        ip_addr: &IpAddr,
        click_time: &DateTime<Utc>,
    ) -> Result<Session> {
        let click_timestamp = click_time.timestamp_millis();

        let root_key = format!("sessions:{}:{}", route_id, ip_addr);

        let script_value = r#"
            local current = redis.call('GET', KEYS[1]) or 'none'
            local expiry = tonumber(ARGV[2])
            
            if current == 'none' then
                local json = cjson.decode('{}')

                json['first'] = tonumber(ARGV[1])
                json['last'] = tonumber(ARGV[1])
                json['count'] = 1

                local json_str = cjson.encode(json)
                redis.call('SET', KEYS[1], json_str)
                redis.call('EXPIRE', KEYS[1], expiry)

                return json_str
            else
                local json = cjson.decode(current)

                json['last'] = tonumber(ARGV[1])
                json['count'] = json['count'] + 1

                local json_str = cjson.encode(json)
                redis.call('SET', KEYS[1], json_str)
                redis.call('EXPIRE', KEYS[1], expiry)

                return json_str
            end
            "#;

        let script = Script::new(script_value);

        // Clone the connection manager (lightweight, uses Arc internally)
        let mut connection = self.connection_manager.clone();

        // Use async invoke with timeout to prevent hanging
        let result = timeout(
            Duration::from_secs(REDIS_TIMEOUT_SECS),
            script
                .key(root_key)
                .arg(click_timestamp)
                .arg(EXPIRATION_OFFSET)
                .invoke_async::<String>(&mut connection),
        )
        .await
        .context("Redis session lookup timed out")?
        .context("Redis session lookup failed")?;

        let session: Session = serde_json::from_str(&result)?;

        Ok(session)
    }
}
