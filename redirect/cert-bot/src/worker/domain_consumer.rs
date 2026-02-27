use anyhow::Result;
use chrono::{Duration, Utc};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::adapters::click_router_api::ClickRouterApiClient;
use crate::adapters::mongodb::MongodbOrderStore;
use crate::core::OrderStore;
use crate::model::CertificateOrder;
use crate::settings::Settings;

/// Message received when a domain's verification status changes
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainStateChangedMessage {
    pub domain_id: String,
    pub domain_name: String,
    pub owner_id: String,
    pub status: String,
    pub verification_reason: Option<String>,
    pub last_check_at: Option<i64>,
    pub next_check_at: Option<i64>,
}

/// Worker that consumes domain verification events and creates certificate orders
/// for newly verified domains
pub struct DomainConsumer {
    settings: Settings,
    order_store: Arc<MongodbOrderStore>,
    api_client: Arc<ClickRouterApiClient>,
}

impl DomainConsumer {
    pub fn new(
        settings: Settings,
        order_store: Arc<MongodbOrderStore>,
        api_client: Arc<ClickRouterApiClient>,
    ) -> Self {
        Self {
            settings,
            order_store,
            api_client,
        }
    }

    pub async fn run(self) -> Result<()> {
        info!("Domain consumer starting...");

        loop {
            if let Err(e) = self.consume_domain_changes().await {
                warn!("Domain consumer error: {}, reconnecting...", e);
            }

            // Wait before reconnecting
            tokio::time::sleep(tokio::time::Duration::from_secs(
                self.settings.rabbitmq.reconnect_seconds,
            ))
            .await;
        }
    }

    async fn consume_domain_changes(&self) -> Result<()> {
        info!("Connecting to RabbitMQ for domain events...");

        let conn = Connection::connect(
            &self.settings.rabbitmq.uri,
            ConnectionProperties::default(),
        )
        .await?;

        let channel = conn.create_channel().await?;

        // Declare exclusive queue for this consumer
        let queue = channel
            .queue_declare(
                "",
                QueueDeclareOptions {
                    exclusive: true,
                    auto_delete: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Bind to domain state changes exchange
        channel
            .queue_bind(
                queue.name().as_str(),
                &self.settings.rabbitmq.domain_exchange,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "Listening for domain state changes on exchange: {}",
            self.settings.rabbitmq.domain_exchange
        );

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "cert-bot-domain-consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    if let Err(e) = self.handle_message(&delivery.data).await {
                        error!("Error handling domain message: {}", e);
                    }

                    // Acknowledge the message
                    if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                        error!("Error acknowledging message: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, data: &[u8]) -> Result<()> {
        let message: DomainStateChangedMessage = serde_json::from_slice(data)?;

        info!(
            "Received domain state change: {} -> {}",
            message.domain_name, message.status
        );

        // Only process verified domains
        if message.status != "verified" {
            return Ok(());
        }

        // Check if certificate already exists and is valid
        let needs_cert = match self
            .api_client
            .get_certificate(&message.domain_name)
            .await
        {
            Ok(Some(cert)) => {
                // Check if certificate expires within renewal window
                cert.expires_at.map_or(true, |exp| {
                    exp < Utc::now() + Duration::days(self.settings.renewal.renewal_days_before)
                })
            }
            Ok(None) => true,
            Err(e) => {
                warn!(
                    "Error checking certificate for {}: {}, will create order",
                    message.domain_name, e
                );
                true
            }
        };

        if !needs_cert {
            info!(
                "Certificate for {} is still valid, skipping",
                message.domain_name
            );
            return Ok(());
        }

        // Check if there's already an active order
        if let Ok(Some(_)) = self
            .order_store
            .get_active_order_for_domain(&message.domain_name)
            .await
        {
            info!(
                "Active order already exists for {}, skipping",
                message.domain_name
            );
            return Ok(());
        }

        // Create certificate order
        let order = CertificateOrder::new(
            message.domain_name.clone(),
            message.owner_id.clone(),
            self.settings.worker.max_retries,
        );

        match self.order_store.store_order(&order).await {
            Ok(_) => {
                info!(
                    "Created certificate order for newly verified domain: {}",
                    message.domain_name
                );
            }
            Err(e) => {
                error!(
                    "Failed to create certificate order for {}: {}",
                    message.domain_name, e
                );
            }
        }

        Ok(())
    }
}
