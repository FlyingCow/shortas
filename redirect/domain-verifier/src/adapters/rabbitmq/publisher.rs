use anyhow::Result;
use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{debug, info, warn};

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

        // Enable publisher confirms
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;

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

        debug!(
            "Publishing domain state changed message: {} -> {}",
            message.domain_id, message.status
        );

        let properties = BasicProperties::default()
            .with_delivery_mode(2) // persistent
            .with_content_type("application/json".into());

        match self
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
            Ok(confirm) => {
                // Wait for publisher confirm
                match confirm.await {
                    Ok(confirmation) => {
                        info!(
                            "Published domain state changed: {} -> {} (confirmed: {:?})",
                            message.domain_id, message.status, confirmation
                        );
                    }
                    Err(e) => {
                        warn!(
                            "Publisher confirm failed for domain {}: {}",
                            message.domain_id, e
                        );
                    }
                }
            }
            Err(e) => {
                warn!("Failed to publish domain state changed message: {}", e);
            }
        }
    }
}
