use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::DynClone;

use crate::model::RouteToVerify;

/// Trait for accessing and updating routes for safety verification.
#[async_trait::async_trait]
pub trait RouteStore: DynClone + Send + Sync {
    /// Store a new route for verification (upsert - updates if exists).
    async fn store_route(&self, route: &RouteToVerify) -> Result<()>;

    /// Update an existing route's destinations.
    async fn update_route(&self, route: &RouteToVerify) -> Result<()>;

    /// Delete a route by ID.
    async fn delete_route(&self, id: &str) -> Result<()>;

    /// Get a route by ID.
    async fn get_route(&self, id: &str) -> Result<Option<RouteToVerify>>;

    /// List routes with optional filtering and pagination.
    async fn list_routes(
        &self,
        owner_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<RouteToVerify>, u64)>;

    /// Get routes that need safety verification.
    /// Returns routes where next_safety_check <= now OR next_safety_check is missing.
    async fn get_routes_for_verification(
        &self,
        before: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<RouteToVerify>>;

    /// Update the safety check timestamps for a route.
    async fn update_safety_check_timestamps(
        &self,
        route_id: &str,
        last_check: DateTime<Utc>,
        next_check: DateTime<Utc>,
    ) -> Result<()>;

    /// Update route status (Active/Blocked) with optional reason.
    async fn update_route_status(
        &self,
        route_id: &str,
        status: &str,
        blocked_reason: Option<&str>,
    ) -> Result<()>;
}

dyn_clone::clone_trait_object!(RouteStore);
