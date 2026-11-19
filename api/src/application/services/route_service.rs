//! Route service for orchestrating route operations.

use rand::Rng;
use std::collections::HashSet;
use std::sync::Arc;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::domain::entities::{ApiError, OutboxMessage, Result, Route, RouteStatus};
use crate::domain::traits::{
    DomainRepository, OutboxRepository, PaginatedResult, RouteFilters, RouteRepository,
    WorkspaceRepository,
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
    workspace_repo: Arc<dyn WorkspaceRepository>,
}

impl RouteService {
    pub fn new(
        route_repo: Arc<dyn RouteRepository>,
        domain_repo: Arc<dyn DomainRepository>,
        outbox_repo: Arc<dyn OutboxRepository>,
        click_router: Arc<ClickRouterClient>,
        workspace_repo: Arc<dyn WorkspaceRepository>,
    ) -> Self {
        Self {
            route_repo,
            domain_repo,
            outbox_repo,
            click_router,
            workspace_repo,
        }
    }

    /// Get route by ID with ownership validation.
    pub async fn get_by_id(&self, id: Uuid, user_id: &str) -> Result<Route> {
        let route = self
            .route_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

        self.validate_read_access(&route, user_id).await?;
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

        self.validate_read_access(&route, user_id).await?;
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
        let mut filters = filters;

        // Don't trust a client-supplied workspace_id filter without checking
        // the caller actually belongs to that workspace.
        if let Some(workspace_id) = &filters.workspace_id {
            let workspace_uuid = Uuid::parse_str(workspace_id)
                .map_err(|_| ApiError::validation("Invalid workspace ID"))?;
            let has_access = self
                .workspace_repo
                .user_has_access(workspace_uuid, user_id)
                .await
                .unwrap_or(false);
            if !has_access {
                return Err(ApiError::forbidden());
            }
            // Inside a shared workspace, routes can belong to any member
            // (e.g. the owner, when an admin created them) - membership
            // above is the access check, not per-route ownership.
        } else if filters.owner_id.is_none() {
            // No workspace context: default to the caller's own routes.
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

        // If this route is being created inside a shared workspace, it belongs
        // to the workspace owner's account (so it uses the owner's domains and
        // shows up under their quota/billing) even when an admin member — not
        // the owner — is the one creating it. The admin is still recorded as
        // the creator.
        let effective_owner_id = self.resolve_route_owner(&route, user_id).await?;

        if !domain.can_use(&effective_owner_id) {
            return Err(ApiError::forbidden());
        }

        route.properties.owner_id = Some(effective_owner_id);
        route.properties.creator_id = Some(user_id.to_string());

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

        self.validate_write_access(&existing, user_id).await?;

        // Enforce immutable fields
        let mut route = updated;
        route.link = existing.link.clone(); // Link is immutable
        route.domain_id = existing.domain_id; // Domain is immutable
        route.properties.owner_id = existing.properties.owner_id.clone();
        route.properties.creator_id = existing.properties.creator_id.clone();
        route.properties.route_id = existing.properties.route_id.clone();

        let dest_changed = existing.dest != route.dest;

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

        // Re-queue for safety check if the destination changed
        if dest_changed {
            if let Some(ref dest) = saved.dest {
                let safety_msg = OutboxMessage::check_route_safety(saved.id, dest.clone());
                let _ = self.outbox_repo.create(&safety_msg).await;
            }
        }

        Ok(saved)
    }

    /// Apply a status change coming from an external safety check (route-verifier
    /// blocking a route for Safe Browsing, for example). There's no end-user
    /// request behind this, so unlike `update()` it skips ownership validation
    /// and the immutable-field reset, and - crucially - still propagates the
    /// new status to click-router so the redirect-serving store actually
    /// reflects the block instead of only Postgres/the dashboard.
    pub async fn apply_safety_status(&self, route_id: Uuid, status: RouteStatus) -> Result<()> {
        let mut route = self
            .route_repo
            .get_by_id(route_id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &route_id.to_string()))?;

        route.status = status;
        let saved = self.route_repo.update(&route).await?;

        if let Some(domain_id) = saved.domain_id {
            if let Ok(Some(domain)) = self.domain_repo.get_by_id(domain_id).await {
                if let Err(e) = self.propagate_route(&saved, &domain.name).await {
                    warn!(route_id = %saved.id, error = %e, "Failed to propagate safety status to click-router");
                }
            }
        }

        Ok(())
    }

