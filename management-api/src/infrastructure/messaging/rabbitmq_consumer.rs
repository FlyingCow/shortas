//! RabbitMQ consumer for processing async messages.

use futures::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, ExchangeDeclareOptions,
        QueueBindOptions, QueueDeclareOptions,
    },
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer, ExchangeKind,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::domain::entities::{BlockedReason, RouteStatus};
use crate::domain::traits::DomainRepository;
use crate::settings::RabbitMqSettings;

/// Domain verification message, matching domain-verifier's
/// `DomainStateChangedMessage` wire format. Unused fields are kept out so a
/// field rename on either side fails loudly instead of silently mismatching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainVerificationMessage {
    pub domain_id: String,
    pub status: String,
    pub verification_reason: String,
}

/// Route status message (Safe Browsing results), matching route-verifier's
/// `RouteStatusChangedMessage` wire format. Unused fields are kept out so a
/// field rename on either side fails loudly instead of silently mismatching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStatusMessage {
    pub route_id: String,
    pub new_status: String,
    pub blocked_reason: Option<String>,
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

    /// Create a channel, declare a durable queue, and bind it to the given
    /// fanout exchange. The publishers (domain-verifier, route-verifier)
    /// publish to a fanout exchange rather than directly to a queue, so
    /// without this bind messages are published into the void and silently
    /// dropped - declaring the queue alone is not enough to receive them.
    async fn setup_queue(&self, queue_name: &str, exchange_name: &str) -> anyhow::Result<(Channel, Consumer)> {
        let channel = self.connection.create_channel().await?;

        channel
            .exchange_declare(
                exchange_name,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

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

        channel
            .queue_bind(
                queue_name,
                exchange_name,
                "",
                QueueBindOptions::default(),
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
            .setup_queue(&self.settings.domain_verification_queue, &self.settings.domain_state_exchange)
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
                                    // domain-verifier serializes its status as lowercase
                                    // ("verified"/"failed"/"pending" via #[serde(rename_all = "snake_case")]),
                                    // but DomainVerificationStatus::from_str only recognizes the
                                    // capitalized form used elsewhere in this app - anything else
                                    // silently falls back to "Pending". Normalize here.
                                    let status = match msg.status.to_lowercase().as_str() {
                                        "verified" => "Verified",
                                        "failed" => "Failed",
                                        _ => "Pending",
                                    };

                                    if let Err(e) = domain_repo
                                        .update_verification_status(domain_id, status, &msg.verification_reason)
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
            .setup_queue(&self.settings.route_status_queue, &self.settings.route_status_exchange)
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
                                        if msg.new_status == "Blocked" {
                                            let reason = msg.blocked_reason.unwrap_or_else(|| "Safe Browsing".to_string());
                                            route.status = RouteStatus::Blocked(
                                                BlockedReason::Reasoned(reason)
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
