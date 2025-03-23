pub mod aws;
pub mod fluvio;
pub mod geo_ip;
pub mod kafka;
pub mod moka;
pub mod redis;
pub mod uaparser;

use anyhow::Result;
use aws::user_settings_store::DynamoUserSettingsStore;
use chrono::{DateTime, Utc};
use geo_ip::geo_ip_location_detector::GeoIPLocationDetector;
use moka::user_settings_store::MokaDecoratedUserSettingsStore;
use redis::session_detector::RedisSessionDetector;
use std::{net::IpAddr, sync::mpsc::SyncSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uaparser::user_agent_detector::UAParserUserAgentDetector;

use crate::{
    FluvioHitStream, KafkaHitStream,
    core::{
        Country, Hit, HitStreamSource, UserAgent, UserAgentDetector, UserSettingsStore,
        location::LocationDetector,
        session::{Session, SessionDetector},
    },
};

pub enum HitStreamSourceType {
    Kafka(KafkaHitStream),
    Fluvio(FluvioHitStream),
}

#[async_trait::async_trait]
impl HitStreamSource for HitStreamSourceType {
    async fn pull(&self, ts: SyncSender<Hit>, token: CancellationToken) -> Result<JoinHandle<()>> {
        match self {
            HitStreamSourceType::Kafka(stream) => stream.pull(ts, token).await,
            HitStreamSourceType::Fluvio(stream) => stream.pull(ts, token).await,
        }
    }
}

pub enum UserSettingsStoreType<S>
where
    S: UserSettingsStore + Send + Sync,
{
    //Redis,
    Dynamo(DynamoUserSettingsStore),
    Moka(MokaDecoratedUserSettingsStore<S>),
}

#[async_trait::async_trait]
impl<S> UserSettingsStore for UserSettingsStoreType<S>
where
    S: UserSettingsStore + Send + Sync,
{
    async fn get(&self, user_id: &str) -> Result<Option<crate::core::UserSettings>> {
        match self {
            UserSettingsStoreType::Dynamo(store) => store.get(user_id).await,
            UserSettingsStoreType::Moka(store) => store.get(user_id).await,
        }
    }

    async fn invalidate(&self, user_id: &str) -> Result<()> {
        match self {
            UserSettingsStoreType::Dynamo(store) => store.invalidate(user_id).await,
            UserSettingsStoreType::Moka(store) => store.invalidate(user_id).await,
        }
    }
}

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
