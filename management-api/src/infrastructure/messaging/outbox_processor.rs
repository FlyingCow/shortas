//! Outbox processor for reliable message delivery.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::domain::entities::{Notification, OutboxMessageType};
use crate::domain::traits::{InvitationRepository, NotificationRepository, OutboxRepository};
use crate::infrastructure::email::EmailSender;
use crate::infrastructure::email::render_invite_email;
use crate::infrastructure::search::ElasticsearchService;

/// Outbox processor background service.
pub struct OutboxProcessor {
    outbox_repo: Arc<dyn OutboxRepository>,
    search_service: Arc<ElasticsearchService>,
    route_repo: Arc<dyn crate::domain::traits::RouteRepository>,
    notification_repo: Arc<dyn NotificationRepository>,
    invitation_repo: Arc<dyn InvitationRepository>,
    email_sender: Arc<dyn EmailSender>,
    dashboard_base_url: String,
}

impl OutboxProcessor {
    /// Create a new outbox processor.
    pub fn new(
        outbox_repo: Arc<dyn OutboxRepository>,
        search_service: Arc<ElasticsearchService>,
        route_repo: Arc<dyn crate::domain::traits::RouteRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
        invitation_repo: Arc<dyn InvitationRepository>,
        email_sender: Arc<dyn EmailSender>,
        dashboard_base_url: String,
    ) -> Self {
        Self {
            outbox_repo,
            search_service,
            route_repo,
            notification_repo,
            invitation_repo,
            email_sender,
            dashboard_base_url,
        }
    }

    /// Start processing outbox messages.
    pub async fn start(&self, mut shutdown: broadcast::Receiver<()>) {
        let mut process_interval = interval(Duration::from_secs(5));
        let mut cleanup_interval = interval(Duration::from_secs(3600)); // Hourly cleanup
        let mut invitation_expiry_interval = interval(Duration::from_secs(3600)); // Hourly

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
                _ = invitation_expiry_interval.tick() => {
                    self.expire_overdue_invitations().await;
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
                    self.process_send_notification(&message.payload).await
                }
                OutboxMessageType::SendInviteEmail => {
                    self.process_send_invite_email(&message.payload).await
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

    /// Process a notification-center delivery message.
    async fn process_send_notification(&self, payload: &serde_json::Value) -> anyhow::Result<()> {
        let user_id = payload["user_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing user_id"))?;
        let notification_type = payload["notification_type"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing notification_type"))?;
        let inner_payload = payload.get("payload").cloned().unwrap_or(serde_json::json!({}));

        let notification = Notification::new(
            user_id.to_string(),
            notification_type.to_string(),
            inner_payload,
        );
        self.notification_repo.create(&notification).await?;

        info!("Created notification for user {}: {}", user_id, notification_type);
        Ok(())
    }

    /// Process a workspace invitation email message.
    async fn process_send_invite_email(&self, payload: &serde_json::Value) -> anyhow::Result<()> {
        let to_email = payload["to_email"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing to_email"))?;
        let token = payload["token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing token"))?;
        let workspace_name = payload["workspace_name"].as_str().unwrap_or("a workspace");
        let inviter_name = payload["inviter_name"].as_str().unwrap_or("Someone");
        let role = payload["role"].as_str().unwrap_or("Member");

        let invite_url = format!("{}/invite/{}", self.dashboard_base_url.trim_end_matches('/'), token);
        let (subject, html) = render_invite_email(workspace_name, inviter_name, role, &invite_url);

        self.email_sender.send(to_email, &subject, &html).await?;

        info!("Sent invite email to {} for workspace '{}'", to_email, workspace_name);
        Ok(())
    }

    /// Expire any pending invitations past their expiry date.
    async fn expire_overdue_invitations(&self) {
        match self.invitation_repo.expire_overdue().await {
            Ok(count) => {
                if count > 0 {
                    info!("Expired {} overdue workspace invitations", count);
                }
            }
            Err(e) => {
                error!("Failed to expire overdue invitations: {}", e);
            }
        }
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
