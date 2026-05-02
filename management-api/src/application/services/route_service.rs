//! Route service for orchestrating route operations.

use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::{ApiError, OutboxMessage, Result, Route};
use crate::domain::traits::{
    DomainRepository, OutboxRepository, PaginatedResult, RouteFilters, RouteRepository,
};
use crate::infrastructure::http_clients::ClickRouterClient;

/// Route service for managing routes.
pub struct RouteService {
    route_repo: Arc<dyn RouteRepository>,
    domain_repo: Arc<dyn DomainRepository>,
    outbox_repo: Arc<dyn OutboxRepository>,
    click_router: Arc<ClickRouterClient>,
}

impl RouteService {
    pub fn new(
        route_repo: Arc<dyn RouteRepository>,
        domain_repo: Arc<dyn DomainRepository>,
        outbox_repo: Arc<dyn OutboxRepository>,
        click_router: Arc<ClickRouterClient>,
    ) -> Self {
        Self {
            route_repo,
            domain_repo,
            outbox_repo,
            click_router,
        }
    }

    /// Get route by ID with ownership validation.
    pub async fn get_by_id(&self, id: Uuid, user_id: &str) -> Result<Route> {
        let route = self
            .route_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

        self.validate_ownership(&route, user_id)?;
        Ok(route)
    }

