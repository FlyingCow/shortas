use anyhow::Result;
use futures::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Connection, ConnectionProperties, ExchangeKind,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::settings::RabbitMqSettings;

use super::messages::RouteChangedMessage;

pub struct RouteEventConsumer {
    settings: RabbitMqSettings,
    message_tx: mpsc::Sender<RouteChangedMessage>,
}

impl RouteEventConsumer {
    pub fn new(settings: RabbitMqSettings, message_tx: mpsc::Sender<RouteChangedMessage>) -> Self {
        Self {
            settings,
            message_tx,
        }
    }

    pub fn start(self: Arc<Self>) {
        let consumer = self.clone();
        tokio::spawn(async move {
            loop {
                info!("RabbitMQ consumer connecting...");
                match consumer.run_consumer().await {
                    Ok(_) => {
                        warn!("RabbitMQ consumer disconnected, reconnecting...");
                    }
                    Err(e) => {
                        warn!("RabbitMQ consumer error: {}, reconnecting...", e);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    consumer.settings.reconnect_seconds,
                ))
                .await;
            }
        });
    }

    async fn run_consumer(&self) -> Result<()> {
        let conn =
            Connection::connect(&self.settings.uri, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

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

        channel
            .queue_bind(
                queue.name().as_str(),
                &self.settings.route_exchange,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "route-icon-worker",
                BasicConsumeOptions {
                    no_ack: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        info!(
            "RabbitMQ consumer connected, queue: {}",
            queue.name()
        );

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    match serde_json::from_slice::<RouteChangedMessage>(&delivery.data) {
                        Ok(msg) => {
                            info!(
                                "Received route event: {:?} route_id={}",
                                msg.action, msg.route_id
                            );
                            if let Err(e) = self.message_tx.send(msg).await {
                                error!("Failed to send message to worker: {}", e);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to deserialize route changed message: {}", e);
                        }
                    }
                    delivery.ack(BasicAckOptions::default()).await?;
                }
                Err(e) => {
                    warn!("Consumer delivery error: {}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }
}
