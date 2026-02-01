use anyhow::Result;
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{info, warn};

use super::{
    messages::{RouteChangedMessage, UserSettingsChangedMessage},
    settings::RabbitMqSettings,
};

#[derive(Clone)]
pub struct RabbitMqPublisher {
    channel: Channel,
    route_exchange: String,
    user_settings_exchange: String,
}

impl RabbitMqPublisher {
    pub async fn new(settings: &RabbitMqSettings) -> Result<Self> {
        let conn = Connection::connect(&settings.uri, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        // Declare fanout exchanges
        channel
            .exchange_declare(
                &settings.route_exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .exchange_declare(
                &settings.user_settings_exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        info!(
            "RabbitMQ publisher connected, exchanges: {}, {}",
            settings.route_exchange, settings.user_settings_exchange
        );

        Ok(Self {
            channel,
            route_exchange: settings.route_exchange.clone(),
            user_settings_exchange: settings.user_settings_exchange.clone(),
        })
    }

    pub async fn publish_route_changed(&self, message: &RouteChangedMessage) {
        let payload = match serde_json::to_vec(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to serialize route changed message: {}", e);
                return;
            }
        };

        let properties = BasicProperties::default().with_delivery_mode(1); // non-persistent

        if let Err(e) = self
            .channel
            .basic_publish(
                &self.route_exchange,
                "",
                BasicPublishOptions::default(),
                &payload,
                properties,
            )
            .await
        {
            warn!("Failed to publish route changed message: {}", e);
        }
    }

    pub async fn publish_user_settings_changed(&self, message: &UserSettingsChangedMessage) {
        let payload = match serde_json::to_vec(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to serialize user settings changed message: {}", e);
                return;
            }
        };

        let properties = BasicProperties::default().with_delivery_mode(1); // non-persistent

        if let Err(e) = self
            .channel
            .basic_publish(
                &self.user_settings_exchange,
                "",
                BasicPublishOptions::default(),
                &payload,
                properties,
            )
            .await
        {
            warn!(
                "Failed to publish user settings changed message: {}",
                e
            );
        }
    }
}
