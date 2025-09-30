use anyhow::Result;
use salvo::prelude::*;
use tracing::info;

use crate::adapters;
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
}

#[derive(Clone)]
pub struct Api {
    pub settings: ServerSettings,
    pub api_pool: AppState,
}

impl Api {
    fn new(settings: Settings) -> Result<Self> {
        Ok(Api {
            api_pool: AppState::new(&settings)?,
            settings: settings.server,
        })
    }

    async fn start_server(self) -> Result<()> {
        let _port = self.settings.port.unwrap_or(8080);

        let router = adapters::api::api_routes::routes();

        let _app_state = self.api_pool.clone();

        let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

        let router = router
            .hoop(app_state_middleware)
            .unshift(doc.into_router("/api-doc/openapi.json"))
            .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

        let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

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
        }
    }

    pub fn build(&self) -> Result<Api> {
        info!("{}", "BUILDING");

        Api::new(self.settings.clone())
    }
}