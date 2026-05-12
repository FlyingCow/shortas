//! Route service for orchestrating route operations.

use rand::Rng;
use std::collections::HashSet;
use std::sync::Arc;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::domain::entities::{ApiError, OutboxMessage, Result, Route, RouteStatus};
use crate::domain::traits::{
    DomainRepository, OutboxRepository, PaginatedResult, RouteFilters, RouteRepository,
};
use crate::infrastructure::http_clients::ClickRouterClient;

/// Constants for link generation algorithm
const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 10;
const BATCH_SIZE: usize = 10;
const FILL_THRESHOLD: f64 = 0.3; // grow length when >30% of space is used
const MAX_RETRIES: usize = 3;

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
            route.link = self.suggest_link(domain_id).await?;
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

        // Get domain for propagation (non-blocking - log errors but don't fail the request)
        if let Some(domain_id) = saved.domain_id {
            if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                if let Err(e) = self.propagate_route(&saved, &domain.name).await {
                    warn!(route_id = %saved.id, error = %e, "Failed to propagate route to click-router");
                }
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

        route.status = RouteStatus::Active;

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
    ///
    /// Uses a probabilistic algorithm:
    /// 1. Get the route count for the domain (cheap query).
    /// 2. Determine optimal tag length based on fill ratio.
    /// 3. Generate a batch of random candidates.
    /// 4. Check which candidates already exist (single batch query).
    /// 5. Return the first available candidate.
    /// 6. If all collide, retry with increased length.
    pub async fn suggest_link(&self, domain_id: Uuid) -> Result<String> {
        // Validate domain exists
        let domain = self
            .domain_repo
            .get_by_id(domain_id)
            .await?
            .ok_or_else(|| ApiError::not_found("Domain", &domain_id.to_string()))?;

        // Step 1: Get route count for this domain
        let existing_count = self.route_repo.count_by_domain(domain_id).await?;

        // Step 2: Determine optimal length
        let mut length = Self::calculate_optimal_length(existing_count as usize);

        // Steps 3-6: Generate and verify with retries
        for retry in 0..MAX_RETRIES {
            let candidates = Self::generate_candidates(length, BATCH_SIZE);

            // Step 4: Batch-check which candidates already exist
            let existing_links = self
                .route_repo
                .find_existing_links(domain_id, &candidates)
                .await?;
            let existing_set: HashSet<_> = existing_links.into_iter().collect();

            // Step 5: Return first available candidate
            if let Some(available) = candidates.into_iter().find(|c| !existing_set.contains(c)) {
                debug!(
                    tag = %available,
                    domain_id = %domain_id,
                    domain_name = %domain.name,
                    length = length,
                    retry = retry,
                    "Generated slash tag"
                );
                return Ok(available);
            }

            // All candidates collided - increase length and retry
            warn!(
                batch_size = BATCH_SIZE,
                domain_id = %domain_id,
                length = length,
                next_length = length + 1,
                "All candidates collided, retrying with increased length"
            );
            length = (length + 1).min(MAX_LENGTH);
        }

        Err(ApiError::internal(
            "Failed to generate a unique slash tag after maximum retries",
        ))
    }

    /// Calculate optimal tag length based on existing route count.
    /// Find the smallest length L (>= MIN_LENGTH) where fill ratio < threshold.
    fn calculate_optimal_length(existing_count: usize) -> usize {
        let alphabet_size = ALPHABET.len() as f64; // 36

        for length in MIN_LENGTH..=MAX_LENGTH {
            let total_space = alphabet_size.powi(length as i32);
            let fill_ratio = existing_count as f64 / total_space;

            if fill_ratio < FILL_THRESHOLD {
                return length;
            }
        }

        MAX_LENGTH
    }

    /// Generate a batch of random strings from the alphabet.
    fn generate_candidates(length: usize, count: usize) -> Vec<String> {
        let mut rng = rand::rng();
        let alphabet_len = ALPHABET.len();

        (0..count)
            .map(|_| {
                (0..length)
                    .map(|_| {
                        let idx = rng.random_range(0..alphabet_len);
                        ALPHABET[idx] as char
                    })
                    .collect()
            })
            .collect()
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
