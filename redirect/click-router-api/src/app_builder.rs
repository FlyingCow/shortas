use anyhow::Result;
use salvo::prelude::*;
use std::sync::Arc;
use tracing::{info, warn};

use crate::adapters;
use crate::adapters::rabbitmq::publisher::RabbitMqPublisher;
use crate::core::{ChallengeStore, CryptoStore, RoutesStore, UserSettingsStore};
use crate::settings::Server as ServerSettings;
use crate::{adapters::api::app_state::AppState, settings::Settings};


#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn RoutesStore + Send + Sync + 'static>>,
    pub(super) crypto_store: Option<Box<dyn CryptoStore + Send + Sync + 'static>>,
    pub(super) user_settings_store: Option<Box<dyn UserSettingsStore + Send + Sync + 'static>>,
    pub(super) challenge_store: Option<Box<dyn ChallengeStore + Send + Sync + 'static>>,
    pub(super) rabbitmq_publisher: Option<RabbitMqPublisher>,
}

#[derive(Clone)]
pub struct Api {
    pub settings: ServerSettings,
    pub api_pool: AppState,
}

impl Api {
    fn new(
        settings: ServerSettings,
        routes_store: Box<dyn RoutesStore + Send + Sync>,
        crypto_store: Box<dyn CryptoStore + Send + Sync>,
        user_settings_store: Box<dyn UserSettingsStore + Send + Sync>,
        challenge_store: Box<dyn ChallengeStore + Send + Sync>,
        rabbitmq_publisher: Option<RabbitMqPublisher>,
    ) -> Self {
        Api {
            api_pool: AppState::new(routes_store, crypto_store, user_settings_store, challenge_store, rabbitmq_publisher),
            settings,
        }
    }

    async fn start_server(self) -> Result<()> {
        let port = self.settings.port.unwrap_or(8080);
        info!("Starting server on port {}", port);

        let router = adapters::api::api_routes::routes();

        let app_state = self.api_pool.clone();

        let doc = OpenApi::new("Click Router API", "0.1.0")
            .merge_router(&router)
            .add_security_scheme("Bearer", salvo::oapi::security::ApiKey::Header(salvo::oapi::security::ApiKeyValue::new("Authorization")))
            .add_security_scheme("RPT", salvo::oapi::security::ApiKey::Header(salvo::oapi::security::ApiKeyValue::new("Authorization")))
            .info(
                salvo::oapi::Info::new("Click Router API", "0.1.0")
                    .description("A high-performance click aggregation API with JWT authentication via Keycloak")
                    .contact(salvo::oapi::Contact::new().name("API Support").email("support@example.com"))
                    .license(salvo::oapi::License::new("MIT"))
            )
            ;

        let app_state_arc = Arc::new(app_state);

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

        let router = router
            .hoop(AppStateInjector { state: app_state_arc })
            .unshift(doc.into_router("/api-doc/openapi.json"))
            .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

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
            routes_store: None,
            crypto_store: None,
            user_settings_store: None,
            challenge_store: None,
            rabbitmq_publisher: None,
        }
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

    pub fn build(&self) -> Result<Api> {
        info!("{}", "BUILDING");

        let router = Api::new(
            self.settings.server.clone(),
            self.routes_store.clone().unwrap(),
            self.crypto_store.clone().unwrap(),
            self.user_settings_store.clone().unwrap(),
            self.challenge_store.clone().unwrap(),
            self.rabbitmq_publisher.clone(),
        );

        Ok(router)
    }
}
