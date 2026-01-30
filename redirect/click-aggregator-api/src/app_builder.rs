use anyhow::Result;
use salvo::prelude::*;
use tracing::info;

use crate::adapters;
use crate::settings::Server as ServerSettings;
use crate::{adapters::api::app_state::AppState, settings::Settings};

#[derive(Clone)]
pub struct AppStateMiddleware {
    app_state: AppState,
}

impl AppStateMiddleware {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

#[async_trait]
impl Handler for AppStateMiddleware {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        depot.inject(self.app_state.clone());
        ctrl.call_next(req, depot, res).await;
    }
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
        let port = self.settings.port.unwrap_or(8080);

        let router = adapters::api::api_routes::routes();

        let app_state = self.api_pool.clone();

        let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

        let router = router
            .hoop(AppStateMiddleware::new(app_state))
            .unshift(doc.into_router("/api-doc/openapi.json"))
            .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));


        let bind_address = format!("0.0.0.0:{}", port);

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
        Self { settings }
    }

    pub fn build(&self) -> Result<Api> {
        info!("{}", "BUILDING");

        Api::new(self.settings.clone())
    }
}