    /// Delete a route.
    pub async fn delete(&self, id: Uuid, user_id: &str) -> Result<()> {
        let route = self
            .route_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("Route", &id.to_string()))?;

        self.validate_write_access(&route, user_id).await?;

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

        // Queue removal from route-verifier
        let safety_msg = OutboxMessage::delete_route_safety_check(id);
        let _ = self.outbox_repo.create(&safety_msg).await;

        Ok(())
    }

    /// Bulk create routes.
    pub async fn bulk_create(&self, mut routes: Vec<Route>, user_id: &str) -> Result<Vec<Route>> {
        // Validate all routes and resolve each one's effective owner (see
        // `resolve_route_owner` - same shared-workspace rules as `create`).
        for route in &mut routes {
            let Some(domain_id) = route.domain_id else {
                return Err(ApiError::required("domain_id"));
            };

            let domain = self
                .domain_repo
                .get_by_id(domain_id)
                .await?
                .ok_or_else(|| ApiError::not_found("Domain", &domain_id.to_string()))?;

            let effective_owner_id = self.resolve_route_owner(route, user_id).await?;

            if !domain.can_use(&effective_owner_id) {
                return Err(ApiError::forbidden());
            }

            route.properties.owner_id = Some(effective_owner_id);
            route.properties.creator_id = Some(user_id.to_string());
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

            // Queue for safety check if has destination
            if let Some(ref dest) = route.dest {
                let safety_msg = OutboxMessage::check_route_safety(route.id, dest.clone());
                let _ = self.outbox_repo.create(&safety_msg).await;
            }
        }

        Ok(created)
    }

    /// Bulk update routes.
    pub async fn bulk_update(&self, routes: Vec<Route>, user_id: &str) -> Result<Vec<Route>> {
        // Validate ownership of all routes
        for route in &routes {
            self.validate_write_access(route, user_id).await?;
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

            if let Some(ref dest) = route.dest {
                let safety_msg = OutboxMessage::check_route_safety(route.id, dest.clone());
                let _ = self.outbox_repo.create(&safety_msg).await;
            }
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

            self.validate_write_access(&route, user_id).await?;
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

            // Queue removal from route-verifier
            let safety_msg = OutboxMessage::delete_route_safety_check(route.id);
            let _ = self.outbox_repo.create(&safety_msg).await;
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

        self.validate_write_access(&route, user_id).await?;

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
    /// Any workspace membership (Owner/Admin/Member) grants read access, in
    /// addition to the literal creator.
    async fn validate_read_access(&self, route: &Route, user_id: &str) -> Result<()> {
        if route.properties.owner_id.as_deref() == Some(user_id) {
            return Ok(());
        }
        if self.has_workspace_membership(route, user_id).await {
            return Ok(());
        }
        Err(ApiError::forbidden())
    }

    /// Only the literal creator or a workspace Owner/Admin may mutate a route.
    /// A plain workspace Member ("User" in the UI) has view-only/stats access.
    async fn validate_write_access(&self, route: &Route, user_id: &str) -> Result<()> {
        if route.properties.owner_id.as_deref() == Some(user_id) {
            return Ok(());
        }
        if self.has_workspace_admin_access(route, user_id).await {
            return Ok(());
        }
        Err(ApiError::forbidden())
    }

    async fn has_workspace_membership(&self, route: &Route, user_id: &str) -> bool {
        let Some(workspace_id) = route.properties.workspace_id.as_deref() else {
            return false;
        };
        let Ok(workspace_uuid) = Uuid::parse_str(workspace_id) else {
            return false;
        };
        self.workspace_repo
            .user_has_access(workspace_uuid, user_id)
            .await
            .unwrap_or(false)
    }

    async fn has_workspace_admin_access(&self, route: &Route, user_id: &str) -> bool {
        let Some(workspace_id) = route.properties.workspace_id.as_deref() else {
            return false;
        };
        let Ok(workspace_uuid) = Uuid::parse_str(workspace_id) else {
            return false;
        };
        match self.workspace_repo.get_membership(workspace_uuid, user_id).await {
            Ok(Some(membership)) => membership.is_admin(),
            _ => false,
        }
    }

    /// Determine who a new route should belong to, and that the caller is
    /// actually allowed to create one in the given context.
    ///
    /// - No workspace on the route: the caller owns it, as before.
    /// - Workspace given, caller is its owner: unchanged, caller owns it.
    /// - Workspace given, caller is an admin member (not owner): the route
    ///   belongs to the workspace owner's account instead — `create()` still
    ///   records the caller as `creator_id` separately.
    /// - Workspace given, caller has no admin membership: forbidden. This
    ///   also closes a gap where any caller could previously tag a new route
    ///   with an arbitrary `workspace_id` with no membership check at all.
    async fn resolve_route_owner(&self, route: &Route, user_id: &str) -> Result<String> {
        let Some(workspace_id) = route.properties.workspace_id.as_deref() else {
            return Ok(user_id.to_string());
        };
        let workspace_uuid = Uuid::parse_str(workspace_id)
            .map_err(|_| ApiError::validation("Invalid workspace ID"))?;

        let membership = self
            .workspace_repo
            .get_membership(workspace_uuid, user_id)
            .await?
            .ok_or_else(ApiError::forbidden)?;

        if !membership.is_admin() {
            return Err(ApiError::forbidden());
        }
        if membership.is_owner() {
            return Ok(user_id.to_string());
        }

        Ok(self
            .workspace_repo
            .get_owner_user_id(workspace_uuid)
            .await?
            .unwrap_or_else(|| user_id.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        DestinationFormat, RouteDomain, RouteProperties, RouteStatus, RoutingPolicy,
        RoutingTerminal, UserWorkspace,
    };
    use crate::domain::traits::{MockDomainRepository, MockOutboxRepository, MockRouteRepository, MockWorkspaceRepository};
    use crate::settings::ClickRouterSettings;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// A click-router double that accepts any PUT/DELETE, for tests that
    /// need propagation to succeed but aren't asserting on it directly.
    async fn ok_click_router() -> Arc<ClickRouterClient> {
        let server = MockServer::start().await;
        Mock::given(method("PUT")).respond_with(ResponseTemplate::new(200)).mount(&server).await;
        Mock::given(method("DELETE")).respond_with(ResponseTemplate::new(200)).mount(&server).await;
        // Leak the server so it outlives the client for the duration of the test process;
        // acceptable in a test-only helper used once per test.
        let uri = server.uri();
        std::mem::forget(server);
        Arc::new(ClickRouterClient::new(&ClickRouterSettings { base_url: uri, timeout_ms: 5000 }).unwrap())
    }

    fn sample_domain(owner_id: &str) -> RouteDomain {
        RouteDomain::new("example.com".to_string(), owner_id.to_string())
    }

    fn route_with_domain(domain_id: Uuid, link: &str) -> Route {
        Route {
            id: Uuid::new_v4(),
            switch: "main".to_string(),
            link: link.to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: None,
            ttl: None,
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties::default(),
            domain_id: Some(domain_id),
        }
    }

    async fn service(
        route_repo: MockRouteRepository,
        domain_repo: MockDomainRepository,
        outbox_repo: MockOutboxRepository,
        workspace_repo: MockWorkspaceRepository,
    ) -> RouteService {
        RouteService::new(
            Arc::new(route_repo),
            Arc::new(domain_repo),
            Arc::new(outbox_repo),
            ok_click_router().await,
            Arc::new(workspace_repo),
        )
    }

    // --- pure helpers ---

    #[test]
    fn test_calculate_optimal_length_grows_with_existing_count() {
        assert_eq!(RouteService::calculate_optimal_length(0), MIN_LENGTH);
        // 36^3 = 46656; 30% of that is ~13996, so above that it should grow to length 4.
        assert_eq!(RouteService::calculate_optimal_length(20_000), 4);
    }

    #[test]
    fn test_generate_candidates_respects_length_count_and_alphabet() {
        let candidates = RouteService::generate_candidates(5, 10);
        assert_eq!(candidates.len(), 10);
        for c in &candidates {
            assert_eq!(c.len(), 5);
            assert!(c.bytes().all(|b| ALPHABET.contains(&b)));
        }
    }

    // --- suggest_link ---

    #[tokio::test]
    async fn test_suggest_link_returns_available_candidate() {
        let domain_id = Uuid::new_v4();
        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_count_by_domain().returning(|_| Ok(0));
        route_repo.expect_find_existing_links().returning(|_, _| Ok(vec![]));

        let svc = service(
            route_repo,
            domain_repo,
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let link = svc.suggest_link(domain_id).await.unwrap();
        assert_eq!(link.len(), MIN_LENGTH);
        assert!(link.bytes().all(|b| ALPHABET.contains(&b)));
    }

    #[tokio::test]
    async fn test_suggest_link_domain_not_found() {
        let domain_id = Uuid::new_v4();
        let mut domain_repo = MockDomainRepository::new();
        domain_repo.expect_get_by_id().returning(|_| Ok(None));

        let svc = service(
            MockRouteRepository::new(),
            domain_repo,
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.suggest_link(domain_id).await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn test_suggest_link_exhausts_retries_when_all_candidates_collide() {
        let domain_id = Uuid::new_v4();
        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_count_by_domain().returning(|_| Ok(0));
        // Echo every candidate back as "already existing" to force exhaustion.
        route_repo
            .expect_find_existing_links()
            .returning(|_, links| Ok(links.to_vec()));

        let svc = service(
            route_repo,
            domain_repo,
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.suggest_link(domain_id).await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::InternalError);
    }

    // --- get_by_id access control ---

    #[tokio::test]
    async fn test_get_by_id_allows_owner() {
        let domain_id = Uuid::new_v4();
        let mut route = route_with_domain(domain_id, "abc");
        route.properties.owner_id = Some("owner-1".to_string());
        let route_id = route.id;

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(move |_| Ok(Some(route.clone())));

        let svc = service(
            route_repo,
            MockDomainRepository::new(),
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let result = svc.get_by_id(route_id, "owner-1").await.unwrap();
        assert_eq!(result.id, route_id);
    }

    #[tokio::test]
    async fn test_get_by_id_allows_workspace_member() {
        let domain_id = Uuid::new_v4();
        let workspace_id = Uuid::new_v4();
        let mut route = route_with_domain(domain_id, "abc");
        route.properties.owner_id = Some("owner-1".to_string());
        route.properties.workspace_id = Some(workspace_id.to_string());
        let route_id = route.id;

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(move |_| Ok(Some(route.clone())));

        let mut workspace_repo = MockWorkspaceRepository::new();
        workspace_repo.expect_user_has_access().returning(|_, _| Ok(true));

        let svc = service(route_repo, MockDomainRepository::new(), MockOutboxRepository::new(), workspace_repo).await;

        let result = svc.get_by_id(route_id, "member-1").await.unwrap();
        assert_eq!(result.id, route_id);
    }

    #[tokio::test]
    async fn test_get_by_id_denies_non_member() {
        let domain_id = Uuid::new_v4();
        let mut route = route_with_domain(domain_id, "abc");
        route.properties.owner_id = Some("owner-1".to_string());
        let route_id = route.id;

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(move |_| Ok(Some(route.clone())));

        let svc = service(
            route_repo,
            MockDomainRepository::new(),
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.get_by_id(route_id, "stranger").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::Forbidden);
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(|_| Ok(None));

        let svc = service(
            route_repo,
            MockDomainRepository::new(),
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.get_by_id(Uuid::new_v4(), "owner-1").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::NotFound);
    }

    // --- create ---

    #[tokio::test]
    async fn test_create_requires_domain_id() {
        let mut route = route_with_domain(Uuid::new_v4(), "abc");
        route.domain_id = None;

        let svc = service(
            MockRouteRepository::new(),
            MockDomainRepository::new(),
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.create(route, "owner-1").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::ValidationError);
    }

    #[tokio::test]
    async fn test_create_domain_not_found() {
        let domain_id = Uuid::new_v4();
        let route = route_with_domain(domain_id, "abc");

        let mut domain_repo = MockDomainRepository::new();
        domain_repo.expect_get_by_id().returning(|_| Ok(None));

        let svc = service(
            MockRouteRepository::new(),
            domain_repo,
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.create(route, "owner-1").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn test_create_forbidden_when_domain_not_usable_by_owner() {
        let domain_id = Uuid::new_v4();
        let route = route_with_domain(domain_id, "abc");

        let mut domain_repo = MockDomainRepository::new();
        // Domain owned by someone else and not shared.
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("someone-else"))));

        let svc = service(
            MockRouteRepository::new(),
            domain_repo,
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.create(route, "owner-1").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::Forbidden);
    }

    #[tokio::test]
    async fn test_create_conflict_when_link_exists() {
        let domain_id = Uuid::new_v4();
        let route = route_with_domain(domain_id, "abc");

        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_link_exists().returning(|_, _| Ok(true));

        let svc = service(
            route_repo,
            domain_repo,
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc.create(route, "owner-1").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::Conflict);
    }

    #[tokio::test]
    async fn test_create_happy_path_sets_owner_creator_and_queues_outbox() {
        let domain_id = Uuid::new_v4();
        let route = route_with_domain(domain_id, "abc");

        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_link_exists().returning(|_, _| Ok(false));
        route_repo.expect_create().returning(|r| Ok(r.clone()));

        let mut outbox_repo = MockOutboxRepository::new();
        // index_route + check_route_safety (dest is Some)
        outbox_repo.expect_create().times(2).returning(|m| Ok(m.clone()));

        let svc = service(route_repo, domain_repo, outbox_repo, MockWorkspaceRepository::new()).await;

        let created = svc.create(route, "owner-1").await.unwrap();
        assert_eq!(created.properties.owner_id, Some("owner-1".to_string()));
        assert_eq!(created.properties.creator_id, Some("owner-1".to_string()));
        assert_eq!(created.properties.route_id, Some(created.id.to_string()));
    }

    #[tokio::test]
    async fn test_create_in_shared_workspace_assigns_owner_to_workspace_owner() {
        let domain_id = Uuid::new_v4();
        let workspace_id = Uuid::new_v4();
        let mut route = route_with_domain(domain_id, "abc");
        route.properties.workspace_id = Some(workspace_id.to_string());

        let mut domain_repo = MockDomainRepository::new();
        // Domain belongs to the workspace owner, not the admin creating it.
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("workspace-owner"))));

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_link_exists().returning(|_, _| Ok(false));
        route_repo.expect_create().returning(|r| Ok(r.clone()));

        let mut outbox_repo = MockOutboxRepository::new();
        outbox_repo.expect_create().times(2).returning(|m| Ok(m.clone()));

        let mut workspace_repo = MockWorkspaceRepository::new();
        let admin_membership = UserWorkspace::new("admin-1".to_string(), workspace_id, "Admin".to_string());
        workspace_repo
            .expect_get_membership()
            .returning(move |_, _| Ok(Some(admin_membership.clone())));
        workspace_repo
            .expect_get_owner_user_id()
            .returning(|_| Ok(Some("workspace-owner".to_string())));

        let svc = service(route_repo, domain_repo, outbox_repo, workspace_repo).await;

        let created = svc.create(route, "admin-1").await.unwrap();
        assert_eq!(created.properties.owner_id, Some("workspace-owner".to_string()));
        assert_eq!(created.properties.creator_id, Some("admin-1".to_string()));
    }

    #[tokio::test]
    async fn test_create_forbidden_when_no_workspace_membership() {
        let domain_id = Uuid::new_v4();
        let workspace_id = Uuid::new_v4();
        let mut route = route_with_domain(domain_id, "abc");
        route.properties.workspace_id = Some(workspace_id.to_string());

        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut workspace_repo = MockWorkspaceRepository::new();
        workspace_repo.expect_get_membership().returning(|_, _| Ok(None));

        let svc = service(
            MockRouteRepository::new(),
            domain_repo,
            MockOutboxRepository::new(),
            workspace_repo,
        )
        .await;

        let err = svc.create(route, "stranger").await.unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::Forbidden);
    }

    // --- update ---

    #[tokio::test]
    async fn test_update_preserves_immutable_fields() {
        let domain_id = Uuid::new_v4();
        let other_domain_id = Uuid::new_v4();
        let mut existing = route_with_domain(domain_id, "original-link");
        existing.properties.owner_id = Some("owner-1".to_string());
        existing.properties.creator_id = Some("owner-1".to_string());
        existing.properties.route_id = Some(existing.id.to_string());
        let route_id = existing.id;

        let mut attempted_update = existing.clone();
        attempted_update.link = "hacked-link".to_string();
        attempted_update.domain_id = Some(other_domain_id);
        attempted_update.properties.owner_id = Some("attacker".to_string());
        attempted_update.dest = Some("https://still-example.com".to_string());

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(move |_| Ok(Some(existing.clone())));
        route_repo.expect_update().returning(|r| Ok(r.clone()));

        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut outbox_repo = MockOutboxRepository::new();
        outbox_repo.expect_create().returning(|m| Ok(m.clone()));

        let svc = service(route_repo, domain_repo, outbox_repo, MockWorkspaceRepository::new()).await;

        let saved = svc.update(route_id, attempted_update, "owner-1").await.unwrap();
        assert_eq!(saved.link, "original-link");
        assert_eq!(saved.domain_id, Some(domain_id));
        assert_eq!(saved.properties.owner_id, Some("owner-1".to_string()));
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(|_| Ok(None));

        let svc = service(
            route_repo,
            MockDomainRepository::new(),
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc
            .update(Uuid::new_v4(), route_with_domain(Uuid::new_v4(), "abc"), "owner-1")
            .await
            .unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn test_update_forbidden_for_non_owner_non_admin() {
        let domain_id = Uuid::new_v4();
        let mut existing = route_with_domain(domain_id, "abc");
        existing.properties.owner_id = Some("owner-1".to_string());
        let route_id = existing.id;

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(move |_| Ok(Some(existing.clone())));

        let svc = service(
            route_repo,
            MockDomainRepository::new(),
            MockOutboxRepository::new(),
            MockWorkspaceRepository::new(),
        )
        .await;

        let err = svc
            .update(route_id, route_with_domain(domain_id, "abc"), "stranger")
            .await
            .unwrap_err();
        assert_eq!(err.code, crate::domain::entities::ErrorCode::Forbidden);
    }

    // --- delete ---

    #[tokio::test]
    async fn test_delete_removes_route_and_queues_cleanup() {
        let domain_id = Uuid::new_v4();
        let mut route = route_with_domain(domain_id, "abc");
        route.properties.owner_id = Some("owner-1".to_string());
        let route_id = route.id;

        let mut route_repo = MockRouteRepository::new();
        route_repo.expect_get_by_id().returning(move |_| Ok(Some(route.clone())));
        route_repo.expect_delete().returning(|_| Ok(()));

        let mut domain_repo = MockDomainRepository::new();
        domain_repo
            .expect_get_by_id()
            .returning(move |_| Ok(Some(sample_domain("owner-1"))));

        let mut outbox_repo = MockOutboxRepository::new();
        // delete_route_index + delete_route_safety_check
        outbox_repo.expect_create().times(2).returning(|m| Ok(m.clone()));

        let svc = service(route_repo, domain_repo, outbox_repo, MockWorkspaceRepository::new()).await;

        svc.delete(route_id, "owner-1").await.unwrap();
    }
}
