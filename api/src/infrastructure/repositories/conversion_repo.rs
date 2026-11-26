//! Conversion repository implementation using SQLx.

use async_trait::async_trait;
use sqlx::{postgres::PgRow, PgPool, Postgres, Row, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

use super::route_coverage;
use crate::domain::entities::{
    ApiError, Conversion, ConversionDefinition, ConversionStatus, ConversionStep, CountingMode,
    OutboxMessage, Result,
};
use crate::domain::traits::ConversionRepository;

pub(super) fn db_err(e: sqlx::Error) -> ApiError {
    ApiError::internal(e.to_string())
}

/// Queued in the caller's transaction, so a saved change is always sent.
pub(super) async fn queue_propagation(
    tx: &mut Transaction<'_, Postgres>,
    workspace_id: Uuid,
    id: Uuid,
    public_key: &str,
) -> Result<()> {
    let message = OutboxMessage::propagate_conversion(workspace_id, id, public_key);
    sqlx::query(
        "INSERT INTO outbox_messages (id, message_type, payload, max_retries) VALUES ($1, $2, $3, $4)",
    )
    .bind(message.id)
    .bind(message.message_type.as_str())
    .bind(message.payload.to_string())
    .bind(message.max_retries)
    .execute(&mut **tx)
    .await
    .map_err(db_err)?;
    Ok(())
}

/// PostgreSQL conversion repository.
pub struct PgConversionRepository {
    pool: PgPool,
}

impl PgConversionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &PgRow) -> Result<Conversion> {
        let get_err = |what: &str| ApiError::internal(format!("Invalid conversion {what}"));
        let status: String = row.try_get("status").map_err(db_err)?;
        let counting: String = row.try_get("counting").map_err(db_err)?;
        let all_routes: bool = row.try_get("all_routes").map_err(db_err)?;

