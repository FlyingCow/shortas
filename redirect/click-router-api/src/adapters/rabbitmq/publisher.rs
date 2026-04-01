use anyhow::Result;
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use super::{
    messages::{RouteChangedMessage, UserSettingsChangedMessage},
    settings::RabbitMqSettings,
};

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
                    "RabbitMQ publisher connected, exchanges: {}, {}",
                    self.settings.route_exchange, self.settings.user_settings_exchange
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

        // Declare fanout exchanges
        channel
            .exchange_declare(
                &self.settings.route_exchange,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .exchange_declare(
                &self.settings.user_settings_exchange,
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

    pub async fn publish_route_changed(&self, message: &RouteChangedMessage) {
        let payload = match serde_json::to_vec(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to serialize route changed message: {}", e);
                return;
            }
        };

        let properties = BasicProperties::default().with_delivery_mode(1); // non-persistent

        let channel = self.channel.read().await;
        if let Some(ref ch) = *channel {
            if let Err(e) = ch
                .basic_publish(
                    &self.settings.route_exchange,
                    "",
                    BasicPublishOptions::default(),
                    &payload,
                    properties,
                )
                .await
            {
                warn!("Failed to publish route changed message: {}", e);
            }
        } else {
            warn!("RabbitMQ publisher not connected, skipping route changed message");
        }
    }

    /// Publish route changed messages for all routes in a family.
    /// This invalidates cache for master + all child routes.
    pub async fn publish_route_family_changed(
        &self,
        routes: &[crate::model::Route],
        action: super::messages::ChangeAction,
    ) {
        for route in routes {
            self.publish_route_changed(&RouteChangedMessage::from_route(route, action.clone()))
                .await;
        }
    }

    /// Publish route changed messages for all routes in a family with previous destination info.
    /// Used for updates to track destination URL changes.
    pub async fn publish_route_family_updated(
        &self,
        routes: &[crate::model::Route],
        previous_dests: &[Option<String>],
    ) {
        for (route, previous_dest) in routes.iter().zip(previous_dests.iter()) {
            self.publish_route_changed(&RouteChangedMessage::from_route_with_previous(
                route,
                super::messages::ChangeAction::Updated,
                previous_dest.clone(),
            ))
            .await;
        }
    }

    pub async fn publish_user_settings_changed(&self, message: &UserSettingsChangedMessage) {
        let payload = match serde_json::to_vec(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to serialize user settings changed message: {}", e);
                return;
            }
        };

        let properties = BasicProperties::default().with_delivery_mode(1); // non-persistent

        let channel = self.channel.read().await;
        if let Some(ref ch) = *channel {
            if let Err(e) = ch
                .basic_publish(
                    &self.settings.user_settings_exchange,
                    "",
                    BasicPublishOptions::default(),
                    &payload,
                    properties,
                )
                .await
            {
                warn!(
                    "Failed to publish user settings changed message: {}",
                    e
                );
            }
        } else {
            warn!("RabbitMQ publisher not connected, skipping user settings changed message");
        }
    }
}
