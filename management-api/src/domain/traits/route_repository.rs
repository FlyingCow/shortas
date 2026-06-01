//! Route repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Result, Route};

/// Route query filters.
#[derive(Debug, Clone, Default)]
pub struct RouteFilters {
    pub status: Option<String>,
    pub owner_id: Option<String>,
    pub workspace_id: Option<String>,
    pub domain_id: Option<Uuid>,
    pub search: Option<String>,
}

/// Paginated result.
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i32,
    pub page_size: i32,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total_count: i64, page: i32, page_size: i32) -> Self {
        Self {
            items,
            total_count,
            page,
            page_size,
        }
    }

    pub fn total_pages(&self) -> i32 {
        ((self.total_count as f64) / (self.page_size as f64)).ceil() as i32
    }
}

/// Lightweight route info for enrichment.
#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub id: Uuid,
    pub link: String,
    pub domain_name: Option<String>,
}

/// Route repository trait for database operations.
#[async_trait]
pub trait RouteRepository: Send + Sync {
    /// Get route by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Route>>;

    /// Get route info (link + domain name) for multiple route IDs.
    async fn get_route_info_by_ids(&self, ids: &[Uuid]) -> Result<Vec<RouteInfo>>;

    /// Get route by domain and path.
    async fn get_by_domain_and_path(
        &self,
        domain: &str,
        path: &str,
        switch: Option<&str>,
    ) -> Result<Option<Route>>;

    /// List routes with pagination and filters.
    async fn list(
        &self,
        page: i32,
        page_size: i32,
        filters: RouteFilters,
    ) -> Result<PaginatedResult<Route>>;

    /// Create a new route.
    async fn create(&self, route: &Route) -> Result<Route>;

    /// Update an existing route.
    async fn update(&self, route: &Route) -> Result<Route>;

    /// Delete a route by ID.
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// Bulk create routes.
    async fn bulk_create(&self, routes: &[Route]) -> Result<Vec<Route>>;

    /// Bulk update routes.
    async fn bulk_update(&self, routes: &[Route]) -> Result<Vec<Route>>;

    /// Bulk delete routes by IDs.
    async fn bulk_delete(&self, ids: &[Uuid]) -> Result<()>;

    /// Check if link exists for domain.
    async fn link_exists(&self, domain_id: Uuid, link: &str) -> Result<bool>;

    /// Check which links from a batch already exist for a domain.
    async fn find_existing_links(&self, domain_id: Uuid, links: &[String]) -> Result<Vec<String>>;

    /// Count routes for a domain.
    async fn count_by_domain(&self, domain_id: Uuid) -> Result<i64>;

    /// Get routes by owner ID.
    async fn get_by_owner(&self, owner_id: &str, limit: i32) -> Result<Vec<Route>>;

    /// Count routes for workspace.
    async fn count_by_workspace(&self, workspace_id: Uuid) -> Result<i64>;
}
