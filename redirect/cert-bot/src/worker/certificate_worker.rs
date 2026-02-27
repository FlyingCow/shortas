use anyhow::Result;
use chrono::Utc;
use instant_acme::{Account, AuthorizationStatus, ChallengeType, Identifier, NewAccount, NewOrder, OrderStatus as AcmeOrderStatus};
use rcgen::{CertificateParams, DistinguishedName, KeyPair};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

use crate::adapters::click_router_api::ClickRouterApiClient;
use crate::adapters::mongodb::MongodbOrderStore;
use crate::core::OrderStore;
use crate::model::{CertificateOrder, OrderStatus};
use crate::settings::Settings;

pub struct CertificateWorker {
    settings: Settings,
    order_store: Arc<MongodbOrderStore>,
    api_client: Arc<ClickRouterApiClient>,
    account: Option<Account>,
}

impl CertificateWorker {
    pub async fn new(settings: Settings) -> Result<Self> {
        let order_store = Arc::new(MongodbOrderStore::new(&settings.mongodb).await?);
        let api_client = Arc::new(ClickRouterApiClient::new(&settings.click_router_api)?);

        Ok(Self {
            settings,
            order_store,
            api_client,
            account: None,
        })
    }

    /// Initialize ACME account
    async fn init_account(&mut self) -> Result<()> {
        info!("Initializing ACME account with Let's Encrypt");

        let (account, _credentials) = Account::create(
            &NewAccount {
                contact: &[&format!("mailto:{}", self.settings.acme.account_email)],
                terms_of_service_agreed: true,
                only_return_existing: false,
            },
            &self.settings.acme.directory_url,
            None,
        )
        .await?;

        info!("ACME account created/loaded successfully");
        self.account = Some(account);

        Ok(())
    }

    pub async fn run(mut self) -> Result<()> {
        // Initialize ACME account
        self.init_account().await?;

        info!(
            "Certificate worker started (interval: {}s)",
            self.settings.worker.check_interval_seconds
        );

        let mut ticker = interval(Duration::from_secs(self.settings.worker.check_interval_seconds));

        loop {
            ticker.tick().await;

            // Process pending orders
            if let Err(e) = self.process_pending_orders().await {
                error!("Error processing pending orders: {}", e);
            }

            // Check challenge validation for orders waiting
            if let Err(e) = self.check_challenge_validation().await {
                error!("Error checking challenge validation: {}", e);
            }
        }
    }

    async fn process_pending_orders(&self) -> Result<()> {
        let orders = self
            .order_store
            .get_orders_by_status(OrderStatus::Pending, self.settings.worker.batch_size)
            .await?;

        for mut order in orders {
            info!("Processing pending order for domain: {}", order.domain);

            match self.process_order(&mut order).await {
                Ok(_) => {
                    info!("Order processed successfully for domain: {}", order.domain);
                }
                Err(e) => {
                    error!("Failed to process order for domain {}: {}", order.domain, e);
                    order.error_message = Some(e.to_string());
                    order.retry_count += 1;
                    order.updated_at = Utc::now();

                    if order.retry_count >= order.max_retries {
                        order.status = OrderStatus::Failed;
                        warn!("Order for domain {} failed after {} retries", order.domain, order.retry_count);
                    } else {
                        order.next_retry_at = Some(Utc::now() + chrono::Duration::minutes(5));
                    }

                    let _ = self.order_store.update_order(&order).await;
                }
            }
        }

        Ok(())
    }

