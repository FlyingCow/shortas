//! Route repository implementation using SQLx.

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{
    ApiError, DestinationFormat, Result, Route, RouteProperties, RouteStatus, RoutingPolicy,
    RoutingTerminal,
};
use crate::domain::traits::{PaginatedResult, RouteFilters, RouteRepository};

/// PostgreSQL route repository.
pub struct PgRouteRepository {
    pool: PgPool,
}

impl PgRouteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Map database row to Route entity.
    fn map_row(row: &sqlx::postgres::PgRow) -> Result<Route> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let switch: String = row.try_get("switch").unwrap_or_default();
        let link: String = row.try_get("link").unwrap_or_default();
        let dest: Option<String> = row.try_get("dest").ok();
        let dest_format_str: String = row.try_get("dest_format").unwrap_or_else(|_| "Http".to_string());
        let code: Option<i32> = row.try_get("code").ok();
        let ttl: Option<i64> = row.try_get("ttl").ok();
        let status_str: String = row.try_get("status").unwrap_or_else(|_| "Active".to_string());
        let terminal_str: String = row.try_get("terminal").unwrap_or_else(|_| "External".to_string());
        let policy_json: String = row.try_get("policy_json").unwrap_or_else(|_| r#"{"type":"Basic"}"#.to_string());
        let _properties_id: Option<Uuid> = row.try_get("properties_id").ok();
        let domain_id: Option<Uuid> = row.try_get("domain_id").ok();

        // Parse properties from joined columns
        let properties = RouteProperties {
            route_id: row.try_get("prop_route_id").ok(),
            domain_id: row.try_get::<Option<Uuid>, _>("prop_domain_id").ok().flatten().map(|u| u.to_string()),
            owner_id: row.try_get("owner_id").ok(),
            creator_id: row.try_get("creator_id").ok(),
            workspace_id: row.try_get("workspace_id").ok(),
            scripts: row
                .try_get::<String, _>("scripts_json")
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok()),
            tags: row
                .try_get::<String, _>("tags_json")
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok()),
            custom: row
                .try_get::<String, _>("custom_json")
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok()),
            native: row
                .try_get::<String, _>("native_json")
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok()),
            bundling: row
                .try_get::<String, _>("bundling_json")
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok()),
            qr_settings: row
                .try_get::<String, _>("qr_settings_json")
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok()),
            opengraph: row.try_get("opengraph").unwrap_or(false),
            allow_debug: row.try_get("allow_debug").unwrap_or(false),
        };

        let dest_format = match dest_format_str.as_str() {
            "Native" => DestinationFormat::Native,
            _ => DestinationFormat::Http,
        };

        let terminal = match terminal_str.as_str() {
            "Internal" => RoutingTerminal::Internal,
            "Middleware" => RoutingTerminal::Middleware,
            _ => RoutingTerminal::External,
        };

        let status = match status_str.as_str() {
            "Blocked" => RouteStatus::Blocked(shortas_common::BlockedReason::Unknown),
            _ => RouteStatus::Active,
        };

        let policy: RoutingPolicy = serde_json::from_str(&policy_json).unwrap_or_default();

        Ok(Route {
            id,
            switch,
            link,
            dest,
            dest_format,
            code: code.map(|c| c as u16),
            ttl: ttl.map(|t| t as u64),
            status,
            terminal,
            policy,
            properties,
            domain_id,
        })
    }

    /// Generate a random alphanumeric link.
    fn generate_link() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::rng();
        (0..8)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[async_trait]
