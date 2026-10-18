//! Application state and middleware for dependency injection.

use async_trait::async_trait;
use salvo::prelude::*;
use sqlx::PgPool;
use std::sync::Arc;

use crate::application::services::RouteService;
use crate::domain::traits::{
    CertificateRepository, DomainRepository, OutboxRepository, RouteRepository, WorkspaceRepository,
};
use crate::infrastructure::{
    http_clients::{ClickAggregatorClient, ClickRouterClient},
    repositories::{
        PgCertificateRepository, PgDomainRepository, PgOutboxRepository, PgRouteRepository,
        PgWorkspaceRepository,
    },
    search::ElasticsearchService,
    storage::MinioService,
};
use crate::settings::Settings;

/// Application state containing all services and repositories.
#[derive(Clone)]
pub struct AppState {
    pub route_repo: Arc<dyn RouteRepository>,
    pub domain_repo: Arc<dyn DomainRepository>,
    pub workspace_repo: Arc<dyn WorkspaceRepository>,
    pub certificate_repo: Arc<dyn CertificateRepository>,
    pub outbox_repo: Arc<dyn OutboxRepository>,
    pub route_service: Arc<RouteService>,
    pub click_router: Arc<ClickRouterClient>,
    pub click_aggregator: Arc<ClickAggregatorClient>,
    pub search_service: Arc<ElasticsearchService>,
    pub storage_service: Arc<MinioService>,
    pub settings: Settings,
}

impl AppState {
    /// Create a new application state with all dependencies.
    pub async fn new(settings: Settings, pool: PgPool) -> anyhow::Result<Self> {
        // Create repositories
        let route_repo: Arc<dyn RouteRepository> = Arc::new(PgRouteRepository::new(pool.clone()));
        let domain_repo: Arc<dyn DomainRepository> = Arc::new(PgDomainRepository::new(pool.clone()));
        let workspace_repo: Arc<dyn WorkspaceRepository> =
            Arc::new(PgWorkspaceRepository::new(pool.clone()));
        let certificate_repo: Arc<dyn CertificateRepository> =
            Arc::new(PgCertificateRepository::new(pool.clone()));
        let outbox_repo: Arc<dyn OutboxRepository> = Arc::new(PgOutboxRepository::new(pool.clone()));

        // Create HTTP clients
        let click_router = Arc::new(ClickRouterClient::new(&settings.click_router)?);
        let click_aggregator = Arc::new(ClickAggregatorClient::new(&settings.click_aggregator)?);

        // Create infrastructure services
        let search_service = Arc::new(ElasticsearchService::new(&settings.elasticsearch)?);
        let storage_service = Arc::new(MinioService::new(&settings.minio).await?);

        // Create application services
        let route_service = Arc::new(RouteService::new(
            route_repo.clone(),
            domain_repo.clone(),
            outbox_repo.clone(),
            click_router.clone(),
        ));

        Ok(Self {
            route_repo,
            domain_repo,
            workspace_repo,
            certificate_repo,
            outbox_repo,
            route_service,
            click_router,
            click_aggregator,
            search_service,
            storage_service,
            settings,
        })
    }
}

/// Middleware to inject AppState into depot.
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
    async fn handle(
        &self,
        _req: &mut Request,
        depot: &mut Depot,
        _res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        depot.inject(self.app_state.clone());
        ctrl.call_next(_req, depot, _res).await;
    }
}

/// Extension trait for getting AppState from depot.
pub trait DepotExt {
    fn app_state(&self) -> anyhow::Result<&AppState>;
}

impl DepotExt for Depot {
    fn app_state(&self) -> anyhow::Result<&AppState> {
        self.obtain::<AppState>()
            .map_err(|_| anyhow::anyhow!("AppState not found in depot"))
    }
}
