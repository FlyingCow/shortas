use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mongodb {
    pub connection_string: String,
    pub database_name: String,
    pub collection: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: String,
    pub domain_state_exchange: String,
    pub reconnect_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DnsSettings {
    pub txt_record_name: String,
    pub allowed_ipv4: Vec<String>,
    pub allowed_ipv6: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkerSettings {
    pub check_interval_seconds: u64,
    pub batch_size: usize,
    pub recheck_interval_minutes: i64,
    pub failed_recheck_interval_minutes: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub mongodb: Mongodb,
    pub rabbitmq: Option<RabbitMqSettings>,
    pub dns: DnsSettings,
    pub worker: WorkerSettings,
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