impl RouteRepository for PgRouteRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Route>> {
        let row = sqlx::query(
            r#"
            SELECT r.*,
                   p.id as properties_id, p.route_id as prop_route_id, p.domain_id as prop_domain_id,
                   p.owner_id, p.creator_id, p.workspace_id,
                   p.scripts_json, p.tags_json, p.custom_json, p.native_json,
                   p.bundling_json, p.qr_settings_json, p.opengraph, p.allow_debug
            FROM routes r
            LEFT JOIN route_properties p ON r.id = p.route_id
            WHERE r.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn get_by_domain_and_path(
        &self,
        domain: &str,
        path: &str,
        switch: Option<&str>,
    ) -> Result<Option<Route>> {
        let switch_val = switch.unwrap_or("main");

        let row = sqlx::query(
            r#"
            SELECT r.*,
                   p.id as properties_id, p.route_id as prop_route_id, p.domain_id as prop_domain_id,
                   p.owner_id, p.creator_id, p.workspace_id,
                   p.scripts_json, p.tags_json, p.custom_json, p.native_json,
                   p.bundling_json, p.qr_settings_json, p.opengraph, p.allow_debug
            FROM routes r
            LEFT JOIN route_properties p ON r.id = p.route_id
            JOIN route_domains d ON r.domain_id = d.id
            WHERE LOWER(d.name) = LOWER($1) AND r.link = $2 AND r.switch = $3
            "#,
        )
        .bind(domain)
        .bind(path)
        .bind(switch_val)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list(
        &self,
        page: i32,
        page_size: i32,
        filters: RouteFilters,
    ) -> Result<PaginatedResult<Route>> {
        let offset = (page - 1) * page_size;

        // Build dynamic WHERE clause
        let mut conditions = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(ref status) = filters.status {
            params.push(status.clone());
            conditions.push(format!("r.status = ${}", params.len()));
        }

        if let Some(ref owner_id) = filters.owner_id {
            params.push(owner_id.clone());
            conditions.push(format!("p.owner_id = ${}", params.len()));
        }

        if let Some(ref workspace_id) = filters.workspace_id {
            params.push(workspace_id.clone());
            conditions.push(format!("p.workspace_id = ${}", params.len()));
        }

        if let Some(domain_id) = filters.domain_id {
            params.push(domain_id.to_string());
            conditions.push(format!("r.domain_id = ${}::uuid", params.len()));
        }

        if let Some(ref search) = filters.search {
            params.push(format!("%{}%", search));
            conditions.push(format!("(r.link ILIKE ${} OR r.dest ILIKE ${})", params.len(), params.len()));
        }

        // Always filter for main switch (don't show conditional children)
        conditions.push("r.switch = 'main'".to_string());

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // Count query
        let count_sql = format!(
            r#"
            SELECT COUNT(*) as count
            FROM routes r
            LEFT JOIN route_properties p ON r.id = p.route_id
            {}
            "#,
            where_clause
        );

        let count_row = sqlx::query(&count_sql)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        let total_count: i64 = count_row.try_get("count").unwrap_or(0);

        // Data query
        let data_sql = format!(
            r#"
            SELECT r.*,
                   p.id as properties_id, p.route_id as prop_route_id, p.domain_id as prop_domain_id,
                   p.owner_id, p.creator_id, p.workspace_id,
                   p.scripts_json, p.tags_json, p.custom_json, p.native_json,
                   p.bundling_json, p.qr_settings_json, p.opengraph, p.allow_debug
            FROM routes r
            LEFT JOIN route_properties p ON r.id = p.route_id
            {}
            ORDER BY r.created_at DESC
            LIMIT {} OFFSET {}
            "#,
            where_clause, page_size, offset
        );

        let rows = sqlx::query(&data_sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        let routes: Vec<Route> = rows
            .iter()
            .filter_map(|r| Self::map_row(r).ok())
            .collect();

        Ok(PaginatedResult::new(routes, total_count, page, page_size))
    }

    async fn create(&self, route: &Route) -> Result<Route> {
        let mut tx = self.pool.begin().await.map_err(|e| ApiError::internal(e.to_string()))?;

        // Insert route
        let policy_json = serde_json::to_string(&route.policy).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO routes (id, switch, link, dest, dest_format, code, ttl, status, terminal, policy_json, domain_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(route.id)
        .bind(&route.switch)
        .bind(&route.link)
        .bind(&route.dest)
        .bind(format!("{:?}", route.dest_format))
        .bind(route.code.map(|c| c as i32))
        .bind(route.ttl.map(|t| t as i64))
        .bind(route.status.as_str())
        .bind(format!("{:?}", route.terminal))
        .bind(&policy_json)
        .bind(route.domain_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        // Insert properties
        let scripts_json = serde_json::to_string(&route.properties.scripts).unwrap_or_else(|_| "[]".to_string());
        let tags_json = serde_json::to_string(&route.properties.tags).unwrap_or_else(|_| "[]".to_string());
        let custom_json = serde_json::to_string(&route.properties.custom).unwrap_or_else(|_| "{}".to_string());
        let native_json = serde_json::to_string(&route.properties.native).unwrap_or_else(|_| "{}".to_string());
        let bundling_json = serde_json::to_string(&route.properties.bundling).unwrap_or_else(|_| "{}".to_string());
        let qr_settings_json = serde_json::to_string(&route.properties.qr_settings).unwrap_or_else(|_| "null".to_string());

        sqlx::query(
            r#"
            INSERT INTO route_properties (id, route_id, domain_id, owner_id, creator_id, workspace_id,
                scripts_json, tags_json, custom_json, native_json, bundling_json, qr_settings_json,
                opengraph, allow_debug)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(route.id)
        .bind(route.domain_id)
        .bind(&route.properties.owner_id)
        .bind(&route.properties.creator_id)
        .bind(&route.properties.workspace_id)
        .bind(&scripts_json)
        .bind(&tags_json)
        .bind(&custom_json)
        .bind(&native_json)
        .bind(&bundling_json)
        .bind(&qr_settings_json)
        .bind(route.properties.opengraph)
        .bind(route.properties.allow_debug)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| ApiError::internal(e.to_string()))?;

        // Return the created route
        self.get_by_id(route.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created route"))
    }

    async fn update(&self, route: &Route) -> Result<Route> {
        let policy_json = serde_json::to_string(&route.policy).unwrap_or_default();

        sqlx::query(
            r#"
            UPDATE routes
            SET dest = $2, dest_format = $3, code = $4, ttl = $5, status = $6,
                terminal = $7, policy_json = $8, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(route.id)
        .bind(&route.dest)
        .bind(format!("{:?}", route.dest_format))
        .bind(route.code.map(|c| c as i32))
        .bind(route.ttl.map(|t| t as i64))
        .bind(route.status.as_str())
        .bind(format!("{:?}", route.terminal))
        .bind(&policy_json)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        // Update properties
        let scripts_json = serde_json::to_string(&route.properties.scripts).unwrap_or_else(|_| "[]".to_string());
        let tags_json = serde_json::to_string(&route.properties.tags).unwrap_or_else(|_| "[]".to_string());
        let custom_json = serde_json::to_string(&route.properties.custom).unwrap_or_else(|_| "{}".to_string());
        let native_json = serde_json::to_string(&route.properties.native).unwrap_or_else(|_| "{}".to_string());
        let bundling_json = serde_json::to_string(&route.properties.bundling).unwrap_or_else(|_| "{}".to_string());
        let qr_settings_json = serde_json::to_string(&route.properties.qr_settings).unwrap_or_else(|_| "null".to_string());

        sqlx::query(
            r#"
            UPDATE route_properties
            SET workspace_id = $2, scripts_json = $3, tags_json = $4, custom_json = $5,
                native_json = $6, bundling_json = $7, qr_settings_json = $8,
                opengraph = $9, allow_debug = $10
            WHERE route_id = $1
            "#,
        )
        .bind(route.id)
        .bind(&route.properties.workspace_id)
        .bind(&scripts_json)
        .bind(&tags_json)
        .bind(&custom_json)
        .bind(&native_json)
        .bind(&bundling_json)
        .bind(&qr_settings_json)
        .bind(route.properties.opengraph)
        .bind(route.properties.allow_debug)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(route.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve updated route"))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        // Delete properties first (foreign key)
        sqlx::query("DELETE FROM route_properties WHERE route_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        sqlx::query("DELETE FROM routes WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn bulk_create(&self, routes: &[Route]) -> Result<Vec<Route>> {
        let mut created = Vec::with_capacity(routes.len());
        for route in routes {
            created.push(self.create(route).await?);
        }
        Ok(created)
    }

    async fn bulk_update(&self, routes: &[Route]) -> Result<Vec<Route>> {
        let mut updated = Vec::with_capacity(routes.len());
        for route in routes {
            updated.push(self.update(route).await?);
        }
        Ok(updated)
    }

    async fn bulk_delete(&self, ids: &[Uuid]) -> Result<()> {
        for id in ids {
            self.delete(*id).await?;
        }
        Ok(())
    }

    async fn link_exists(&self, domain_id: Uuid, link: &str) -> Result<bool> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM routes WHERE domain_id = $1 AND link = $2 AND switch = 'main') as exists",
        )
        .bind(domain_id)
        .bind(link)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(row.try_get::<bool, _>("exists").unwrap_or(false))
    }

    async fn suggest_link(&self, domain_id: Uuid) -> Result<String> {
        // Try up to 10 times to generate a unique link
        for _ in 0..10 {
            let link = Self::generate_link();
            if !self.link_exists(domain_id, &link).await? {
                return Ok(link);
            }
        }
        Err(ApiError::internal("Failed to generate unique link"))
    }

    async fn get_by_owner(&self, owner_id: &str, limit: i32) -> Result<Vec<Route>> {
        let rows = sqlx::query(
            r#"
            SELECT r.*,
                   p.id as properties_id, p.route_id as prop_route_id, p.domain_id as prop_domain_id,
                   p.owner_id, p.creator_id, p.workspace_id,
                   p.scripts_json, p.tags_json, p.custom_json, p.native_json,
                   p.bundling_json, p.qr_settings_json, p.opengraph, p.allow_debug
            FROM routes r
            LEFT JOIN route_properties p ON r.id = p.route_id
            WHERE p.owner_id = $1 AND r.switch = 'main'
            ORDER BY r.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(owner_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn count_by_workspace(&self, workspace_id: Uuid) -> Result<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as count FROM routes r JOIN route_properties p ON r.id = p.route_id WHERE p.workspace_id = $1::text AND r.switch = 'main'",
        )
        .bind(workspace_id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(row.try_get::<i64, _>("count").unwrap_or(0))
    }
}
