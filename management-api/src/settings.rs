//! Application settings and configuration.

use serde::Deserialize;

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
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
}

impl Default for MinioSettings {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:9000".to_string(),
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
    /// Load settings from environment variables and config files.
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        let settings: Settings = config.try_deserialize().unwrap_or_else(|_| Settings::default());
        Ok(settings)
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
        }
    }
}