    async fn process_order(&self, order: &mut CertificateOrder) -> Result<()> {
        let account = self.account.as_ref().ok_or_else(|| anyhow::anyhow!("ACME account not initialized"))?;

        // Create ACME order
        let identifier = Identifier::Dns(order.domain.clone());
        let mut acme_order = account
            .new_order(&NewOrder {
                identifiers: &[identifier],
            })
            .await?;

        order.acme_order_url = Some(acme_order.url().to_string());

        // Get authorization
        let authorizations = acme_order.authorizations().await?;
        let authz = authorizations
            .first()
            .ok_or_else(|| anyhow::anyhow!("No authorization found"))?;

        // Note: Authorization URL is part of the order, not individual authz

        // Find HTTP-01 challenge
        let challenge = authz
            .challenges
            .iter()
            .find(|c| c.r#type == ChallengeType::Http01)
            .ok_or_else(|| anyhow::anyhow!("HTTP-01 challenge not available"))?;

        // Get key authorization
        let key_authorization = acme_order.key_authorization(challenge).as_str().to_string();

        // Store challenge via click-router-api
        self.api_client
            .store_challenge(&order.domain, &challenge.token, &key_authorization)
            .await?;

        info!("Challenge stored for domain: {}, token: {}", order.domain, challenge.token);

        // Tell ACME we're ready
        acme_order.set_challenge_ready(&challenge.url).await?;

        // Update order status
        order.status = OrderStatus::ChallengeCreated;
        order.updated_at = Utc::now();
        self.order_store.update_order(order).await?;

        Ok(())
    }

    async fn check_challenge_validation(&self) -> Result<()> {
        let orders = self
            .order_store
            .get_orders_by_status(OrderStatus::ChallengeCreated, self.settings.worker.batch_size)
            .await?;

        for mut order in orders {
            match self.check_and_finalize_order(&mut order).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error checking order for domain {}: {}", order.domain, e);
                    order.error_message = Some(e.to_string());
                    order.retry_count += 1;
                    order.updated_at = Utc::now();

                    if order.retry_count >= order.max_retries {
                        order.status = OrderStatus::Failed;
                    } else {
                        order.next_retry_at = Some(Utc::now() + chrono::Duration::minutes(5));
                    }

                    let _ = self.order_store.update_order(&order).await;
                }
            }
        }

        Ok(())
    }

    async fn check_and_finalize_order(&self, order: &mut CertificateOrder) -> Result<()> {
        let account = self.account.as_ref().ok_or_else(|| anyhow::anyhow!("ACME account not initialized"))?;

        // Get the existing order
        let order_url = order.acme_order_url.as_ref().ok_or_else(|| anyhow::anyhow!("No ACME order URL"))?;
        let mut acme_order = account.order(order_url.to_string()).await?;

        // Check order status
        let acme_status = acme_order.state().status;

        match acme_status {
            AcmeOrderStatus::Ready => {
                info!("Order ready for finalization, domain: {}", order.domain);

                // Generate CSR
                let mut params = CertificateParams::default();
                params.distinguished_name = DistinguishedName::new();
                params.subject_alt_names = vec![rcgen::SanType::DnsName(order.domain.clone().try_into()?)];

                let key_pair = KeyPair::generate()?;
                let csr = params.serialize_request(&key_pair)?;

                // Finalize order
                acme_order.finalize(csr.der()).await?;

                // Wait for certificate
                let cert_chain_pem = loop {
                    match acme_order.certificate().await? {
                        Some(cert) => break cert,
                        None => {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    }
                };

                // Store certificate via click-router-api
                let private_key_pem = key_pair.serialize_pem();
                self.api_client
                    .store_certificate(&order.domain, &private_key_pem, &cert_chain_pem)
                    .await?;

                // Cleanup challenge
                let _ = self.api_client.delete_domain_challenges(&order.domain).await;

                order.status = OrderStatus::Valid;
                order.updated_at = Utc::now();
                self.order_store.update_order(order).await?;

                info!("Certificate issued successfully for domain: {}", order.domain);
            }
            AcmeOrderStatus::Invalid => {
                order.status = OrderStatus::Failed;
                order.error_message = Some("ACME order became invalid".into());
                order.updated_at = Utc::now();
                self.order_store.update_order(order).await?;

                warn!("ACME order invalid for domain: {}", order.domain);
            }
            AcmeOrderStatus::Pending => {
                // Check authorization status
                let authorizations = acme_order.authorizations().await?;
                if let Some(authz) = authorizations.first() {
                    match authz.status {
                        AuthorizationStatus::Invalid => {
                            order.status = OrderStatus::Failed;
                            order.error_message = Some("Authorization invalid - challenge verification failed".into());
                            order.updated_at = Utc::now();
                            self.order_store.update_order(order).await?;

                            warn!("Authorization invalid for domain: {}", order.domain);
                        }
                        _ => {
                            // Still pending, will check again next iteration
                            info!("Order still pending for domain: {}", order.domain);
                        }
                    }
                }
            }
            _ => {
                // Processing or other state, will check again next iteration
                info!("Order in state {:?} for domain: {}", acme_status, order.domain);
            }
        }

        Ok(())
    }
}
