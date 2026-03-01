use config::{Config, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AcmeSettings {
    pub directory_url: String,
    pub account_email: String,
    #[serde(default = "default_account_key_path")]
    pub account_key_path: String,
}

fn default_account_key_path() -> String {
    "/etc/cert-bot/account.key".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClickRouterApiSettings {
    pub base_url: String,
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
}

fn default_timeout_seconds() -> u64 {
    30
}

#[derive(Debug, Deserialize, Clone)]
pub struct MongodbSettings {
    pub uri: String,
    pub database: String,
    #[serde(default = "default_orders_collection")]
    pub orders_collection: String,
}

fn default_orders_collection() -> String {
    "certificate_orders".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: String,
    pub domain_exchange: String,
    pub certificate_exchange: String,
    #[serde(default = "default_reconnect_seconds")]
    pub reconnect_seconds: u64,
}

fn default_reconnect_seconds() -> u64 {
    5
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkerSettings {
    #[serde(default = "default_check_interval_seconds")]
    pub check_interval_seconds: u64,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

fn default_check_interval_seconds() -> u64 {
    30
}

fn default_batch_size() -> usize {
    10
}

fn default_max_retries() -> u32 {
    3
}

#[derive(Debug, Deserialize, Clone)]
pub struct RenewalSettings {
    #[serde(default = "default_check_interval_hours")]
    pub check_interval_hours: u64,
    #[serde(default = "default_renewal_days_before")]
    pub renewal_days_before: i64,
}

fn default_check_interval_hours() -> u64 {
    24
}

fn default_renewal_days_before() -> i64 {
    30
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub acme: AcmeSettings,
    pub click_router_api: ClickRouterApiSettings,
    pub mongodb: MongodbSettings,
    pub rabbitmq: RabbitMqSettings,
    pub worker: WorkerSettings,
    pub renewal: RenewalSettings,
}

impl Settings {
    pub fn new(run_mode: Option<&str>, config_path: Option<&str>) -> anyhow::Result<Self> {
        let run_mode = run_mode.unwrap_or("development");
        let config_path = config_path.unwrap_or("./config");

        let builder = Config::builder()
            .add_source(File::with_name(&format!("{}/default", config_path)).required(false))
            .add_source(File::with_name(&format!("{}/{}", config_path, run_mode)).required(false))
            .add_source(File::with_name(&format!("{}/local", config_path)).required(false))
            .add_source(Environment::with_prefix("CERTBOT").separator("__"));

        let settings = builder.build()?.try_deserialize()?;

        Ok(settings)
    }
}
