use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMqSettings {
    pub uri: String,
    pub route_exchange: String,
    pub user_settings_exchange: String,
}
