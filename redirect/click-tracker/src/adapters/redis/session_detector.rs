use std::net::IpAddr;

use anyhow::Result;
use chrono::{DateTime, Utc};
use redis::Client;
use redis::Script;
use tracing::info;

use crate::core::session::{Session, SessionDetector};

use super::settings::Redis;

const EXPIRATION_OFFSET: i64 = 30 * 60;

#[derive(Clone)]
pub struct RedisSessionDetector {
    redis_client: Client,
}

impl RedisSessionDetector {
    pub fn new(settings: &Redis) -> Self {
        info!("  redis -> {}", &settings.host);

        let client = Client::open(settings.host.as_str()).unwrap();

        let _con = client.get_connection().unwrap();

        Self {
            redis_client: client,
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

        let mut connection = self.redis_client.get_connection()?;

        let result: Result<Option<String>, redis::RedisError> = script
            .key(root_key)
            .arg(click_timestamp)
            .arg(EXPIRATION_OFFSET)
            .invoke(&mut connection);

        let session: Session = serde_json::from_str(&result?.unwrap())?;

        Ok(session)
    }
}
