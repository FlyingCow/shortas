use anyhow::Result;
use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{debug, info, warn};

use super::messages::RouteStatusChangedMessage;
use crate::settings::RabbitMqSettings;

#[derive(Clone)]
pub struct RabbitMqPublisher {
    channel: Channel,
    route_status_exchange: String,
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
                &settings.route_status_exchange,
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
            settings.route_status_exchange
        );

        Ok(Self {
            channel,
            route_status_exchange: settings.route_status_exchange.clone(),
        })
    }

    pub async fn publish_route_status_changed(&self, message: &RouteStatusChangedMessage) {
        let payload = match serde_json::to_vec(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to serialize route status changed message: {}", e);
                return;
            }
        };

        debug!(
            "Publishing route status changed message: {} -> {}",
            message.route_id, message.new_status
        );

        let properties = BasicProperties::default()
            .with_delivery_mode(2) // persistent
            .with_content_type("application/json".into());

        match self
            .channel
            .basic_publish(
                &self.route_status_exchange,
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
                            "Published route status changed: {} -> {} (confirmed: {:?})",
                            message.route_id, message.new_status, confirmation
                        );
                    }
                    Err(e) => {
                        warn!(
                            "Publisher confirm failed for route {}: {}",
                            message.route_id, e
                        );
                    }
                }
            }
            Err(e) => {
                warn!("Failed to publish route status changed message: {}", e);
            }
        }
    }
}
