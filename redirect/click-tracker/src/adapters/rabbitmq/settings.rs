use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: String,
    pub user_settings_exchange: String,
    #[serde(default = "default_reconnect_seconds")]
    pub reconnect_seconds: u64,
}

fn default_reconnect_seconds() -> u64 {
    5
}
