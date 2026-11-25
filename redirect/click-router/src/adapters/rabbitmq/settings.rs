use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: String,
    pub route_exchange: String,
    pub user_settings_exchange: String,
    #[serde(default = "default_certificate_exchange")]
    pub certificate_exchange: String,
    #[serde(default = "default_reconnect_seconds")]
    pub reconnect_seconds: u64,
}

fn default_reconnect_seconds() -> u64 {
    5
}

fn default_certificate_exchange() -> String {
    "cache.invalidation.certificates".to_string()
}
