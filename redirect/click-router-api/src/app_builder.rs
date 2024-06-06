use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use tracing::info;

use crate::adapters;
use crate::core::{BaseCryptoStore, BaseUserSettingsStore};
use crate::settings::Server;
use crate::{adapters::api::app_state::AppState, core::BaseRoutesStore, settings::Settings};

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn BaseRoutesStore + Send + Sync + 'static>>,
    pub(super) crypto_store: Option<Box<dyn BaseCryptoStore + Send + Sync + 'static>>,
    pub(super) user_settings_store: Option<Box<dyn BaseUserSettingsStore + Send + Sync + 'static>>,
}

#[derive(Clone)]
pub struct Api {
    pub settings: Server,
    pub api_pool: AppState,
}

impl Api {
    fn new(
        settings: Server,
        routes_store: Box<dyn BaseRoutesStore + Send + Sync>,
        crypto_store: Box<dyn BaseCryptoStore + Send + Sync>,
        user_settings_store: Box<dyn BaseUserSettingsStore + Send + Sync>,
    ) -> Self {
        Api {
            api_pool: AppState::new(routes_store, crypto_store, user_settings_store),
            settings,
        }
    }

    async fn start_server(self) -> Result<()> {
        let port = self.settings.port.unwrap_or(8080);

        info!("Server running on port {}", port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(self.api_pool.clone()))
                .wrap(Logger::default())
                .configure(adapters::api::api_routes::routes)
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await?;

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
        env_logger::try_init()?;
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
