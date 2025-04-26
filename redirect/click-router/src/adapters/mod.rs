use std::net::IpAddr;

use anyhow::Result;
use aws::dynamo::{
    crypto_store::DynamoCryptoStore, routes_store::DynamoRoutesStore,
    user_settings_store::DynamoUserSettingsStore,
};
use fluvio::hit_registrar::FluvioHitRegistrar;
use geo_ip::geo_ip_location_detector::GeoIPLocationDetector;
use moka::{
    crypto_cache::MokaCryptoCache, routes_cache::MokaRoutesCache,
    user_settings_cache::MokaUserSettingsCache,
};
use rdkafka::hit_registrar::KafkaHitRegistrar;
use uaparser::user_agent_detector::UAParserUserAgentDetector;

use crate::{
    core::{
        crypto::CryptoCache,
        hits_register::HitRegistrar,
        location::{Country, LocationDetector},
        routes::RoutesCache,
        user_agent::{Device, UserAgent, UserAgentDetector, OS},
        user_settings::UserSettingsCache,
        CryptoStore, RoutesStore, UserSettingsStore,
    },
    model::{Hit, Keycert, Route, UserSettings},
};

pub mod aws;
pub mod fluvio;
pub mod geo_ip;
pub mod kafka;
pub mod moka;
pub mod rdkafka;
pub mod uaparser;

#[derive(Clone)]
pub enum HitRegistrarType {
    Kafka(KafkaHitRegistrar),
    Fluvio(FluvioHitRegistrar),
}

#[async_trait::async_trait]
impl HitRegistrar for HitRegistrarType {
    async fn register(&self, hit: Hit) -> Result<()> {
        match self {
            HitRegistrarType::Kafka(registrar) => registrar.register(hit).await,
            HitRegistrarType::Fluvio(registrar) => registrar.register(hit).await,
        }
    }
}

#[derive(Clone)]
pub enum UserSettingsStoreType {
    //Redis,
    Dynamo(DynamoUserSettingsStore),
}

#[async_trait::async_trait]
impl UserSettingsStore for UserSettingsStoreType {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        match self {
            UserSettingsStoreType::Dynamo(store) => store.get_user_settings(user_id).await,
        }
    }
}

#[derive(Clone)]
pub enum UserSettingsCacheType {
    //Redis,
    Moka(MokaUserSettingsCache),
}

#[async_trait::async_trait]
impl UserSettingsCache for UserSettingsCacheType {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
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
pub enum CryptoStoreType {
    Dynamo(DynamoCryptoStore),
}

#[async_trait::async_trait]
impl CryptoStore for CryptoStoreType {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        match self {
            CryptoStoreType::Dynamo(store) => store.get_certificate(server_name).await,
        }
    }
}

#[derive(Clone)]
pub enum CryptoCacheType {
    Moka(MokaCryptoCache),
}

#[async_trait::async_trait]
impl CryptoCache for CryptoCacheType {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        match self {
            CryptoCacheType::Moka(store) => store.get_certificate(server_name).await,
        }
    }
    async fn invalidate(&self, server_name: &str) -> Result<()> {
        match self {
            CryptoCacheType::Moka(cache) => cache.invalidate(server_name).await,
        }
    }
}

#[derive(Clone)]
pub enum RoutesStoreType {
    Dynamo(DynamoRoutesStore),
}

#[async_trait::async_trait]
impl RoutesStore for RoutesStoreType {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        match self {
            RoutesStoreType::Dynamo(store) => store.get_route(switch, path).await,
        }
    }
}

#[derive(Clone)]
pub enum RoutesCacheType {
    Moka(MokaRoutesCache),
}

#[async_trait::async_trait]
impl RoutesCache for RoutesCacheType {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        match self {
            RoutesCacheType::Moka(cache) => cache.get_route(switch, path).await,
        }
    }

    async fn invalidate(&self, switch: &str, path: &str) -> Result<()> {
        match self {
            RoutesCacheType::Moka(cache) => cache.invalidate(switch, path).await,
        }
    }
}

#[derive(Clone)]
pub enum LocationDetectorType {
    GeoIP(GeoIPLocationDetector),
}

#[async_trait::async_trait]
impl LocationDetector for LocationDetectorType {
    fn detect_country(&self, &ip_addr: &IpAddr) -> Option<Country> {
        match self {
            LocationDetectorType::GeoIP(locator) => locator.detect_country(&ip_addr),
        }
    }
}

#[derive(Clone)]
pub enum UserAgentDetectorType {
    UAParser(UAParserUserAgentDetector),
}

#[async_trait::async_trait]
impl UserAgentDetector for UserAgentDetectorType {
    fn parse_device(&self, user_agent: &str) -> Device {
        match self {
            UserAgentDetectorType::UAParser(detector) => detector.parse_device(user_agent),
        }
    }
    fn parse_os(&self, user_agent: &str) -> OS {
        match self {
            UserAgentDetectorType::UAParser(detector) => detector.parse_os(user_agent),
        }
    }
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent {
        match self {
            UserAgentDetectorType::UAParser(detector) => detector.parse_user_agent(user_agent),
        }
    }
}
