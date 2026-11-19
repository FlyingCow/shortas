use anyhow::Result;
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};
use std::future::Future;
use tracing::{error, info, warn};

use crate::settings::RabbitMqSettings;

/// Configuration for a RabbitMQ consumer
#[derive(Clone)]
pub struct ConsumerConfig {
    pub exchange: String,
    pub routing_key: String,
    pub consumer_tag: String,
}

/// Generic RabbitMQ consumer that handles connection management and message delivery
pub struct RabbitMqConsumer {
    settings: RabbitMqSettings,
    config: ConsumerConfig,
}

impl RabbitMqConsumer {
    pub fn new(settings: RabbitMqSettings, config: ConsumerConfig) -> Self {
        Self { settings, config }
    }

    /// Run the consumer with automatic reconnection
    ///
    /// The handler function is called for each message received.
    /// If the handler returns Ok(()), the message is acknowledged.
    /// If the handler returns Err, the error is logged but the message is still acknowledged.
    pub async fn run<F, Fut>(&self, handler: F) -> Result<()>
    where
        F: Fn(Vec<u8>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<()>> + Send,
    {
        info!("RabbitMQ consumer starting...");

        loop {
            if let Err(e) = self.consume(&handler).await {
                warn!("RabbitMQ consumer error: {}, reconnecting...", e);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(
                self.settings.reconnect_seconds,
            ))
            .await;
        }
    }

    async fn consume<F, Fut>(&self, handler: &F) -> Result<()>
    where
        F: Fn(Vec<u8>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<()>> + Send,
    {
        info!("Connecting to RabbitMQ...");

        let conn =
            Connection::connect(&self.settings.uri, ConnectionProperties::default()).await?;

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

        // Bind to exchange
        channel
            .queue_bind(
                queue.name().as_str(),
                &self.config.exchange,
                &self.config.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "Listening on exchange: {}, routing_key: {}",
            self.config.exchange, self.config.routing_key
        );

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                &self.config.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    if let Err(e) = handler(delivery.data.clone()).await {
                        error!("Error handling message: {}", e);
                    }

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
}
