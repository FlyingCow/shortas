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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_loads_default_config() {
        let settings = Settings::new(Some("development"), Some("./config"));
        assert!(settings.is_ok(), "Failed to load settings: {:?}", settings.err());

        let s = settings.unwrap();
        assert_eq!(s.server.port, 5830);
        assert_eq!(s.dns.txt_record_name, "_shortas-domain-challenge");
        assert!(!s.dns.allowed_ipv4.is_empty());
    }

    #[test]
    fn test_settings_default_run_mode_is_development() {
        let settings = Settings::new(None, Some("./config"));
        assert!(settings.is_ok());
    }

    #[test]
    fn test_settings_mongodb_fields() {
        let settings = Settings::new(Some("development"), Some("./config")).unwrap();
        assert!(!settings.mongodb.connection_string.is_empty());
        assert!(!settings.mongodb.database_name.is_empty());
        assert!(!settings.mongodb.collection.is_empty());
    }

    #[test]
    fn test_settings_worker_fields() {
        let settings = Settings::new(Some("development"), Some("./config")).unwrap();
        assert!(settings.worker.check_interval_seconds > 0);
        assert!(settings.worker.batch_size > 0);
        assert!(settings.worker.recheck_interval_minutes > 0);
        assert!(settings.worker.failed_recheck_interval_minutes > 0);
    }

    #[test]
    fn test_settings_dns_fields() {
        let settings = Settings::new(Some("development"), Some("./config")).unwrap();
        assert!(!settings.dns.txt_record_name.is_empty());
        // allowed_ipv6 can be empty
    }

    #[test]
    fn test_settings_nonexistent_run_mode_falls_back() {
        // Non-existent run mode file is optional, should still load default
        let settings = Settings::new(Some("nonexistent_mode"), Some("./config"));
        assert!(settings.is_ok());
    }

    #[test]
    #[should_panic(expected = "No configuration folder specified")]
    fn test_settings_panics_without_path() {
        let _ = Settings::new(Some("development"), None);
    }

    #[test]
    fn test_settings_invalid_path_fails() {
        let settings = Settings::new(Some("development"), Some("/nonexistent/path"));
        assert!(settings.is_err());
    }
}
