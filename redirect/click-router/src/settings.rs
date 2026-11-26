use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

use crate::adapters::aws::settings::AWS;
use crate::adapters::fluvio::settings::{Fluvio, HitStreamConfig};
use crate::adapters::geo_ip::settings::GeoIP;
use crate::adapters::moka::settings::Moka;
use crate::adapters::mongodb::settings::Mongodb;
use crate::adapters::rabbitmq::settings::RabbitMqSettings;
use crate::adapters::uaparser::settings::UAParser;
use crate::core::rate_limit::Rate;

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Redirect {
    pub not_found_url: String,
    pub index_url: String,
    pub blocked_url: String,
    /// Base URL for fetching stored QR images (e.g., "http://minio-nginx/route-images")
    /// QR images are fetched from: {qr_images_base_url}/{owner_id}/{route_id}/qr.svg
    #[serde(default)]
    pub qr_images_base_url: Option<String>,
    /// Add `shortas_cid` to the destinations of routes covered by conversions.
    #[serde(default)]
    pub click_ids_enabled: bool,
}
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub threads: usize,
    pub listen_os_signals: bool,
    pub exit: bool,
}
/// Conversion endpoints.
#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Conversions {
    pub enabled: bool,
    /// Must be acknowledged, so the linger time delays every response.
    pub stream: HitStreamConfig,
    /// Networks whose `X-Forwarded-For` is believed.
    pub trusted_proxies: Vec<String>,
    pub max_concurrent_requests: usize,
    /// Per client address. Servers with a valid secret key are exempt.
    pub address_rate: Rate,
    /// Per conversion, separately for browsers and servers.
    pub key_rate: Rate,
}

impl Default for Conversions {
    fn default() -> Self {
        Self {
            enabled: false,
            stream: HitStreamConfig {
                host: "sc:9003".into(),
                topic: "conversion-stream-main".into(),
                batch_size: 100_000,
                linger: 5,
            },
            trusted_proxies: vec![],
            max_concurrent_requests: 256,
            address_rate: Rate { per_second: 20.0, burst: 100.0 },
            key_rate: Rate { per_second: 200.0, burst: 1_000.0 },
        }
    }
}

/// HTTPS listener settings.
#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Tls {
    /// Pick the certificate by requested server name. When off, every
    /// connection gets the fallback certificate.
    pub sni_enabled: bool,
    /// PEM files of the fallback certificate. A self-signed one is generated
    /// at startup when neither is set.
    pub fallback_cert_path: Option<String>,
    pub fallback_key_path: Option<String>,
    /// How long a handshake waits for a certificate before using the fallback.
    pub lookup_timeout_millis: u64,
    /// How long a client gets to finish the handshake.
    pub handshake_timeout_seconds: u64,
}

impl Default for Tls {
    fn default() -> Self {
        Self {
            sni_enabled: false,
            fallback_cert_path: None,
            fallback_key_path: None,
            lookup_timeout_millis: 2_000,
            handshake_timeout_seconds: 10,
        }
    }
}

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    #[serde(default)]
    pub tls: Tls,
    #[serde(default)]
    pub conversions: Conversions,
    pub aws: AWS,
    pub mongodb: Mongodb,
    pub fluvio: Fluvio,
    pub moka: Moka,
    pub uaparser: UAParser,
    pub geo_ip: GeoIP,
    pub server: Server,
    pub redirect: Redirect,
    pub rabbitmq: Option<RabbitMqSettings>,
}
const DEV_RUN_MODE: &'static str = "development";

impl Settings {
    pub fn new(run_mode: Option<&str>, path: Option<&str>) -> Result<Self, ConfigError> {
        let run_mode = run_mode.unwrap_or(DEV_RUN_MODE);

        let path = path.expect("No configuration folder specified.");

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&format!("{}/default", path)))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("{}/{}", path, run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(&format!("{}/local", path)).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app").separator("__"))
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

        // // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("database.url"));

        // // You can deserialize (and thus freeze) the entire configuration as

        s.try_deserialize()
    }
}
