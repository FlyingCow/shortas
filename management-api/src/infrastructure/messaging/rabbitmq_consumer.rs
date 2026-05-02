//! RabbitMQ consumer for processing async messages.

use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::domain::traits::DomainRepository;
use crate::settings::RabbitMqSettings;

/// Domain verification message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainVerificationMessage {
    pub domain_id: String,
    pub status: String,
    pub reason: String,
}

/// Route status message (Safe Browsing results).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStatusMessage {
    pub route_id: String,
    pub status: String,
    pub reason: Option<String>,
}

/// RabbitMQ consumer service.
pub struct RabbitMqConsumer {
    connection: Connection,
    settings: RabbitMqSettings,
}

impl RabbitMqConsumer {
    /// Create a new RabbitMQ consumer.
    pub async fn new(settings: &RabbitMqSettings) -> anyhow::Result<Self> {
        let connection = Connection::connect(&settings.url, ConnectionProperties::default()).await?;

        Ok(Self {
            connection,
            settings: settings.clone(),
        })
    }

    /// Create a channel and declare a queue.
    async fn setup_queue(&self, queue_name: &str) -> anyhow::Result<(Channel, Consumer)> {
        let channel = self.connection.create_channel().await?;

        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let consumer = channel
            .basic_consume(
                queue_name,
                "management-api",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok((channel, consumer))
    }

    /// Start the domain verification consumer.
    pub async fn start_domain_verification_consumer(
        &self,
        domain_repo: Arc<dyn DomainRepository>,
        mut shutdown: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let (channel, mut consumer) = self
            .setup_queue(&self.settings.domain_verification_queue)
            .await?;

        info!(
            "Started domain verification consumer on queue: {}",
            self.settings.domain_verification_queue
        );

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Shutting down domain verification consumer");
                    break;
                }
                delivery = consumer.next() => {
                    if let Some(Ok(delivery)) = delivery {
                        let data = &delivery.data;

                        match serde_json::from_slice::<DomainVerificationMessage>(data) {
                            Ok(msg) => {
                                info!("Processing domain verification: {:?}", msg);

                                if let Ok(domain_id) = uuid::Uuid::parse_str(&msg.domain_id) {
                                    if let Err(e) = domain_repo
                                        .update_verification_status(domain_id, &msg.status, &msg.reason)
                                        .await
                                    {
                                        error!("Failed to update domain verification: {}", e);
                                        let _ = channel
                                            .basic_nack(
                                                delivery.delivery_tag,
                                                BasicNackOptions { requeue: true, ..Default::default() },
                                            )
                                            .await;
                                        continue;
                                    }
                                }

                                let _ = channel
                                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                                    .await;
                            }
                            Err(e) => {
                                warn!("Failed to parse domain verification message: {}", e);
                                // Don't requeue invalid messages
                                let _ = channel
                                    .basic_nack(
                                        delivery.delivery_tag,
                                        BasicNackOptions { requeue: false, ..Default::default() },
                                    )
                                    .await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Start the route status consumer (Safe Browsing results).
    pub async fn start_route_status_consumer(
        &self,
        route_repo: Arc<dyn crate::domain::traits::RouteRepository>,
        mut shutdown: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let (channel, mut consumer) = self
            .setup_queue(&self.settings.route_status_queue)
            .await?;

        info!(
            "Started route status consumer on queue: {}",
            self.settings.route_status_queue
        );

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Shutting down route status consumer");
                    break;
                }
                delivery = consumer.next() => {
                    if let Some(Ok(delivery)) = delivery {
                        let data = &delivery.data;

                        match serde_json::from_slice::<RouteStatusMessage>(data) {
                            Ok(msg) => {
                                info!("Processing route status: {:?}", msg);

                                if let Ok(route_id) = uuid::Uuid::parse_str(&msg.route_id) {
                                    if let Ok(Some(mut route)) = route_repo.get_by_id(route_id).await {
                                        // Update route status based on Safe Browsing result
                                        if msg.status == "Blocked" {
                                            let reason = msg.reason.unwrap_or_else(|| "Safe Browsing".to_string());
                                            route.status = shortas_common::RouteStatus::Blocked(
                                                shortas_common::BlockedReason::Reasoned(reason)
                                            );

                                            if let Err(e) = route_repo.update(&route).await {
                                                error!("Failed to update route status: {}", e);
                                                let _ = channel
                                                    .basic_nack(
                                                        delivery.delivery_tag,
                                                        BasicNackOptions { requeue: true, ..Default::default() },
                                                    )
                                                    .await;
                                                continue;
                                            }
                                        }
                                    }
                                }

                                let _ = channel
                                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                                    .await;
                            }
                            Err(e) => {
                                warn!("Failed to parse route status message: {}", e);
                                let _ = channel
                                    .basic_nack(
                                        delivery.delivery_tag,
                                        BasicNackOptions { requeue: false, ..Default::default() },
                                    )
                                    .await;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check connection health.
    pub fn is_connected(&self) -> bool {
        self.connection.status().connected()
    }
}
