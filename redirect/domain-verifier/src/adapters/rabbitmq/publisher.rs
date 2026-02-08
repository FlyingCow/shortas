use anyhow::Result;
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{info, warn};

use super::messages::DomainStateChangedMessage;
use crate::settings::RabbitMqSettings;

#[derive(Clone)]
pub struct RabbitMqPublisher {
    channel: Channel,
    domain_state_exchange: String,
}

impl RabbitMqPublisher {
    pub async fn new(settings: &RabbitMqSettings) -> Result<Self> {
        let conn = Connection::connect(&settings.uri, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        channel
            .exchange_declare(
                &settings.domain_state_exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        info!(
            "RabbitMQ publisher connected, exchange: {}",
            settings.domain_state_exchange
        );

        Ok(Self {
            channel,
            domain_state_exchange: settings.domain_state_exchange.clone(),
        })
    }

    pub async fn publish_domain_state_changed(&self, message: &DomainStateChangedMessage) {
        let payload = match serde_json::to_vec(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to serialize domain state changed message: {}", e);
                return;
            }
        };

        let properties = BasicProperties::default()
            .with_delivery_mode(2) // persistent
            .with_content_type("application/json".into());

        if let Err(e) = self
            .channel
            .basic_publish(
                &self.domain_state_exchange,
                "",
                BasicPublishOptions::default(),
                &payload,
                properties,
            )
            .await
        {
            warn!("Failed to publish domain state changed message: {}", e);
        } else {
            info!(
                "Published domain state changed: {} -> {}",
                message.domain_id, message.status
            );
        }
    }
}
