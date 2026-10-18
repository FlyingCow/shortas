//! Application settings and configuration.

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const DEV_RUN_MODE: &str = "development";

/// Root settings structure.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub jwt: JwtSettings,
    pub click_router: ClickRouterSettings,
    pub click_aggregator: ClickAggregatorSettings,
    pub elasticsearch: ElasticsearchSettings,
    pub minio: MinioSettings,
    pub rabbitmq: RabbitMqSettings,
    #[serde(default)]
    pub shared_domains: SharedDomainsSettings,
}

/// Shared domains settings.
#[derive(Debug, Clone, Deserialize)]
pub struct SharedDomainsSettings {
    pub names: Vec<String>,
}

impl Default for SharedDomainsSettings {
    fn default() -> Self {
        Self {
            names: vec!["shortas.space".to_string()],
        }
    }
}

/// HTTP server settings.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

/// PostgreSQL database settings.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

impl DatabaseSettings {
    /// Get the connection string.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "shortas".to_string(),
            max_connections: 10,
        }
    }
}

/// JWT authentication settings.
#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub issuer: String,
    pub audience: String,
    pub jwks_url: String,
}

impl Default for JwtSettings {
    fn default() -> Self {
        Self {
            issuer: "https://auth.shortas.work/realms/shortas".to_string(),
            audience: "shortas-api".to_string(),
            jwks_url: "https://auth.shortas.work/realms/shortas/protocol/openid-connect/certs".to_string(),
        }
    }
}

/// Click Router API settings.
#[derive(Debug, Clone, Deserialize)]
pub struct ClickRouterSettings {
    pub base_url: String,
    pub timeout_ms: u64,
}

impl Default for ClickRouterSettings {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8081".to_string(),
            timeout_ms: 5000,
        }
    }
}

/// Click Aggregator API settings.
#[derive(Debug, Clone, Deserialize)]
pub struct ClickAggregatorSettings {
    pub base_url: String,
    pub timeout_ms: u64,
}

impl Default for ClickAggregatorSettings {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8082".to_string(),
            timeout_ms: 10000,
        }
    }
}

/// Elasticsearch settings.
#[derive(Debug, Clone, Deserialize)]
pub struct ElasticsearchSettings {
    pub url: String,
    pub routes_index: String,
}

impl Default for ElasticsearchSettings {
    fn default() -> Self {
        Self {
            url: "http://localhost:9200".to_string(),
            routes_index: "routes".to_string(),
        }
    }
}

/// MinIO/S3 settings.
#[derive(Debug, Clone, Deserialize)]
pub struct MinioSettings {
    pub endpoint: String,
    /// Public endpoint for presigned URLs (browser-accessible). Falls back to endpoint if not set.
    pub public_endpoint: Option<String>,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
}

impl Default for MinioSettings {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:9000".to_string(),
            public_endpoint: None,
            region: "us-east-1".to_string(),
            access_key: "minioadmin".to_string(),
            secret_key: "minioadmin".to_string(),
            bucket: "shortas".to_string(),
        }
    }
}

/// RabbitMQ settings.
#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqSettings {
    pub url: String,
    pub domain_verification_queue: String,
    pub route_status_queue: String,
}

impl Default for RabbitMqSettings {
    fn default() -> Self {
        Self {
            url: "amqp://localhost:5672".to_string(),
            domain_verification_queue: "domain-verification".to_string(),
            route_status_queue: "route-status".to_string(),
        }
    }
}

impl Settings {
    /// Load settings from config files and environment variables.
    ///
    /// Config files are loaded in the following order (later sources override earlier ones):
    /// 1. `{config_path}/default.toml` - Base configuration
    /// 2. `{config_path}/{run_mode}.toml` - Environment-specific config (optional)
    /// 3. `{config_path}/local.toml` - Local overrides, not in git (optional)
    /// 4. Environment variables with `APP__` prefix (e.g., `APP__SERVER__PORT=8080`)
    pub fn new(run_mode: Option<&str>, config_path: Option<&str>) -> Result<Self, ConfigError> {
        let run_mode = run_mode.unwrap_or(DEV_RUN_MODE);
        let path = config_path.unwrap_or("./config");

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
            // Eg.. `APP__SERVER__PORT=8080 ./target/app` would set the `server.port` key
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        s.try_deserialize()
    }

    /// Load settings with default run mode and config path.
    /// Convenience method for backwards compatibility.
    pub fn load() -> anyhow::Result<Self> {
        Self::new(None, None).map_err(|e| anyhow::anyhow!("Failed to load settings: {}", e))
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings::default(),
            database: DatabaseSettings::default(),
            jwt: JwtSettings::default(),
            click_router: ClickRouterSettings::default(),
            click_aggregator: ClickAggregatorSettings::default(),
            elasticsearch: ElasticsearchSettings::default(),
            minio: MinioSettings::default(),
            rabbitmq: RabbitMqSettings::default(),
            shared_domains: SharedDomainsSettings::default(),
        }
    }
}
