//! Publishes `PublishNotificationEvent` outbox payloads onto the
//! notifications exchange for `notification-worker` to consume.

use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions},
    BasicProperties, Connection, ConnectionProperties,
};
use tracing::info;

use crate::settings::RabbitMqSettings;

use super::notification_topology::declare_notification_topology;

pub struct NotificationPublisher {
    channel: lapin::Channel,
    exchange: String,
}

impl NotificationPublisher {
    pub async fn new(settings: &RabbitMqSettings) -> anyhow::Result<Self> {
        let connection =
            Connection::connect(&settings.url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;

        declare_notification_topology(&channel, settings).await?;

        info!(
            "NotificationPublisher connected, publishing to exchange '{}'",
            settings.notifications_exchange
        );

        Ok(Self {
            channel,
            exchange: settings.notifications_exchange.clone(),
        })
    }

    /// Publish an already-serialized `NotificationEvent` (an outbox
    /// message's payload). Routing key is the event's `notification_id`.
    pub async fn publish(&self, event_payload: &serde_json::Value) -> anyhow::Result<()> {
        let notification_id = event_payload["notification_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing notification_id"))?;

        let body = serde_json::to_vec(event_payload)?;

        self.channel
            .basic_publish(
                &self.exchange,
                notification_id,
                BasicPublishOptions::default(),
                &body,
                BasicProperties::default().with_delivery_mode(2), // persistent
            )
            .await?
            .await?; // wait for broker confirm

        Ok(())
    }
}
