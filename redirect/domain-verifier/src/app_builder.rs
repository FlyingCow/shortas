use anyhow::Result;
use mongodb::Client;
use salvo::prelude::*;
use std::sync::Arc;
use tracing::{info, warn};

use crate::adapters;
use crate::adapters::api::app_state::AppState;
use crate::adapters::mongodb::MongodbDomainStore;
use crate::adapters::rabbitmq::RabbitMqPublisher;
use crate::core::DomainStore;
use crate::dns::DnsVerifier;
use crate::settings::Settings;
use crate::worker::VerificationWorker;

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) domain_store: Option<Box<dyn DomainStore + Send + Sync + 'static>>,
    pub(super) rabbitmq_publisher: Option<RabbitMqPublisher>,
    pub(super) dns_verifier: Option<DnsVerifier>,
}

pub struct Api {
    pub settings: Settings,
    pub app_state: AppState,
}

impl Api {
    fn new(
        settings: Settings,
        domain_store: Box<dyn DomainStore + Send + Sync>,
        rabbitmq_publisher: Option<RabbitMqPublisher>,
        dns_verifier: DnsVerifier,
    ) -> Self {
        Api {
            app_state: AppState::new(
                domain_store,
                rabbitmq_publisher,
                dns_verifier,
                settings.dns.clone(),
            ),
            settings,
        }
    }

    async fn start_server(self) -> Result<()> {
        let port = self.settings.server.port;
        info!("Starting server on port {}", port);

        let router = adapters::api::api_routes::routes();

        let app_state = self.app_state.clone();
        let app_state_arc = Arc::new(app_state.clone());

        // Create a handler to inject app_state
        struct AppStateInjector {
            state: Arc<AppState>,
        }

        #[async_trait]
        impl Handler for AppStateInjector {
            async fn handle(
                &self,
                _req: &mut Request,
                depot: &mut Depot,
                _res: &mut Response,
                ctrl: &mut FlowCtrl,
            ) {
                depot.inject(self.state.clone());
                ctrl.call_next(_req, depot, _res).await;
            }
        }

        let doc = OpenApi::new("Domain Verifier API", "0.1.0")
            .merge_router(&router)
            .info(
                salvo::oapi::Info::new("Domain Verifier API", "0.1.0")
                    .description("Domain verification microservice with DNS-based ownership verification")
                    .contact(salvo::oapi::Contact::new().name("API Support").email("support@example.com"))
                    .license(salvo::oapi::License::new("MIT")),
            );

        let router = router
            .hoop(AppStateInjector { state: app_state_arc.clone() })
            .unshift(doc.into_router("/api-doc/openapi.json"))
            .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

        // Start verification worker in background
        let worker = VerificationWorker::new(app_state_arc.clone(), self.settings.worker.clone());
        tokio::spawn(async move {
            worker.run().await;
        });

        let bind_address = format!("0.0.0.0:{}", port);
        info!("Binding to {}", bind_address);
        let acceptor = TcpListener::new(bind_address).bind().await;

        Server::new(acceptor).serve(router).await;

        Ok(())
    }

    pub async fn run(self) -> Result<()> {
        self.start_server().await?;
        Ok(())
    }
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            domain_store: None,
            rabbitmq_publisher: None,
            dns_verifier: None,
        }
    }

    pub async fn with_mongodb(&mut self) -> &mut Self {
        match Client::with_uri_str(&self.settings.mongodb.connection_string).await {
            Ok(client) => {
                let database = client.database(&self.settings.mongodb.database_name);
                let store = MongodbDomainStore::new(&database, &self.settings.mongodb.collection);
                self.domain_store = Some(Box::new(store));
                info!("MongoDB connected successfully");
            }
            Err(e) => {
                warn!("Failed to connect to MongoDB: {}", e);
            }
        }
        self
    }

    pub async fn with_rabbitmq(&mut self) -> &mut Self {
        if let Some(ref rmq_settings) = self.settings.rabbitmq {
            match RabbitMqPublisher::new(rmq_settings).await {
                Ok(publisher) => {
                    self.rabbitmq_publisher = Some(publisher);
                }
                Err(e) => {
                    warn!("Failed to connect to RabbitMQ, continuing without publisher: {}", e);
                }
            }
        } else {
            info!("RabbitMQ settings not configured, skipping publisher");
        }
        self
    }

    pub fn with_dns_verifier(&mut self) -> &mut Self {
        match DnsVerifier::new(&self.settings.dns) {
            Ok(verifier) => {
                self.dns_verifier = Some(verifier);
                info!("DNS verifier initialized");
            }
            Err(e) => {
                warn!("Failed to create DNS verifier: {}", e);
            }
        }
        self
    }

    pub fn build(&self) -> Result<Api> {
        info!("Building domain-verifier application");

        let domain_store = self
            .domain_store
            .clone()
            .expect("Domain store not initialized");

        let dns_verifier = self
            .dns_verifier
            .clone()
            .expect("DNS verifier not initialized");

        let api = Api::new(
            self.settings.clone(),
            domain_store,
            self.rabbitmq_publisher.clone(),
            dns_verifier,
        );

        Ok(api)
    }
}
