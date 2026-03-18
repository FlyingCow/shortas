use crate::adapters::rabbitmq::RabbitMqPublisher;
use crate::adapters::safe_browsing::SafeBrowsingClient;
use crate::core::RouteStore;

#[derive(Clone)]
pub struct AppState {
    pub route_store: Box<dyn RouteStore + Send + Sync>,
    pub rabbitmq_publisher: Option<RabbitMqPublisher>,
    pub safe_browsing_client: SafeBrowsingClient,
}

impl AppState {
    pub fn new(
        route_store: Box<dyn RouteStore + Send + Sync>,
        rabbitmq_publisher: Option<RabbitMqPublisher>,
        safe_browsing_client: SafeBrowsingClient,
    ) -> Self {
        AppState {
            route_store,
            rabbitmq_publisher,
            safe_browsing_client,
        }
    }
}
