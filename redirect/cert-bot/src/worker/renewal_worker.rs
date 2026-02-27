use anyhow::Result;
use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio::time::{interval, Duration as TokioDuration};
use tracing::{error, info, warn};

use crate::adapters::click_router_api::ClickRouterApiClient;
use crate::adapters::mongodb::MongodbOrderStore;
use crate::core::OrderStore;
use crate::model::{CertificateOrder, OrderStatus};
use crate::settings::Settings;

/// Worker that checks for certificates nearing expiration and creates renewal orders
pub struct RenewalWorker {
    settings: Settings,
    order_store: Arc<MongodbOrderStore>,
    api_client: Arc<ClickRouterApiClient>,
}

impl RenewalWorker {
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
        info!(
            "Renewal worker started (interval: {}h, renew {}d before expiry)",
            self.settings.renewal.check_interval_hours,
            self.settings.renewal.renewal_days_before
        );

        let mut ticker = interval(TokioDuration::from_secs(
            self.settings.renewal.check_interval_hours * 3600,
        ));

        loop {
            ticker.tick().await;

            if let Err(e) = self.check_renewals().await {
                error!("Error checking renewals: {}", e);
            }
        }
    }

    async fn check_renewals(&self) -> Result<()> {
        info!("Checking for certificates that need renewal...");

        // Get all certificates expiring within the renewal window
        let renewal_threshold =
            Utc::now() + Duration::days(self.settings.renewal.renewal_days_before);

        // Fetch certificates expiring soon from click-router-api
        let expiring_domains = self
            .api_client
            .get_certificates_expiring_before(renewal_threshold)
            .await?;

        info!(
            "Found {} certificates expiring before {}",
            expiring_domains.len(),
            renewal_threshold
        );

        for domain_info in expiring_domains {
            // Check if there's already a pending order for this domain
            if let Ok(Some(_)) = self
                .order_store
                .get_active_order_for_domain(&domain_info.domain)
                .await
            {
                info!(
                    "Active order already exists for domain: {}, skipping",
                    domain_info.domain
                );
                continue;
            }

            // Create renewal order
            let order = CertificateOrder::new(
                domain_info.domain.clone(),
                domain_info.owner_id.unwrap_or_default(),
                self.settings.worker.max_retries,
            );

            match self.order_store.store_order(&order).await {
                Ok(_) => {
                    info!(
                        "Created renewal order for domain: {} (expires: {:?})",
                        domain_info.domain, domain_info.expires_at
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to create renewal order for {}: {}",
                        domain_info.domain, e
                    );
                }
            }
        }

        Ok(())
    }
}
