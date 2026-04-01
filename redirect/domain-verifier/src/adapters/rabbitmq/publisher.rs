use anyhow::Result;
use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::messages::DomainStateChangedMessage;
use crate::settings::RabbitMqSettings;

#[derive(Clone)]
pub struct RabbitMqPublisher {
    channel: Arc<RwLock<Option<Channel>>>,
    settings: RabbitMqSettings,
}

impl RabbitMqPublisher {
    pub async fn new(settings: &RabbitMqSettings) -> Result<Self> {
        let publisher = Self {
            channel: Arc::new(RwLock::new(None)),
            settings: settings.clone(),
        };

        // Try initial connection
        publisher.reconnect().await;

        // Start background reconnection loop
        let publisher_clone = publisher.clone();
        tokio::spawn(async move {
            publisher_clone.reconnection_loop().await;
        });

        Ok(publisher)
    }

    async fn reconnection_loop(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(self.settings.reconnect_seconds))
                .await;

            let needs_reconnect = {
                let channel = self.channel.read().await;
                match &*channel {
                    Some(ch) => !ch.status().connected(),
                    None => true,
                }
            };

            if needs_reconnect {
                warn!("RabbitMQ publisher disconnected, reconnecting...");
                self.reconnect().await;
            }
        }
    }

    async fn reconnect(&self) {
        match self.try_connect().await {
            Ok(channel) => {
                let mut ch = self.channel.write().await;
                *ch = Some(channel);
                info!(
                    "RabbitMQ publisher connected, exchange: {}",
                    self.settings.domain_state_exchange
                );
            }
            Err(e) => {
                warn!("Failed to connect to RabbitMQ: {}", e);
            }
        }
    }

    async fn try_connect(&self) -> Result<Channel> {
        let conn =
            Connection::connect(&self.settings.uri, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        // Enable publisher confirms
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;

        channel
            .exchange_declare(
                &self.settings.domain_state_exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        Ok(channel)
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

        let channel = self.channel.read().await;
        if let Some(ref ch) = *channel {
            match ch
                .basic_publish(
                    &self.settings.domain_state_exchange,
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
        } else {
            warn!("RabbitMQ publisher not connected, skipping domain state changed message");
        }
    }
}
