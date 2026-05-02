//! Outbox processor for reliable message delivery.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::domain::entities::OutboxMessageType;
use crate::domain::traits::OutboxRepository;
use crate::infrastructure::search::ElasticsearchService;

/// Outbox processor background service.
pub struct OutboxProcessor {
    outbox_repo: Arc<dyn OutboxRepository>,
    search_service: Arc<ElasticsearchService>,
    route_repo: Arc<dyn crate::domain::traits::RouteRepository>,
}

impl OutboxProcessor {
    /// Create a new outbox processor.
    pub fn new(
        outbox_repo: Arc<dyn OutboxRepository>,
        search_service: Arc<ElasticsearchService>,
        route_repo: Arc<dyn crate::domain::traits::RouteRepository>,
    ) -> Self {
        Self {
            outbox_repo,
            search_service,
            route_repo,
        }
    }

    /// Start processing outbox messages.
    pub async fn start(&self, mut shutdown: broadcast::Receiver<()>) {
        let mut process_interval = interval(Duration::from_secs(5));
        let mut cleanup_interval = interval(Duration::from_secs(3600)); // Hourly cleanup

        info!("Started outbox processor");

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Shutting down outbox processor");
                    break;
                }
                _ = process_interval.tick() => {
                    self.process_pending().await;
                }
                _ = cleanup_interval.tick() => {
                    self.cleanup_old_messages().await;
                }
            }
        }
    }

    /// Process pending outbox messages.
    async fn process_pending(&self) {
        // Fetch up to 100 pending messages
        let messages = match self.outbox_repo.get_pending(100).await {
            Ok(msgs) => msgs,
            Err(e) => {
                error!("Failed to fetch pending outbox messages: {}", e);
                return;
            }
        };

        if messages.is_empty() {
            return;
        }

        info!("Processing {} outbox messages", messages.len());

        for message in messages {
            // Mark as processing
            if let Err(e) = self.outbox_repo.mark_processing(message.id).await {
                warn!("Failed to mark message {} as processing: {}", message.id, e);
                continue;
            }

            // Process based on message type
            let result = match message.message_type {
                OutboxMessageType::IndexRoute => {
                    self.process_index_route(&message.payload).await
                }
                OutboxMessageType::DeleteRouteIndex => {
                    self.process_delete_route_index(&message.payload).await
                }
                OutboxMessageType::VerifyDomain => {
                    // Domain verification is handled by RabbitMQ consumer
                    // Just mark as completed
                    Ok(())
                }
                OutboxMessageType::CheckRouteSafety => {
                    // Safety check is handled externally
                    // Just mark as completed
                    Ok(())
                }
                OutboxMessageType::SendNotification => {
                    // Notifications not implemented yet
                    Ok(())
                }
            };

            match result {
                Ok(()) => {
                    if let Err(e) = self.outbox_repo.mark_completed(message.id).await {
                        error!("Failed to mark message {} as completed: {}", message.id, e);
                    }
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    warn!("Failed to process message {}: {}", message.id, error_msg);
                    if let Err(e) = self.outbox_repo.mark_failed(message.id, &error_msg).await {
                        error!("Failed to mark message {} as failed: {}", message.id, e);
                    }
                }
            }
        }
    }

    /// Process index route message.
    async fn process_index_route(&self, payload: &serde_json::Value) -> anyhow::Result<()> {
        let route_id = payload["route_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing route_id"))?;

        let route_uuid = uuid::Uuid::parse_str(route_id)?;

        let route = self
            .route_repo
            .get_by_id(route_uuid)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Route not found: {}", route_id))?;

        self.search_service.index_route(&route).await?;

        info!("Indexed route: {}", route_id);
        Ok(())
    }

    /// Process delete route index message.
    async fn process_delete_route_index(&self, payload: &serde_json::Value) -> anyhow::Result<()> {
        let route_id = payload["route_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing route_id"))?;

        let route_uuid = uuid::Uuid::parse_str(route_id)?;

        self.search_service.delete_route(route_uuid).await?;

        info!("Deleted route index: {}", route_id);
        Ok(())
    }

    /// Clean up old completed messages.
    async fn cleanup_old_messages(&self) {
        match self.outbox_repo.cleanup_completed(7).await {
            Ok(count) => {
                if count > 0 {
                    info!("Cleaned up {} old outbox messages", count);
                }
            }
            Err(e) => {
                error!("Failed to cleanup old outbox messages: {}", e);
            }
        }
    }
}
