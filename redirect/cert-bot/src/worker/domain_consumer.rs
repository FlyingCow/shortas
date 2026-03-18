use anyhow::Result;
use chrono::{Duration, Utc};
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::adapters::click_router_api::ClickRouterApiClient;
use crate::adapters::mongodb::MongodbOrderStore;
use crate::adapters::rabbitmq::{ConsumerConfig, DomainStateChangedMessage, RabbitMqConsumer};
use crate::core::OrderStore;
use crate::model::CertificateOrder;
use crate::settings::Settings;

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

        let consumer = RabbitMqConsumer::new(
            self.settings.rabbitmq.clone(),
            ConsumerConfig {
                exchange: self.settings.rabbitmq.domain_exchange.clone(),
                routing_key: String::new(),
                consumer_tag: "cert-bot-domain-consumer".to_string(),
            },
        );

        let handler = DomainMessageHandler {
            settings: self.settings,
            order_store: self.order_store,
            api_client: self.api_client,
        };

        let handler = Arc::new(handler);

        consumer
            .run(move |data| {
                let handler = Arc::clone(&handler);
                async move { handler.handle_message(&data).await }
            })
            .await
    }
}

struct DomainMessageHandler {
    settings: Settings,
    order_store: Arc<MongodbOrderStore>,
    api_client: Arc<ClickRouterApiClient>,
}

impl DomainMessageHandler {
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