        Ok(Conversion {
            id: row.try_get("id").map_err(db_err)?,
            workspace_id: row.try_get("workspace_id").map_err(db_err)?,
            definition: ConversionDefinition {
                name: row.try_get("name").map_err(db_err)?,
                currency: row.try_get("currency").map_err(db_err)?,
                window_days: row.try_get("window_days").map_err(db_err)?,
                counting: CountingMode::parse(&counting).ok_or_else(|| get_err("counting"))?,
                domain_id: row.try_get("domain_id").map_err(db_err)?,
                allowed_origins: row.try_get("allowed_origins").map_err(db_err)?,
                server_values_only: row.try_get("server_values_only").map_err(db_err)?,
                steps: Vec::new(),
                route_filter: (!all_routes).then(Vec::new),
            },
            archived_steps: Vec::new(),
            status: ConversionStatus::parse(&status).ok_or_else(|| get_err("status"))?,
            public_key: row.try_get("public_key").map_err(db_err)?,
            secret_hash: row.try_get("secret_hash").map_err(db_err)?,
            version: row.try_get("version").map_err(db_err)?,
            created_by: row.try_get("created_by").map_err(db_err)?,
            activated_at: row.try_get("activated_at").map_err(db_err)?,
            created_at: row.try_get("created_at").map_err(db_err)?,
            updated_at: row.try_get("updated_at").map_err(db_err)?,
        })
    }

    /// Adds steps and routes to conversions read from their own table.
    async fn load(&self, rows: Vec<PgRow>) -> Result<Vec<Conversion>> {
        let mut conversions = rows.iter().map(Self::map_row).collect::<Result<Vec<_>>>()?;
        let ids: Vec<Uuid> = conversions.iter().map(|c| c.id).collect();
        let index: HashMap<Uuid, usize> = ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();

        let steps = sqlx::query(
            "SELECT * FROM conversion_steps WHERE conversion_id = ANY($1) ORDER BY position, key",
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        for row in steps {
            let conversion = &mut conversions[index[&row.try_get("conversion_id").map_err(db_err)?]];
            let step = ConversionStep {
                key: row.try_get("key").map_err(db_err)?,
                name: row.try_get("name").map_err(db_err)?,
                default_value: row.try_get("default_value").map_err(db_err)?,
                archived: row.try_get("archived").map_err(db_err)?,
            };
            if step.archived {
                conversion.archived_steps.push(step);
            } else {
                conversion.definition.steps.push(step);
            }
        }

        let routes = sqlx::query(
            "SELECT * FROM conversion_routes WHERE conversion_id = ANY($1) ORDER BY route_id",
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        for row in routes {
            let conversion = &mut conversions[index[&row.try_get("conversion_id").map_err(db_err)?]];
            if let Some(filter) = &mut conversion.definition.route_filter {
                filter.push(row.try_get("route_id").map_err(db_err)?);
            }
        }

        Ok(conversions)
    }

    async fn write_children(tx: &mut Transaction<'_, Postgres>, c: &Conversion) -> Result<()> {
        for table in ["conversion_steps", "conversion_routes"] {
            sqlx::query(&format!("DELETE FROM {table} WHERE conversion_id = $1"))
                .bind(c.id)
                .execute(&mut **tx)
                .await
                .map_err(db_err)?;
        }

        let steps = c.definition.steps.iter().chain(&c.archived_steps);
        for (position, step) in steps.enumerate() {
            sqlx::query(
                "INSERT INTO conversion_steps (conversion_id, key, name, position, default_value, archived) \
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(c.id)
            .bind(&step.key)
            .bind(&step.name)
            .bind(position as i32)
            .bind(step.default_value)
            .bind(step.archived)
            .execute(&mut **tx)
            .await
            .map_err(db_err)?;
        }

        if let Some(routes) = &c.definition.route_filter {
            sqlx::query(
                "INSERT INTO conversion_routes (conversion_id, route_id) \
                 SELECT $1, id FROM routes WHERE id = ANY($2)",
            )
            .bind(c.id)
            .bind(routes)
            .execute(&mut **tx)
            .await
            .map_err(db_err)?;
        }

        Ok(())
    }
}

#[async_trait]
impl ConversionRepository for PgConversionRepository {
    async fn create(&self, c: &Conversion) -> Result<()> {
        let d = &c.definition;
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        route_coverage::lock_for_conversions(&mut tx, c.workspace_id).await?;
        sqlx::query(
            r#"
            INSERT INTO conversions (
                id, workspace_id, name, status, public_key, secret_hash, currency,
                window_days, counting, domain_id, allowed_origins, server_values_only, all_routes,
                version, created_by, activated_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            "#,
        )
        .bind(c.id)
        .bind(c.workspace_id)
        .bind(&d.name)
        .bind(c.status.as_str())
        .bind(&c.public_key)
        .bind(&c.secret_hash)
        .bind(&d.currency)
        .bind(d.window_days)
        .bind(d.counting.as_str())
        .bind(d.domain_id)
        .bind(&d.allowed_origins)
        .bind(d.server_values_only)
        .bind(d.route_filter.is_none())
        .bind(c.version)
        .bind(&c.created_by)
        .bind(c.activated_at)
        .bind(c.created_at)
        .bind(c.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;
        Self::write_children(&mut tx, c).await?;
        route_coverage::refresh_workspace(&mut tx, c.workspace_id).await?;
        queue_propagation(&mut tx, c.workspace_id, c.id, &c.public_key).await?;
        tx.commit().await.map_err(db_err)
    }

    async fn get(&self, workspace_id: Uuid, id: Uuid) -> Result<Option<Conversion>> {
        let rows = sqlx::query("SELECT * FROM conversions WHERE workspace_id = $1 AND id = $2")
            .bind(workspace_id)
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(self.load(rows).await?.pop())
    }

    async fn list(&self, workspace_id: Uuid) -> Result<Vec<Conversion>> {
        let rows = sqlx::query(
            "SELECT * FROM conversions WHERE workspace_id = $1 ORDER BY created_at DESC, id",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        self.load(rows).await
    }

    async fn save(&self, c: &Conversion) -> Result<Conversion> {
        let d = &c.definition;
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        route_coverage::lock_for_conversions(&mut tx, c.workspace_id).await?;
        let row = sqlx::query(
            r#"
            UPDATE conversions SET
                name = $3, status = $4, secret_hash = $5, currency = $6, window_days = $7,
                counting = $8, domain_id = $9, allowed_origins = $10, server_values_only = $11,
                all_routes = $12, activated_at = $13, version = version + 1, updated_at = NOW()
            WHERE id = $1 AND version = $2
            RETURNING version, updated_at
            "#,
        )
        .bind(c.id)
        .bind(c.version)
        .bind(&d.name)
        .bind(c.status.as_str())
        .bind(&c.secret_hash)
        .bind(&d.currency)
        .bind(d.window_days)
        .bind(d.counting.as_str())
        .bind(d.domain_id)
        .bind(&d.allowed_origins)
        .bind(d.server_values_only)
        .bind(d.route_filter.is_none())
        .bind(c.activated_at)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_err)?
        .ok_or_else(|| ApiError::conflict("The conversion was changed by someone else"))?;
        Self::write_children(&mut tx, c).await?;
        route_coverage::refresh_workspace(&mut tx, c.workspace_id).await?;
        queue_propagation(&mut tx, c.workspace_id, c.id, &c.public_key).await?;
        tx.commit().await.map_err(db_err)?;

        Ok(Conversion {
            version: row.try_get("version").map_err(db_err)?,
            updated_at: row.try_get("updated_at").map_err(db_err)?,
            ..c.clone()
        })
    }

    async fn finish_activations(&self) -> Result<u64> {
        let waiting = sqlx::query(
            "SELECT id, workspace_id, public_key FROM conversions WHERE status = 'Activating'",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let mut finished = 0;
        for row in waiting {
            let id: Uuid = row.try_get("id").map_err(db_err)?;
            let workspace_id: Uuid = row.try_get("workspace_id").map_err(db_err)?;
            let public_key: String = row.try_get("public_key").map_err(db_err)?;

            let mut tx = self.pool.begin().await.map_err(db_err)?;
            route_coverage::lock_for_conversions(&mut tx, workspace_id).await?;
            let updated = sqlx::query(
                "UPDATE conversions SET status = 'Active', version = version + 1, updated_at = NOW(), \
                     activated_at = COALESCE(activated_at, NOW()) \
                 WHERE id = $1 AND status = 'Activating' AND NOT EXISTS ( \
                     SELECT 1 FROM route_properties p WHERE LOWER(p.workspace_id) = $2 \
                       AND p.track_conversions <> p.track_conversions_sent)",
            )
            .bind(id)
            .bind(workspace_id.to_string())
            .execute(&mut *tx)
            .await
            .map_err(db_err)?
            .rows_affected();
            if updated > 0 {
                queue_propagation(&mut tx, workspace_id, id, &public_key).await?;
            }
            tx.commit().await.map_err(db_err)?;
            finished += updated;
        }
        Ok(finished)
    }

    async fn list_active_by_domain(&self, domain_id: Uuid) -> Result<Vec<Conversion>> {
        let rows = sqlx::query(
            "SELECT * FROM conversions WHERE domain_id = $1 AND status IN ('Activating', 'Active') \
             ORDER BY name, id",
        )
        .bind(domain_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        self.load(rows).await
    }

    async fn count_routes(&self, workspace_id: Uuid, route_ids: &[Uuid]) -> Result<i64> {
        sqlx::query_scalar(
            "SELECT COUNT(DISTINCT r.id) FROM routes r JOIN route_properties p ON p.route_id = r.id \
             WHERE r.id = ANY($1) AND r.switch = 'main' AND LOWER(p.workspace_id) = $2",
        )
        .bind(route_ids)
        .bind(workspace_id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::domain::entities::conversion::tests::{definition, step};
    use crate::domain::entities::{ErrorCode, Route, RouteProperties};
    use crate::domain::traits::{RouteRepository, WorkspaceRepository};
    use crate::infrastructure::repositories::{PgRouteRepository, PgWorkspaceRepository};

    /// Run with `TEST_DATABASE_URL=postgres://... cargo test -- --ignored`.
    pub(crate) async fn test_pool() -> PgPool {
        let pool = PgPool::connect(&std::env::var("TEST_DATABASE_URL").unwrap()).await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    pub(crate) async fn insert_workspace(pool: &PgPool) -> Uuid {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO workspaces (id, name) VALUES ($1, 'w')").bind(id).execute(pool).await.unwrap();
        id
    }

    pub(crate) async fn insert_route(pool: &PgPool, workspace_id: Uuid) -> Uuid {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO routes (id, switch, link) VALUES ($1, 'main', $2)")
            .bind(id).bind(id.to_string()).execute(pool).await.unwrap();
        sqlx::query("INSERT INTO route_properties (id, route_id, workspace_id) VALUES ($1, $2, $3)")
            .bind(Uuid::new_v4()).bind(id).bind(workspace_id.to_string()).execute(pool).await.unwrap();
        id
    }

    #[tokio::test]
    #[ignore = "needs PostgreSQL"]
    async fn round_trips_and_detects_concurrent_changes() {
        let pool = test_pool().await;
        let repo = PgConversionRepository::new(pool.clone());
        let workspace = insert_workspace(&pool).await;
        let mut d = definition();
        d.steps[0].default_value = Some("5.2500".parse().unwrap());
        let (c, secret) = Conversion::new(workspace, d, "u".into());
        repo.create(&c).await.unwrap();

        let read = repo.get(workspace, c.id).await.unwrap().unwrap();
        assert_eq!(read.definition, c.definition);
        assert_eq!(read.public_key, c.public_key);
        assert!(repo.get(Uuid::new_v4(), c.id).await.unwrap().is_none());
        let stored: String = sqlx::query_scalar("SELECT row_to_json(c)::text FROM conversions c WHERE id = $1")
            .bind(c.id).fetch_one(&pool).await.unwrap();
        assert!(!stored.contains(&secret));

        let queued = || sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM outbox_messages WHERE message_type = 'PropagateConversion' AND payload LIKE $1",
        ).bind(format!("%{}%", c.id)).fetch_one(&pool);
        assert_eq!(queued().await.unwrap(), 1);

        let mut changed = read.clone();
        changed.transition(ConversionStatus::Activating).unwrap();
        changed.definition.steps.reverse();
        let saved = repo.save(&changed).await.unwrap();
        assert_eq!(saved.version, 2);
        assert_eq!(repo.save(&read).await.unwrap_err().code, ErrorCode::Conflict);
        assert_eq!(queued().await.unwrap(), 2, "a refused save queues nothing");

        let mut without_cart = saved.clone();
        without_cart.redefine(ConversionDefinition { steps: vec![step("purchase")], ..saved.definition.clone() });
        repo.save(&without_cart).await.unwrap();
        let read = repo.list(workspace).await.unwrap().pop().unwrap();
        assert_eq!(read.definition.steps.len(), 1);
        assert_eq!(read.archived_steps[0].key, "cart");

        let before = queued().await.unwrap();
        PgWorkspaceRepository::new(pool.clone()).delete(workspace).await.unwrap();
        assert!(repo.get(workspace, c.id).await.unwrap().is_none());
        assert_eq!(queued().await.unwrap(), before + 1, "deletion is propagated too");
    }

    #[tokio::test]
    #[ignore = "needs PostgreSQL"]
    async fn deleting_filtered_routes_does_not_cover_everything() {
        let pool = test_pool().await;
        let repo = PgConversionRepository::new(pool.clone());
        let workspace = insert_workspace(&pool).await;
        let (a, b) = (insert_route(&pool, workspace).await, insert_route(&pool, workspace).await);
        let foreign = insert_route(&pool, insert_workspace(&pool).await).await;
        assert_eq!(repo.count_routes(workspace, &[a, b, foreign]).await.unwrap(), 2);

        let (c, _) = Conversion::new(
            workspace,
            ConversionDefinition { route_filter: Some(vec![a]), ..definition() },
            "u".into(),
        );
        repo.create(&c).await.unwrap();
        sqlx::query("DELETE FROM routes WHERE id = $1").bind(a).execute(&pool).await.unwrap();
        let read = repo.get(workspace, c.id).await.unwrap().unwrap();
        assert_eq!(read.definition.route_filter, Some(vec![]));
    }

    #[tokio::test]
    #[ignore = "needs PostgreSQL"]
    async fn lists_only_running_conversions_of_a_domain() {
        let pool = test_pool().await;
        let repo = PgConversionRepository::new(pool.clone());
        let workspace = insert_workspace(&pool).await;
        let domain = Uuid::new_v4();
        sqlx::query("INSERT INTO route_domains (id, name, owner_id) VALUES ($1, $2, 'o')")
            .bind(domain).bind(format!("{domain}.test")).execute(&pool).await.unwrap();
        let d = || ConversionDefinition { domain_id: Some(domain), ..definition() };
        let (draft, _) = Conversion::new(workspace, d(), "u".into());
        let (mut running, _) = Conversion::new(workspace, d(), "u".into());
        running.transition(ConversionStatus::Activating).unwrap();
        repo.create(&draft).await.unwrap();
        repo.create(&running).await.unwrap();

        let listed = repo.list_active_by_domain(domain).await.unwrap();
        assert_eq!(listed.iter().map(|c| c.id).collect::<Vec<_>>(), [running.id]);
    }

    async fn tracked(routes: &PgRouteRepository, id: Uuid) -> bool {
        routes.get_by_id(id).await.unwrap().unwrap().properties.track_conversions
    }

    async fn saved(repo: &PgConversionRepository, mut c: Conversion, status: ConversionStatus) -> Conversion {
        c.status = status;
        repo.save(&c).await.unwrap()
    }

    fn new_route(workspace: Uuid) -> Route {
        Route {
            id: Uuid::new_v4(),
            switch: "main".into(),
            link: Uuid::new_v4().to_string(),
            properties: RouteProperties {
                workspace_id: Some(workspace.to_string().to_uppercase()),
                // Never taken from the caller.
                track_conversions: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[tokio::test]
    #[ignore = "needs PostgreSQL"]
    async fn coverage_follows_conversions_and_routes() {
        let pool = test_pool().await;
        let repo = PgConversionRepository::new(pool.clone());
        let routes = PgRouteRepository::new(pool.clone());
        let (here, other) = (insert_workspace(&pool).await, insert_workspace(&pool).await);

        let uncovered = routes.create(&new_route(here)).await.unwrap();
        assert!(!uncovered.properties.track_conversions, "client value is ignored");

        let (c, _) = Conversion::new(here, definition(), "u".into());
        repo.create(&c).await.unwrap();
        assert!(!tracked(&routes, uncovered.id).await, "drafts cover nothing");
        let c = saved(&repo, c, ConversionStatus::Activating).await;
        assert!(tracked(&routes, uncovered.id).await);

        let mut route = routes.create(&new_route(here)).await.unwrap();
        assert!(route.properties.track_conversions, "new route");
        route.properties.conversion_opt_out = true;
        route = routes.update(&route).await.unwrap();
        assert!(route.properties.conversion_opt_out && !route.properties.track_conversions);
        route.properties.conversion_opt_out = false;
        route = routes.update(&route).await.unwrap();
        assert!(route.properties.track_conversions);
        route.properties.workspace_id = Some(other.to_string());
        route = routes.update(&route).await.unwrap();
        assert!(!route.properties.track_conversions, "moved away");
        route.properties.workspace_id = Some(here.to_string());
        assert!(routes.update(&route).await.unwrap().properties.track_conversions, "moved back");

        // A filter covers only its routes, a second conversion adds its own.
        let mut filtered = c.clone();
        filtered.definition.route_filter = Some(vec![route.id]);
        let c = repo.save(&filtered).await.unwrap();
        assert!(tracked(&routes, route.id).await && !tracked(&routes, uncovered.id).await);
        let (second, _) = Conversion::new(here, definition(), "u".into());
        repo.create(&second).await.unwrap();
        let second = saved(&repo, second, ConversionStatus::Active).await;
        let c = saved(&repo, c, ConversionStatus::Paused).await;
        assert!(tracked(&routes, uncovered.id).await, "still covered by the second");
        saved(&repo, second, ConversionStatus::Paused).await;
        assert!(!tracked(&routes, route.id).await, "last one paused");

        saved(&repo, c, ConversionStatus::Active).await;
        assert!(tracked(&routes, route.id).await);
        PgWorkspaceRepository::new(pool.clone()).delete(here).await.unwrap();
        assert!(!tracked(&routes, route.id).await, "workspace deleted");
    }

    #[tokio::test]
    #[ignore = "needs PostgreSQL"]
    async fn activation_waits_until_the_router_has_every_route() {
        let pool = test_pool().await;
        let repo = PgConversionRepository::new(pool.clone());
        let routes = PgRouteRepository::new(pool.clone());
        let workspace = insert_workspace(&pool).await;
        let ids = [insert_route(&pool, workspace).await, insert_route(&pool, workspace).await];
        let (c, _) = Conversion::new(workspace, definition(), "u".into());
        repo.create(&c).await.unwrap();
        saved(&repo, c.clone(), ConversionStatus::Activating).await;

        let status = || async { repo.get(workspace, c.id).await.unwrap().unwrap().status };
        let pending = || async {
            let all = routes.pending_coverage(Uuid::nil(), 100_000).await.unwrap();
            ids.iter().filter(|id| all.contains(id)).count()
        };
        assert_eq!(pending().await, 2);
        assert_eq!(routes.mark_coverage_sent(ids[0], true).await.unwrap(), Some(true));
        assert_eq!(pending().await, 1, "what was sent is not repeated");
        repo.finish_activations().await.unwrap();
        assert_eq!(status().await, ConversionStatus::Activating);
        routes.mark_coverage_sent(ids[1], true).await.unwrap();
        repo.finish_activations().await.unwrap();
        assert_eq!(status().await, ConversionStatus::Active);
    }

    /// Routes saved while a conversion is activated are never missed.
    #[tokio::test]
    #[ignore = "needs PostgreSQL"]
    async fn concurrent_route_creation_is_covered() {
        let pool = test_pool().await;
        let repo = PgConversionRepository::new(pool.clone());
        let workspace = insert_workspace(&pool).await;
        let (c, _) = Conversion::new(workspace, definition(), "u".into());
        repo.create(&c).await.unwrap();
        let routes = std::sync::Arc::new(PgRouteRepository::new(pool.clone()));
        let creations: Vec<_> = (0..30)
            .map(|_| {
                let routes = routes.clone();
                tokio::spawn(async move { routes.create(&new_route(workspace)).await.unwrap().id })
            })
            .collect();
        saved(&repo, c, ConversionStatus::Activating).await;
        for creation in creations {
            assert!(tracked(&routes, creation.await.unwrap()).await);
        }
    }
}
