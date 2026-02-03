pub mod aws;
pub mod fluvio;
pub mod geo_ip;
pub mod kafka;
pub mod moka;
pub mod mongodb;
pub mod redis;
pub mod uaparser;

use anyhow::Result;
use aws::user_settings_store::DynamoUserSettingsStore;
use chrono::{DateTime, Utc};
use flume::Sender;
use fluvio::FluvioClickAggsRegistrar;
use geo_ip::geo_ip_location_detector::GeoIPLocationDetector;
use kafka::KafkaClickAggsRegistrar;
use moka::user_settings_cache::MokaUserSettingsCache;
use redis::session_detector::RedisSessionDetector;
use std::net::IpAddr;
use tokio_util::sync::CancellationToken;
use uaparser::user_agent_detector::UAParserUserAgentDetector;

use crate::{
    FluvioHitStream, KafkaHitStream,
    adapters::mongodb::user_settings_store::MongodbUserSettingsStore,
    core::{
        ClickStreamItem, Country, Hit, HitStreamSource, UserAgent, UserAgentDetector,
        UserSettingsCache, UserSettingsStore,
        aggs::ClickAggsRegistrar,
        location::LocationDetector,
        session::{Session, SessionDetector},
    },
};

#[derive(Clone)]
pub enum LocationDetectorType {
    GeoIP(GeoIPLocationDetector),
}

impl LocationDetector for LocationDetectorType {
    fn detect_country(&self, ip_addr: &IpAddr) -> Option<Country> {
        match self {
            LocationDetectorType::GeoIP(detector) => detector.detect_country(ip_addr),
        }
    }
}

#[derive(Clone)]
pub enum ClickAggsRegistrarType {
    Kafka(KafkaClickAggsRegistrar),
    Fluvio(FluvioClickAggsRegistrar),
}

#[async_trait::async_trait]
impl ClickAggsRegistrar for ClickAggsRegistrarType {
    async fn register(&self, click: ClickStreamItem) -> Result<()> {
        match self {
            ClickAggsRegistrarType::Kafka(registrar) => registrar.register(click).await,
            ClickAggsRegistrarType::Fluvio(registrar) => registrar.register(click).await,
        }
    }
}

pub enum HitStreamSourceType {
    Kafka(KafkaHitStream),
    Fluvio(FluvioHitStream),
}

#[async_trait::async_trait]
impl HitStreamSource for HitStreamSourceType {
    async fn pull(&self, ts: Sender<Hit>, token: CancellationToken) -> Result<()> {
        match self {
            HitStreamSourceType::Kafka(stream) => stream.pull(ts, token).await,
            HitStreamSourceType::Fluvio(stream) => stream.pull(ts, token).await,
        }
    }
}

#[derive(Clone)]
pub enum UserSettingsStoreType {
    Mongodb(MongodbUserSettingsStore),
    Dynamo(DynamoUserSettingsStore),
}

#[async_trait::async_trait]
impl UserSettingsStore for UserSettingsStoreType {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<crate::core::UserSettings>> {
        match self {
            UserSettingsStoreType::Mongodb(store) => store.get_user_settings(user_id).await,
            UserSettingsStoreType::Dynamo(store) => store.get_user_settings(user_id).await,
        }
    }
}

#[derive(Clone)]
pub enum UserSettingsCacheType {
    Moka(MokaUserSettingsCache),
}

#[async_trait::async_trait]
impl UserSettingsCache for UserSettingsCacheType {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<crate::core::UserSettings>> {
        match self {
            UserSettingsCacheType::Moka(cache) => cache.get_user_settings(user_id).await,
        }
    }

    async fn invalidate(&self, user_id: &str) -> Result<()> {
        match self {
            UserSettingsCacheType::Moka(cache) => cache.invalidate(user_id).await,
        }
    }
}

#[derive(Clone)]
pub enum SessionDetectorType {
    Redis(RedisSessionDetector),
}

#[async_trait::async_trait]
impl SessionDetector for SessionDetectorType {
    async fn detect(
        &self,
        route_id: &str,
        ip_addr: &IpAddr,
        click_time: &DateTime<Utc>,
    ) -> Result<Session> {
        match self {
            SessionDetectorType::Redis(detector) => {
                detector.detect(route_id, ip_addr, click_time).await
            }
        }
    }
}

#[derive(Clone)]
pub enum UserAgentDetectorType {
    //Udger,
    UAParser(UAParserUserAgentDetector),
}

impl UserAgentDetector for UserAgentDetectorType {
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent {
        match self {
            UserAgentDetectorType::UAParser(detector) => detector.parse_user_agent(user_agent),
        }
    }

    fn parse_device(&self, user_agent: &str) -> crate::core::Device {
        match self {
            UserAgentDetectorType::UAParser(detector) => detector.parse_device(user_agent),
        }
    }

    fn parse_os(&self, user_agent: &str) -> crate::core::OS {
        match self {
            UserAgentDetectorType::UAParser(detector) => detector.parse_os(user_agent),
        }
    }
}
