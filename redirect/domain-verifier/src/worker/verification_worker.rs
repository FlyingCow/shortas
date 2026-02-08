use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio::time::interval;
use tracing::{error, info};

use crate::adapters::api::app_state::AppState;
use crate::adapters::rabbitmq::messages::DomainStateChangedMessage;
use crate::model::VerificationStatus;
use crate::settings::WorkerSettings;

pub struct VerificationWorker {
    app_state: Arc<AppState>,
    settings: WorkerSettings,
}

impl VerificationWorker {
    pub fn new(app_state: Arc<AppState>, settings: WorkerSettings) -> Self {
        Self { app_state, settings }
    }

    pub async fn run(self) {
        info!(
            "Verification worker started (interval: {}s, batch size: {})",
            self.settings.check_interval_seconds, self.settings.batch_size
        );

        let mut ticker =
            interval(std::time::Duration::from_secs(self.settings.check_interval_seconds));

        loop {
            ticker.tick().await;
            self.process_batch().await;
        }
    }

    async fn process_batch(&self) {
        let now = Utc::now();

        let domains = match self
            .app_state
            .domain_store
            .get_domains_for_verification(now, self.settings.batch_size)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to get domains for verification: {}", e);
                return;
            }
        };

        if domains.is_empty() {
            return;
        }

        info!("Processing {} domains for verification", domains.len());

        for mut domain in domains {
            let result = self.app_state.dns_verifier.verify(&domain).await;

            let previous_status = domain.status.clone();
            domain.status = result.status.clone();
            domain.verification_reason = result.reason.clone();
            domain.last_check_at = Some(Utc::now());

            // Set next check time based on result
            let recheck_minutes = match result.status {
                VerificationStatus::Verified => self.settings.recheck_interval_minutes,
                VerificationStatus::Failed | VerificationStatus::Pending => {
                    self.settings.failed_recheck_interval_minutes
                }
            };
            domain.next_check_at = Some(Utc::now() + Duration::minutes(recheck_minutes));

            // Update domain in store
            if let Err(e) = self.app_state.domain_store.update_domain(&domain).await {
                error!("Failed to update domain {}: {}", domain.id, e);
                continue;
            }

            // Publish state change if status changed
            if previous_status != domain.status {
                if let Some(ref publisher) = self.app_state.rabbitmq_publisher {
                    publisher
                        .publish_domain_state_changed(&DomainStateChangedMessage {
                            domain_id: domain.id.clone(),
                            domain_name: domain.name.clone(),
                            owner_id: domain.owner_id.clone(),
                            status: domain.status.clone(),
                            verification_reason: domain.verification_reason.clone(),
                            last_check_at: domain.last_check_at.map(|dt| dt.timestamp_millis()),
                            next_check_at: domain.next_check_at.map(|dt| dt.timestamp_millis()),
                        })
                        .await;
                }

                info!(
                    "Domain {} ({}) verification: {:?} -> {:?}",
                    domain.name, domain.id, previous_status, domain.status
                );
            }
        }
    }
}
