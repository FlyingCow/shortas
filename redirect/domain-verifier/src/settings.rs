use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize as _, Deserializer};
use serde_derive::Deserialize;

use crate::adapters::aws::settings::AwsSettings;

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

/// DNS verification settings.
///
/// `allowed_ipv4` / `allowed_ipv6` are the full set of public IPs a custom
/// domain may point at (e.g. every click-router node or load balancer). A
/// domain passes the A check when at least one of its A records is in the
/// list. Both accept a TOML array or, so deployments can set them from a
/// single env var, a comma-separated string:
/// `APP__DNS__ALLOWED_IPV4="203.0.113.10,203.0.113.11"`.
#[derive(Debug, Deserialize, Clone)]
pub struct DnsSettings {
    pub txt_record_name: String,
    #[serde(deserialize_with = "string_or_list")]
    pub allowed_ipv4: Vec<String>,
    #[serde(default, deserialize_with = "string_or_list")]
    pub allowed_ipv6: Vec<String>,
}

/// Accept either a list of strings or one comma/whitespace-separated string.
/// Empty entries are dropped, so an empty env var yields an empty list.
fn string_or_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(serde_derive::Deserialize)]
    #[serde(untagged)]
    enum StringOrList {
        One(String),
        Many(Vec<String>),
    }

    let items = match StringOrList::deserialize(deserializer)? {
        StringOrList::One(s) => s
            .split(|c: char| c == ',' || c.is_whitespace())
            .map(str::to_owned)
            .collect(),
        StringOrList::Many(v) => v,
    };

    Ok(items
        .into_iter()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect())
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
    #[serde(default)]
    pub aws: AwsSettings,
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
            .add_source(Environment::with_prefix("app").separator("__"))
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
    fn test_dns_ip_lists_accept_comma_separated_string() {
        #[derive(serde_derive::Deserialize)]
        struct Wrapper {
            dns: DnsSettings,
        }
        let cfg = Config::builder()
            .set_override("dns.txt_record_name", "_c")
            .unwrap()
            .set_override("dns.allowed_ipv4", "203.0.113.10, 203.0.113.11,,198.51.100.7")
            .unwrap()
            .set_override("dns.allowed_ipv6", "")
            .unwrap()
            .build()
            .unwrap();
        let w: Wrapper = cfg.try_deserialize().unwrap();
        assert_eq!(
            w.dns.allowed_ipv4,
            vec!["203.0.113.10", "203.0.113.11", "198.51.100.7"]
        );
        assert!(w.dns.allowed_ipv6.is_empty());
    }

    #[test]
    fn test_dns_ip_list_env_var_overrides_toml_array() {
        // Unique prefix so this can't race other tests reading APP__*.
        std::env::set_var("DVTEST__DNS__ALLOWED_IPV4", "198.51.100.1,198.51.100.2");
        let cfg = Config::builder()
            .add_source(File::with_name("./config/default"))
            .add_source(Environment::with_prefix("dvtest").separator("__"))
            .build()
            .unwrap();
        #[derive(serde_derive::Deserialize)]
        struct Wrapper {
            dns: DnsSettings,
        }
        let w: Wrapper = cfg.try_deserialize().unwrap();
        assert_eq!(w.dns.allowed_ipv4, vec!["198.51.100.1", "198.51.100.2"]);
    }

    #[test]
    fn test_dns_ip_lists_accept_toml_array() {
        let s = Settings::new(Some("development"), Some("./config")).unwrap();
        assert!(s.dns.allowed_ipv4.len() >= 2);
    }

    #[test]
    fn test_settings_invalid_path_fails() {
        let settings = Settings::new(Some("development"), Some("/nonexistent/path"));
        assert!(settings.is_err());
    }
}
