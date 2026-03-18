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
    pub route_status_exchange: String,
    pub reconnect_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SafeBrowsingSettings {
    pub base_url: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkerSettings {
    pub check_interval_seconds: u64,
    pub batch_size: usize,
    pub recheck_interval_hours: i64,
    pub blocked_recheck_interval_hours: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub mongodb: Mongodb,
    pub rabbitmq: Option<RabbitMqSettings>,
    pub safe_browsing: SafeBrowsingSettings,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_loads_default_config() {
        let settings = Settings::new(Some("development"), Some("./config"));
        assert!(settings.is_ok(), "Failed to load settings: {:?}", settings.err());

        let s = settings.unwrap();
        assert_eq!(s.server.port, 5831);
        assert!(!s.mongodb.connection_string.is_empty());
    }

    #[test]
    fn test_settings_worker_fields() {
        let settings = Settings::new(Some("development"), Some("./config")).unwrap();
        assert!(settings.worker.check_interval_seconds > 0);
        assert!(settings.worker.batch_size > 0);
        assert!(settings.worker.recheck_interval_hours > 0);
        assert!(settings.worker.blocked_recheck_interval_hours > 0);
    }
}
