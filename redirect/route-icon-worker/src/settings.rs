use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: String,
    pub route_exchange: String,
    #[serde(default = "default_reconnect_seconds")]
    pub reconnect_seconds: u64,
}

fn default_reconnect_seconds() -> u64 {
    5
}

#[derive(Debug, Deserialize, Clone)]
pub struct S3Settings {
    pub endpoint: String,
    pub bucket: String,
    #[serde(default = "default_region")]
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}

fn default_region() -> String {
    "us-east-1".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkerSettings {
    #[serde(default = "default_request_timeout_seconds")]
    pub request_timeout_seconds: u64,
    #[serde(default = "default_max_image_size_bytes")]
    pub max_image_size_bytes: usize,
}

fn default_request_timeout_seconds() -> u64 {
    10
}

fn default_max_image_size_bytes() -> usize {
    1_048_576 // 1MB
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub rabbitmq: RabbitMqSettings,
    pub s3: S3Settings,
    #[serde(default)]
    pub worker: WorkerSettings,
}

impl Default for WorkerSettings {
    fn default() -> Self {
        Self {
            request_timeout_seconds: default_request_timeout_seconds(),
            max_image_size_bytes: default_max_image_size_bytes(),
        }
    }
}

const DEV_RUN_MODE: &str = "development";

impl Settings {
    pub fn new(run_mode: Option<&str>, path: Option<&str>) -> Result<Self, ConfigError> {
        let run_mode = run_mode.unwrap_or(DEV_RUN_MODE);
        let path = path.expect("No configuration folder specified.");

        let s = Config::builder()
            .add_source(File::with_name(&format!("{}/default", path)))
            .add_source(File::with_name(&format!("{}/{}", path, run_mode)).required(false))
            .add_source(File::with_name(&format!("{}/local", path)).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}