    /// Get route by domain and path.
    pub async fn get_by_domain_and_path(
        &self,
        domain: &str,
        path: &str,
        user_id: &str,
        switch: Option<&str>,
    ) -> Result<Route> {
        let route = self
            .route_repo
            .get_by_domain_and_path(domain, path, switch)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &format!("{}/{}", domain, path)))?;

        self.validate_ownership(&route, user_id)?;
        Ok(route)
    }

    /// List routes with pagination and filters.
    pub async fn list(
        &self,
        user_id: &str,
        page: i32,
        page_size: i32,
        filters: RouteFilters,
    ) -> Result<PaginatedResult<Route>> {
        // Ensure user can only see their own routes (unless admin)
        let mut filters = filters;
        if filters.owner_id.is_none() {
            filters.owner_id = Some(user_id.to_string());
        }

        self.route_repo.list(page, page_size, filters).await
    }

    /// Create a new route.
    pub async fn create(&self, mut route: Route, user_id: &str) -> Result<Route> {
        // Validate domain access
        let domain_id = route
            .domain_id
            .ok_or_else(|| ApiError::required("domain_id"))?;

        let domain = self
            .domain_repo
            .get_by_id(domain_id)
            .await?
            .ok_or_else(|| ApiError::not_found("Domain", &domain_id.to_string()))?;

        if !domain.can_use(user_id) {
            return Err(ApiError::forbidden());
        }

        // Generate link if not provided
        if route.link.is_empty() {
            route.link = self.route_repo.suggest_link(domain_id).await?;
        }

        // Check link uniqueness
        if self.route_repo.link_exists(domain_id, &route.link).await? {
            return Err(ApiError::conflict(format!(
                "Link '{}' already exists in this domain",
                route.link
            )));
        }

        // Set route_id in properties for click-router lookup
        route.properties.route_id = Some(route.id.to_string());

        // Create in database
        let created = self.route_repo.create(&route).await?;

        // Propagate to click-router
        self.propagate_route(&created, &domain.name).await?;

        // Queue for search indexing
        let outbox_msg = OutboxMessage::index_route(created.id);
        let _ = self.outbox_repo.create(&outbox_msg).await;

        // Queue for safety check if has destination
        if let Some(ref dest) = created.dest {
            let safety_msg = OutboxMessage::check_route_safety(created.id, dest.clone());
            let _ = self.outbox_repo.create(&safety_msg).await;
        }

        Ok(created)
    }

    /// Update an existing route.
    pub async fn update(&self, id: Uuid, updated: Route, user_id: &str) -> Result<Route> {
        let existing = self
            .route_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

        self.validate_ownership(&existing, user_id)?;

        // Enforce immutable fields
        let mut route = updated;
        route.link = existing.link.clone(); // Link is immutable
        route.domain_id = existing.domain_id; // Domain is immutable
        route.properties.owner_id = existing.properties.owner_id.clone();
        route.properties.creator_id = existing.properties.creator_id.clone();
        route.properties.route_id = existing.properties.route_id.clone();

        // Update in database
        let saved = self.route_repo.update(&route).await?;

        // Get domain for propagation
        if let Some(domain_id) = saved.domain_id {
            if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                self.propagate_route(&saved, &domain.name).await?;
            }
        }

        // Queue for re-indexing
        let outbox_msg = OutboxMessage::index_route(saved.id);
        let _ = self.outbox_repo.create(&outbox_msg).await;

        Ok(saved)
    }

    /// Delete a route.
    pub async fn delete(&self, id: Uuid, user_id: &str) -> Result<()> {
        let route = self
            .route_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

        self.validate_ownership(&route, user_id)?;

        // Delete from database
        self.route_repo.delete(id).await?;

        // Delete from click-router
        if let Some(domain_id) = route.domain_id {
            if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                self.delete_from_click_router(&route, &domain.name).await?;
            }
        }

        // Queue for index deletion
        let outbox_msg = OutboxMessage::delete_route_index(id);
        let _ = self.outbox_repo.create(&outbox_msg).await;

        Ok(())
    }

    /// Bulk create routes.
    pub async fn bulk_create(&self, routes: Vec<Route>, user_id: &str) -> Result<Vec<Route>> {
        // Validate all routes
        for route in &routes {
            if let Some(domain_id) = route.domain_id {
                let domain = self
                    .domain_repo
                    .get_by_id(domain_id)
                    .await?
                    .ok_or_else(|| ApiError::not_found("Domain", &domain_id.to_string()))?;

                if !domain.can_use(user_id) {
                    return Err(ApiError::forbidden());
                }
            } else {
                return Err(ApiError::required("domain_id"));
            }
        }

        // Create all routes
        let created = self.route_repo.bulk_create(&routes).await?;

        // Propagate each to click-router (TODO: batch this)
        for route in &created {
            if let Some(domain_id) = route.domain_id {
                if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                    let _ = self.propagate_route(route, &domain.name).await;
                }
            }

            // Queue for indexing
            let outbox_msg = OutboxMessage::index_route(route.id);
            let _ = self.outbox_repo.create(&outbox_msg).await;
        }

        Ok(created)
    }

    /// Bulk update routes.
    pub async fn bulk_update(&self, routes: Vec<Route>, user_id: &str) -> Result<Vec<Route>> {
        // Validate ownership of all routes
        for route in &routes {
            self.validate_ownership(route, user_id)?;
        }

        let updated = self.route_repo.bulk_update(&routes).await?;

        // Propagate and queue for re-indexing
        for route in &updated {
            if let Some(domain_id) = route.domain_id {
                if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                    let _ = self.propagate_route(route, &domain.name).await;
                }
            }

            let outbox_msg = OutboxMessage::index_route(route.id);
            let _ = self.outbox_repo.create(&outbox_msg).await;
        }

        Ok(updated)
    }

    /// Bulk delete routes.
    pub async fn bulk_delete(&self, ids: Vec<Uuid>, user_id: &str) -> Result<()> {
        // Validate ownership of all routes
        for id in &ids {
            let route = self
                .route_repo
                .get_by_id(*id)
                .await?
                .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

            self.validate_ownership(&route, user_id)?;
        }

        // Get routes for click-router deletion
        let mut routes_to_delete = Vec::new();
        for id in &ids {
            if let Ok(Some(route)) = self.route_repo.get_by_id(*id).await {
                routes_to_delete.push(route);
            }
        }

        // Delete from database
        self.route_repo.bulk_delete(&ids).await?;

        // Delete from click-router
        for route in routes_to_delete {
            if let Some(domain_id) = route.domain_id {
                if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                    let _ = self.delete_from_click_router(&route, &domain.name).await;
                }
            }

            // Queue for index deletion
            let outbox_msg = OutboxMessage::delete_route_index(route.id);
            let _ = self.outbox_repo.create(&outbox_msg).await;
        }

        Ok(())
    }

    /// Unblock a route (reset status to Active).
    pub async fn unblock(&self, id: Uuid, user_id: &str) -> Result<Route> {
        let mut route = self
            .route_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

        self.validate_ownership(&route, user_id)?;

        route.status = shortas_common::RouteStatus::Active;

        let saved = self.route_repo.update(&route).await?;

        // Propagate to click-router
        if let Some(domain_id) = saved.domain_id {
            if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                self.propagate_route(&saved, &domain.name).await?;
            }
        }

        Ok(saved)
    }

    /// Suggest a unique link for a domain.
    pub async fn suggest_link(&self, domain_id: Uuid) -> Result<String> {
        self.route_repo.suggest_link(domain_id).await
    }

    /// Validate user ownership of a route.
    fn validate_ownership(&self, route: &Route, user_id: &str) -> Result<()> {
        if route.properties.owner_id.as_deref() != Some(user_id) {
            return Err(ApiError::forbidden());
        }
        Ok(())
    }

    /// Propagate route to click-router.
    async fn propagate_route(&self, route: &Route, domain_name: &str) -> Result<()> {
        // Build route family for conditional routes
        let family = route.clone().build_family();

        for r in family {
            self.click_router
                .upsert_route(domain_name, &r)
                .await
                .map_err(|e| ApiError::external_service(e.to_string()))?;
        }

        Ok(())
    }

    /// Delete route from click-router.
    async fn delete_from_click_router(&self, route: &Route, domain_name: &str) -> Result<()> {
        // Delete route family for conditional routes
        let family = route.clone().build_family();

        for r in family {
            self.click_router
                .delete_route(domain_name, &r.link, &r.switch)
                .await
                .map_err(|e| ApiError::external_service(e.to_string()))?;
        }

        Ok(())
    }
}
