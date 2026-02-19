use anyhow::Result;
use mongodb::Client;
use salvo::prelude::*;
use std::sync::Arc;
use tracing::{info, warn};

use crate::adapters;
use crate::adapters::api::app_state::AppState;
use crate::adapters::click_router_api::ClickRouterApiClient;
use crate::adapters::mongodb::MongodbRouteStore;
use crate::adapters::rabbitmq::RabbitMqPublisher;
use crate::adapters::safe_browsing::SafeBrowsingClient;
use crate::core::RouteStore;
use crate::settings::Settings;
use crate::worker::VerificationWorker;

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) route_store: Option<Box<dyn RouteStore + Send + Sync + 'static>>,
    pub(super) rabbitmq_publisher: Option<RabbitMqPublisher>,
    pub(super) safe_browsing_client: Option<SafeBrowsingClient>,
    pub(super) click_router_api_client: Option<ClickRouterApiClient>,
}

pub struct Api {
    pub settings: Settings,
    pub app_state: AppState,
}

impl Api {
    fn new(
        settings: Settings,
        route_store: Box<dyn RouteStore + Send + Sync>,
        rabbitmq_publisher: Option<RabbitMqPublisher>,
        safe_browsing_client: SafeBrowsingClient,
        click_router_api_client: ClickRouterApiClient,
    ) -> Self {
        Api {
            app_state: AppState::new(
                route_store,
                rabbitmq_publisher,
                safe_browsing_client,
                click_router_api_client,
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

        let doc = OpenApi::new("Route Verifier API", "0.1.0")
            .merge_router(&router)
            .info(
                salvo::oapi::Info::new("Route Verifier API", "0.1.0")
                    .description("Route safety verification service using Google Safe Browsing")
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
            route_store: None,
            rabbitmq_publisher: None,
            safe_browsing_client: None,
            click_router_api_client: None,
        }
    }

    pub async fn with_mongodb(&mut self) -> &mut Self {
        match Client::with_uri_str(&self.settings.mongodb.connection_string).await {
            Ok(client) => {
                let database = client.database(&self.settings.mongodb.database_name);
                let store = MongodbRouteStore::new(&database, &self.settings.mongodb.collection);
                self.route_store = Some(Box::new(store));
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

    pub fn with_safe_browsing_client(&mut self) -> &mut Self {
        let client = SafeBrowsingClient::new(&self.settings.safe_browsing);
        self.safe_browsing_client = Some(client);
        info!("Safe Browsing client initialized");
        self
    }

    pub fn with_click_router_api_client(&mut self) -> &mut Self {
        let client = ClickRouterApiClient::new(&self.settings.click_router_api);
        self.click_router_api_client = Some(client);
        info!("Click Router API client initialized");
        self
    }

    pub fn build(&self) -> Result<Api> {
        info!("Building route-verifier application");

        let route_store = self
            .route_store
            .clone()
            .expect("Route store not initialized");

        let safe_browsing_client = self
            .safe_browsing_client
            .clone()
            .expect("Safe Browsing client not initialized");

        let click_router_api_client = self
            .click_router_api_client
            .clone()
            .expect("Click Router API client not initialized");

        let api = Api::new(
            self.settings.clone(),
            route_store,
            self.rabbitmq_publisher.clone(),
            safe_browsing_client,
            click_router_api_client,
        );

        Ok(api)
    }
}
