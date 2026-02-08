use crate::adapters::rabbitmq::RabbitMqPublisher;
use crate::core::DomainStore;
use crate::dns::DnsVerifier;
use crate::settings::DnsSettings;

#[derive(Clone)]
pub struct AppState {
    pub domain_store: Box<dyn DomainStore + Send + Sync>,
    pub rabbitmq_publisher: Option<RabbitMqPublisher>,
    pub dns_verifier: DnsVerifier,
    pub dns_settings: DnsSettings,
}

impl AppState {
    pub fn new(
        domain_store: Box<dyn DomainStore + Send + Sync>,
        rabbitmq_publisher: Option<RabbitMqPublisher>,
        dns_verifier: DnsVerifier,
        dns_settings: DnsSettings,
    ) -> Self {
        AppState {
            domain_store,
            rabbitmq_publisher,
            dns_verifier,
            dns_settings,
        }
    }
}
