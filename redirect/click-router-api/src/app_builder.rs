use anyhow::Result;
use salvo::prelude::*;
use tracing::info;

use crate::adapters;
use crate::core::{CryptoStore, RoutesStore, UserSettingsStore};
use crate::settings::Server as ServerSettings;
use crate::{adapters::api::app_state::AppState, settings::Settings};

#[handler]
async fn app_state_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // The app state will be set by the server setup
    ctrl.call_next(req, depot, res).await;
}

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn RoutesStore + Send + Sync + 'static>>,
    pub(super) crypto_store: Option<Box<dyn CryptoStore + Send + Sync + 'static>>,
    pub(super) user_settings_store: Option<Box<dyn UserSettingsStore + Send + Sync + 'static>>,
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
    ) -> Self {
        Api {
            api_pool: AppState::new(routes_store, crypto_store, user_settings_store),
            settings,
        }
    }

    async fn start_server(self) -> Result<()> {
        let _port = self.settings.port.unwrap_or(8080);

        let router = adapters::api::api_routes::routes();

        let _app_state = self.api_pool.clone();

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

        let router = router
            .hoop(app_state_middleware)
            .unshift(doc.into_router("/api-doc/openapi.json"))
            .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

        let acceptor = TcpListener::new("0.0.0.0:5810").bind().await;

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
        }
    }

    pub fn build(&self) -> Result<Api> {
        info!("{}", "BUILDING");

        let router = Api::new(
            self.settings.server.clone(),
            self.routes_store.clone().unwrap(),
            self.crypto_store.clone().unwrap(),
            self.user_settings_store.clone().unwrap(),
        );

        Ok(router)
    }
}
